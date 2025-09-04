use crate::context::EmptyContext;
use crate::GithubAuth;

/// Get a page from the project release list
///
/// <https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#list-releases>
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetProjectReleaseListPageQuery<Cx, Str> {
  pub context: Cx,
  pub auth: Option<GithubAuth<Str>>,
  pub cursor: Str,
}

pub type GetProjectReleaseListPageQueryView<'req, Cx> = GetProjectReleaseListPageQuery<&'req Cx, &'req str>;

impl<Cx, Str> GetProjectReleaseListPageQuery<Cx, Str> {
  pub fn set_context<NewCx>(self, new_context: NewCx) -> GetProjectReleaseListPageQuery<NewCx, Str> {
    GetProjectReleaseListPageQuery {
      context: new_context,
      auth: self.auth,
      cursor: self.cursor,
    }
  }

  pub fn as_view(&self) -> GetProjectReleaseListPageQueryView<'_, Cx>
  where
    Str: AsRef<str>,
  {
    GetProjectReleaseListPageQueryView {
      context: &self.context,
      auth: self.auth.as_ref().map(GithubAuth::as_view),
      cursor: self.cursor.as_ref(),
    }
  }
}

impl<Str: AsRef<str>> GetProjectReleaseListPageQuery<EmptyContext, Str> {
  pub const fn new(cursor: Str) -> Self {
    Self {
      context: EmptyContext::new(),
      auth: None,
      cursor,
    }
  }
}
