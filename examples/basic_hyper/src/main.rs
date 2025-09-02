use bytes::Bytes;
use http_body_util::Full;
use hyper_tls::HttpsConnector;
use hyper_util::client::legacy::Client;
use katal_github_client::client::http::HttpGithubClient;
use katal_github_client::common::project::{ProjectSlug, RepositoryRef};
use katal_github_client::common::release::Release;
use katal_github_client::common::Page;
use katal_github_client::compact_str::CompactString;
use katal_github_client::context::{Context, GithubUrl};
use katal_github_client::query::get_project_release_list::GetProjectReleaseListQuery;
use katal_github_client::query::get_project_release_list_page::GetProjectReleaseListPageQuery;
use katal_github_client::url::Url;
use katal_github_client::{GithubAuth, GithubClient, UserAgent};

#[tokio::main]
async fn main() {
  let authentication: Option<GithubAuth> = None;
  // if let Some(token) = std::env::var("GITHUB_PRIVATE_TOKEN").ok() {
  //   Some(GithubAuth::PrivateToken(token.parse().unwrap()))
  // } else {
  //   None
  // };

  let connector = HttpsConnector::new();
  let client: Client<HttpsConnector<_>, Full<Bytes>> =
    Client::builder(hyper_util::rt::TokioExecutor::new()).build(connector);
  let mut client = HttpGithubClient::new(client);
  let context = Context::new()
    .set_github_url(GithubUrl(Url::parse("https://api.github.com/").unwrap()))
    .set_user_agent(UserAgent::from_static("katal_github_client_example/0.0.0"));
  // let mut query = GetProjectListQuery::<_>::new().set_context(context);
  // // query.auth = Some(GithubAuth::PrivateToken("...".parse().unwrap()));
  // // query.owned = Some(true);
  // let res = client.get_project_list(&query).await.unwrap();
  // for p in &res.items {
  //   dbg!(&p.path_with_namespace);
  // }
  // if let Some(next) = res.next {
  //   let context = Context::new().set_github_url(GithubUrl(Url::parse("https://github.com/").unwrap()));
  //   let query = GetProjectListPageQuery::<_>::new(next).set_context(context);
  //   let res = client.call(&query).await.unwrap();
  //   for s in &res.items {
  //     dbg!(&s.path_with_namespace);
  //   }
  // }
  {
    let mut query = GetProjectReleaseListQuery::<_>::new(RepositoryRef::Slug(ProjectSlug::new(
      CompactString::new("unicode-org"),
      CompactString::new("icu"),
    )))
    .set_context(context.clone());
    query.auth = authentication.clone();
    let mut res: Page<Release> = client.get_project_release_list(&query).await.unwrap();
    eprintln!("successfully fetched first page. count={:?}", res.items.len());
    // eprintln!("successfully fetched first page. last={:?}", res.last);
    while let Some(next) = res.next {
      let mut query = GetProjectReleaseListPageQuery::<_>::new(next).set_context(context.clone());
      query.auth = authentication.clone();
      res = client.get_project_release_list_page(&query).await.unwrap();
      eprintln!("successfully got page. count={:?}", res.items.len());
    }
  }
}
