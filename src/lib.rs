pub use ::chrono;
pub use ::compact_str;
use compact_str::CompactString;
#[cfg(feature = "http")]
pub use ::demurgos_headers::UserAgent;
#[cfg(feature = "reqwest")]
pub use ::reqwest;
#[cfg(feature = "serde")]
pub use ::serde;
use std::future::Future;
pub use ::tower_service;
pub use ::url;

use crate::common::release::Release;
use crate::common::Page;
use crate::query::get_project_release_list::GetProjectReleaseListQuery;
use crate::query::get_project_release_list_page::GetProjectReleaseListPageQuery;
use tower_service::Service;

pub mod client;
pub mod common;
pub mod context;
#[cfg(feature = "http")]
pub mod http;
pub mod query;
pub mod url_util;

pub trait GithubClient<Cx, Str>: Send + Sync {
  type GetProjectReleaseListError<'req>
  where
    Cx: 'req,
    Str: 'req;

  fn get_project_release_list(
    self,
    query: &GetProjectReleaseListQuery<Cx, Str>,
  ) -> impl Send + Future<Output = Result<Page<Release>, Self::GetProjectReleaseListError<'_>>>;

  type GetProjectReleaseListPageError<'req>
  where
    Cx: 'req,
    Str: 'req;

  fn get_project_release_list_page(
    self,
    query: &GetProjectReleaseListPageQuery<Cx, Str>,
  ) -> impl Send + Future<Output = Result<Page<Release>, Self::GetProjectReleaseListPageError<'_>>>;
}

impl<S, Cx, Str> GithubClient<Cx, Str> for &'_ mut S
where
  Self: Send + Sync,
  Cx: Send + Sync,
  Str: Send + Sync,
  for<'req> S: Service<&'req GetProjectReleaseListQuery<Cx, Str>, Response = Page<Release>, Future: Send>,
  for<'req> S: Service<&'req GetProjectReleaseListPageQuery<Cx, Str>, Response = Page<Release>, Future: Send>,
{
  type GetProjectReleaseListError<'req>
    = <S as Service<&'req GetProjectReleaseListQuery<Cx, Str>>>::Error
  where
    Cx: 'req,
    Str: 'req;

  async fn get_project_release_list(
    self,
    query: &GetProjectReleaseListQuery<Cx, Str>,
  ) -> Result<Page<Release>, Self::GetProjectReleaseListError<'_>> {
    self.call(query).await
  }

  type GetProjectReleaseListPageError<'req>
    = <S as Service<&'req GetProjectReleaseListPageQuery<Cx, Str>>>::Error
  where
    Cx: 'req,
    Str: 'req;

  async fn get_project_release_list_page(
    self,
    query: &GetProjectReleaseListPageQuery<Cx, Str>,
  ) -> Result<Page<Release>, Self::GetProjectReleaseListPageError<'_>> {
    self.call(query).await
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputPackageStatus {
  Default,
  Hidden,
}

impl InputPackageStatus {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Default => "default",
      Self::Hidden => "hidden",
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PackageStatus {
  Default,
  Hidden,
  Processing,
  Error,
  PendingDestruction,
}

impl PackageStatus {
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Default => "default",
      Self::Hidden => "hidden",
      Self::Processing => "processing",
      Self::Error => "error",
      Self::PendingDestruction => "pending_destruction",
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GithubAuth<Token = CompactString> {
  PrivateToken(Token),
  JobToken(Token),
}

pub type GithubAuthView<'s> = GithubAuth<&'s str>;

impl<Token: AsRef<str>> GithubAuth<Token> {
  pub fn as_view(&self) -> GithubAuthView<'_> {
    match self {
      Self::PrivateToken(token) => GithubAuth::PrivateToken(token.as_ref()),
      Self::JobToken(token) => GithubAuth::JobToken(token.as_ref()),
    }
  }

  pub fn http_header(&self) -> (&'static str, &str) {
    match self {
      Self::PrivateToken(token) => ("PRIVATE-TOKEN", token.as_ref()),
      Self::JobToken(token) => ("JOB-TOKEN", token.as_ref()),
    }
  }
}
