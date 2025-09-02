pub use ::chrono;
pub use ::compact_str;
#[cfg(feature = "http")]
pub use ::demurgos_headers::UserAgent;
#[cfg(feature = "reqwest")]
pub use ::reqwest;
#[cfg(feature = "serde")]
pub use ::serde;
pub use ::tower_service;
pub use ::url;
use compact_str::CompactString;
use std::future::Future;

use crate::common::project::Project;
use crate::common::Page;
use crate::query::get_project_release_list::GetProjectReleaseListQuery;
use tower_service::Service;
use crate::common::release::Release;
use crate::query::get_project_release_list_page::GetProjectReleaseListPageQuery;

pub mod client;
pub mod common;
pub mod context;
#[cfg(feature = "http")]
pub mod http;
pub mod query;
pub mod url_util;

pub trait GithubClient<Cx>: Send + Sync {
  type Error<'req>
  where
    Cx: 'req;

  fn get_project_release_list(
    self,
    query: &GetProjectReleaseListQuery<Cx>,
  ) -> impl Send + Future<Output = Result<Page<Release>, Self::Error<'_>>>;

  fn get_project_release_list_page(
    self,
    query: &GetProjectReleaseListPageQuery<Cx>,
  ) -> impl Send + Future<Output = Result<Page<Release>, Self::Error<'_>>>;
}

impl<S, Cx> GithubClient<Cx> for &'_ mut S
where
  Self: Send + Sync,
  Cx: 'static + Send + Sync,
  for<'req> S: Service<&'req GetProjectReleaseListQuery<Cx>, Response = Page<Release>>,
  for<'req> <S as Service<&'req GetProjectReleaseListQuery<Cx>>>::Future: Send,
  for<'req> S: Service<&'req GetProjectReleaseListPageQuery<Cx>, Response = Page<Release>, Error = <S as Service<&'req GetProjectReleaseListQuery<Cx>>>::Error>,
  for<'req> <S as Service<&'req GetProjectReleaseListPageQuery<Cx>>>::Future: Send,
{
  type Error<'req>
    = <S as Service<&'req GetProjectReleaseListQuery<Cx>>>::Error
  where
    Cx: 'req;

  async fn get_project_release_list(self, query: &GetProjectReleaseListQuery<Cx>) -> Result<Page<Release>, Self::Error<'_>> {
    self.call(query).await
  }

  async fn get_project_release_list_page(self, query: &GetProjectReleaseListPageQuery<Cx>) -> Result<Page<Release>, Self::Error<'_>> {
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
