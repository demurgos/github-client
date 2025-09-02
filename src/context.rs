use std::ops::Deref;
use url::Url;

/// A very restricted version of frunk hlist to hold the context for github client requests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context<TyGithubUrl, TyUserAgent> {
  github_url: TyGithubUrl,
  user_agent: TyUserAgent,
}

impl<TyGithubUrl, TyUserAgent> Context<TyGithubUrl, TyUserAgent> {
  pub fn set_github_url<NewGithubUrl>(self, github_url: NewGithubUrl) -> Context<NewGithubUrl, TyUserAgent> {
    Context {
      github_url,
      user_agent: self.user_agent,
    }
  }

  pub fn set_user_agent<NewUserAgent>(self, user_agent: NewUserAgent) -> Context<TyGithubUrl, NewUserAgent> {
    Context {
      github_url: self.github_url,
      user_agent,
    }
  }
}

pub type EmptyContext = Context<(), ()>;

impl EmptyContext {
  pub const fn new() -> Self {
    Self {
      github_url: (),
      user_agent: (),
    }
  }
}

impl Default for EmptyContext {
  fn default() -> Self {
    Self::new()
  }
}

pub trait GetRef<T: ?Sized> {
  fn get_ref(&self) -> &T;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GithubUrl(pub Url);

impl Deref for GithubUrl {
  type Target = Url;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<TyUserAgent> GetRef<GithubUrl> for Context<GithubUrl, TyUserAgent> {
  fn get_ref(&self) -> &GithubUrl {
    &self.github_url
  }
}

#[cfg(feature = "http")]
impl<TyGithubUrl> GetRef<demurgos_headers::UserAgent> for Context<TyGithubUrl, demurgos_headers::UserAgent> {
  fn get_ref(&self) -> &demurgos_headers::UserAgent {
    &self.user_agent
  }
}
