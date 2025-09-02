use crate::common::project::RepositoryRef;
use crate::common::Pagination;
use crate::context::EmptyContext;
use crate::GithubAuth;
use compact_str::CompactString;

/// List project releases
///
/// <https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#list-releases>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetProjectReleaseListQuery<Cx, Str = CompactString> {
  pub context: Cx,
  pub auth: Option<GithubAuth<Str>>,
  pub pagination: Option<Pagination>,
  pub repository: RepositoryRef<Str>,
}

pub type GetProjectReleaseListQueryView<'req, Cx> = GetProjectReleaseListQuery<&'req Cx, &'req str>;

impl<Cx, Str> GetProjectReleaseListQuery<Cx, Str> {
  pub fn set_context<NewCx>(self, new_context: NewCx) -> GetProjectReleaseListQuery<NewCx, Str> {
    GetProjectReleaseListQuery {
      context: new_context,
      auth: self.auth,
      pagination: self.pagination,
      repository: self.repository,
    }
  }

  pub fn as_view(&self) -> GetProjectReleaseListQueryView<'_, Cx>
  where
    Str: AsRef<str>,
  {
    GetProjectReleaseListQueryView {
      context: &self.context,
      auth: self.auth.as_ref().map(GithubAuth::as_view),
      pagination: self.pagination,
      repository: self.repository.as_view(),
    }
  }
}

impl<Str: AsRef<str>> GetProjectReleaseListQuery<EmptyContext, Str> {
  pub const fn new(repository: RepositoryRef<Str>) -> Self {
    Self {
      context: EmptyContext::new(),
      auth: None,
      pagination: None,
      repository,
    }
  }
}
