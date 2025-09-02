use crate::common::package::GenericPackageFile;
use crate::common::release::{InputReleaseAssetsView, Release, ReleaseLink};
use crate::common::tree::TreeRecord;
use crate::common::Page;
use crate::context::{GetRef, GithubUrl};
use crate::query::get_project_release_list::GetProjectReleaseListQuery;
use crate::query::get_project_release_list_page::GetProjectReleaseListPageQuery;
use crate::url_util::UrlExt;
use crate::{GithubAuth, GithubAuthView, InputPackageStatus, Project};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use compact_str::CompactString;
use core::task::{Context, Poll};
use demurgos_headers::link::{Link, RelationType};
use demurgos_headers::UserAgent;
use futures::future::BoxFuture;
use http::header::CONTENT_TYPE;
use http::{HeaderMap, Method, Request, Response, StatusCode};
use http_body::Body;
use http_body_util::{BodyExt, Full};
use std::error::Error as StdError;
use tower_service::Service;
use url::Url;
use crate::common::project::RepositoryRef;

pub struct HttpGithubClient<TyInner> {
  inner: TyInner,
}

impl<TyInner> HttpGithubClient<TyInner> {
  pub fn new(inner: TyInner) -> Self {
    Self { inner }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
pub enum HttpGithubClientError {
  #[error("failed to poll ready status: {0}")]
  PollReady(String),
  #[error("failed to send request: {0}")]
  Send(String),
  #[error("failed to receive response: {0}")]
  Receive(String),
  #[error("failed to parse response: {0}")]
  ResponseFormat(String, Bytes),
  #[error("operation is forbidden for provided auth")]
  Forbidden,
  #[error("resource already exists")]
  Conflict,
  #[error("resource not found")]
  NotFound,
  #[error("unexpected error: {0}")]
  Other(String),
}

impl<'req, Cx, TyInner, TyBody> Service<&'req GetProjectReleaseListQuery<Cx>> for HttpGithubClient<TyInner>
where
  Cx: GetRef<GithubUrl> + GetRef<UserAgent>,
  TyInner: Service<Request<Full<Bytes>>, Response = Response<TyBody>> + 'req,
  TyInner::Error: StdError,
  TyInner::Future: Send,
  TyBody: Body + Send,
  TyBody::Data: Send,
  TyBody::Error: StdError,
{
  type Response = Page<Release>;
  type Error = HttpGithubClientError;
  type Future = BoxFuture<'req, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self
      .inner
      .poll_ready(cx)
      .map_err(|e| HttpGithubClientError::PollReady(format!("{e:?}")))
  }

  fn call(&mut self, req: &'req GetProjectReleaseListQuery<Cx>) -> Self::Future {
    let mut url: Url = {
      let base = GetRef::<GithubUrl>::get_ref(&req.context);
      match req.repository.as_view() {
        RepositoryRef::Id(repo_id) => {
          repo_id.with_str(|repo_id| base.url_join(["repositories", repo_id, "releases"]))
        },
        RepositoryRef::Slug(slug) => {
          base.url_join(["repos", slug.owner, slug.name, "releases"])
        },
      }
    };

    {
      let mut query = url.query_pairs_mut();
      if let Some(pagination) = req.pagination {
        if let Some(per_page) = pagination.per_page {
          query.append_pair("per_page", per_page.to_string().as_str());
        }
      }
    }

    let req = Request::builder()
      .method(Method::GET)
      .uri(dbg!(url.as_str()))
      .header(CONTENT_TYPE, "application/vnd.github+json")
      .user_agent(GetRef::<UserAgent>::get_ref(&req.context))
      .github_auth(req.auth.as_ref().map(GithubAuth::as_view))
      .body(Full::new(Bytes::new()))
      .unwrap();
    let res = self.inner.call(req);
    Box::pin(async move {
      let res: Response<TyBody> = res.await.map_err(|e| HttpGithubClientError::Send(format!("{e:?}")))?;
      let cursors = get_cursors(res.headers());
      let body = res
        .into_body()
        .collect()
        .await
        .map_err(|e| HttpGithubClientError::Receive(format!("{e:?}")))?;
      let body: Bytes = body.to_bytes();

      let body: Vec<Release> =
        serde_json::from_slice(&body).map_err(|e| HttpGithubClientError::ResponseFormat(format!("{e:?}"), body))?;
      Ok(Page {
        first: cursors.first,
        next: cursors.next,
        last: cursors.last,
        items: body,
      })
    })
  }
}

impl<'req, Cx, Str, TyInner, TyBody> Service<&'req GetProjectReleaseListPageQuery<Cx, Str>>
  for HttpGithubClient<TyInner>
where
  Cx: GetRef<UserAgent>,
  Str: AsRef<str>,
  TyInner: Service<Request<Full<Bytes>>, Response = Response<TyBody>> + 'req,
  TyInner::Error: StdError,
  TyInner::Future: Send,
  TyBody: Body + Send,
  TyBody::Data: Send,
  TyBody::Error: StdError,
{
  type Response = Page<Release>;
  type Error = HttpGithubClientError;
  type Future = BoxFuture<'req, Result<Self::Response, Self::Error>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self
      .inner
      .poll_ready(cx)
      .map_err(|e| HttpGithubClientError::PollReady(format!("{e:?}")))
  }

  fn call(&mut self, req: &'req GetProjectReleaseListPageQuery<Cx, Str>) -> Self::Future {
    let url: &str = req.cursor.as_ref();

    let req = Request::builder()
      .method(Method::GET)
      .uri(url)
      .user_agent(GetRef::<UserAgent>::get_ref(&req.context))
      .github_auth(req.auth.as_ref().map(GithubAuth::as_view))
      .body(Full::new(Bytes::new()))
      .unwrap();

    let res = self.inner.call(req);
    Box::pin(async move {
      let res: Response<TyBody> = res.await.map_err(|e| HttpGithubClientError::Send(format!("{e:?}")))?;
      let cursors = get_cursors(res.headers());
      let body = res
        .into_body()
        .collect()
        .await
        .map_err(|e| HttpGithubClientError::Receive(format!("{e:?}")))?;
      let body: Bytes = body.to_bytes();

      let body: Vec<Release> =
        serde_json::from_slice(&body).map_err(|e| HttpGithubClientError::ResponseFormat(format!("{e:?}"), body))?;
      Ok(Page {
        first: cursors.first,
        next: cursors.next,
        last: cursors.last,
        items: body,
      })
    })
  }
}

struct Cursors<Str> {
  first: Option<Str>,
  next: Option<Str>,
  last: Option<Str>,
}

fn get_cursors(headers: &HeaderMap) -> Cursors<CompactString> {
  use demurgos_headers::HeaderMapExt;

  let mut next: Option<CompactString> = None;
  let mut first: Option<CompactString> = None;
  let mut last: Option<CompactString> = None;

  let links: Option<Link> = headers.typed_get::<Link>();

  if let Some(links) = links {
    for value in links.values() {
      let rel = match value.rel() {
        Some(rel) => rel,
        None => continue,
      };
      for r in rel {
        // todo: detect when there are multiple different links for the same rel type
        if *r == RelationType::NEXT {
          next = Some(CompactString::new(value.link()));
        }
        if *r == RelationType::FIRST {
          first = Some(CompactString::new(value.link()));
        }
        if *r == RelationType::LAST {
          last = Some(CompactString::new(value.link()));
        }
      }
    }
  }

  Cursors { first, next, last }
}

trait RequestBuilderExt {
  fn user_agent(self, user_agent: &UserAgent) -> Self;

  fn github_auth(self, github_auth: Option<GithubAuthView<'_>>) -> Self;
}

impl RequestBuilderExt for http::request::Builder {
  fn user_agent(mut self, user_agent: &UserAgent) -> Self {
    use demurgos_headers::HeaderMapExt;

    if let Some(headers) = self.headers_mut() {
      HeaderMapExt::typed_insert(headers, user_agent.clone());
    }
    self
  }

  fn github_auth(self, github_auth: Option<GithubAuthView<'_>>) -> Self {
    if let Some(auth) = github_auth {
      let (key, value) = auth.http_header();
      self.header(key, value)
    } else {
      self
    }
  }
}

trait BoolExt {
  fn as_str(&self) -> &'static str;
}

impl BoolExt for bool {
  fn as_str(&self) -> &'static str {
    if *self {
      "true"
    } else {
      "false"
    }
  }
}
