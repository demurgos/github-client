use chrono::{DateTime, Utc};
use compact_str::CompactString;
use url::Url;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Release {
  pub url: Url,
  pub html_url: Url,
  pub assets_url: Url,
  pub upload_url: Url,
  pub tarball_url: Option<Url>,
  pub zipball_url: Option<Url>,
  pub id: u32,
  pub node_id: String,
  pub tag_name: String,
  pub target_commitish: String,
  pub name: Option<String>,
  pub body: Option<String>,
  pub draft: bool,
  pub prerelease: bool,
  pub created_at: DateTime<Utc>,
  pub published_at: Option<DateTime<Utc>>,
  // pub author: ...,
  pub assets: Vec<ReleaseAsset>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseAsset {
  pub url: Url,
  pub browser_download_url: Url,
  pub id: u32,
  pub node_id: String,
  pub name: String,
  pub label: Option<String>,
  pub state: String, // uploaded/open
  pub content_type: String,
  pub size: u32,
  pub download_count: u32,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  // pub uploader: ...,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuthorId(u64);

impl AuthorId {
  pub const fn new(id: u64) -> Self {
    Self(id)
  }

  pub const fn into_u64(self) -> u64 {
    self.0
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Author {
  pub id: AuthorId,
  pub name: String,
  pub username: String,
  pub state: String,
  pub avatar_url: String,
  pub web_url: String,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Commit {
  pub id: String,
  pub short_id: String,
  pub title: String,
  pub created_at: DateTime<Utc>,
  pub parent_ids: Vec<String>,
  pub message: String,
  pub author_name: String,
  pub author_email: String,
  pub authored_date: DateTime<Utc>,
  pub committer_name: String,
  pub committer_email: String,
  pub committed_date: DateTime<Utc>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Milestone {
  // TODO
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseAssets {
  pub count: u64,
  pub sources: Vec<ReleaseSource>,
  pub links: Vec<ReleaseLink>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseSource {
  pub format: String,
  pub url: String,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseEvidence {
  // TODO
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseLinks {
  closed_issues_url: String,
  closed_merge_requests_url: String,
  edit_url: Option<String>,
  merged_merge_requests_url: String,
  opened_issues_url: String,
  opened_merge_requests_url: String,
  #[cfg_attr(feature = "serde", serde(rename = "self"))]
  this: String,
}

/// Criteria used to order releases
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReleaseOrder {
  ReleasedAt,
  CreatedAt,
}

impl ReleaseOrder {
  pub fn as_str(self) -> &'static str {
    match self {
      Self::ReleasedAt => "released_at",
      Self::CreatedAt => "created_at",
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseLinkId(u64);

impl ReleaseLinkId {
  pub const fn new(id: u64) -> Self {
    Self(id)
  }

  pub const fn into_u64(self) -> u64 {
    self.0
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReleaseLink {
  pub id: ReleaseLinkId,
  pub name: String,
  pub url: String,
  pub direct_asset_url: String,
  pub link_type: ReleaseLinkType,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReleaseLinkType {
  #[cfg_attr(feature = "serde", serde(rename = "other"))]
  Other,
  #[cfg_attr(feature = "serde", serde(rename = "runbook"))]
  Runbook,
  #[cfg_attr(feature = "serde", serde(rename = "image"))]
  Image,
  #[cfg_attr(feature = "serde", serde(rename = "package"))]
  Package,
}

impl ReleaseLinkType {
  pub fn as_str(self) -> &'static str {
    match self {
      Self::Other => "other",
      Self::Runbook => "runbook",
      Self::Image => "image",
      Self::Package => "package",
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputReleaseAssets<Links = Vec<InputReleaseLink<CompactString>>> {
  pub links: Links,
}

pub type InputReleaseAssetsView<'req, Str = CompactString> = InputReleaseAssets<&'req [InputReleaseLink<Str>]>;

impl<Links> InputReleaseAssets<Links> {
  pub fn as_view<Str>(&self) -> InputReleaseAssetsView<'_, Str>
  where
    Links: AsRef<[InputReleaseLink<Str>]>,
  {
    InputReleaseAssetsView {
      links: self.links.as_ref(),
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputReleaseLink<Str = CompactString> {
  pub name: Str,
  pub url: Str,
  pub direct_asset_path: Option<Str>,
  pub link_type: ReleaseLinkType,
}

pub type InputReleaseLinkView<'req> = InputReleaseLink<&'req str>;

impl<Str: AsRef<str>> InputReleaseLink<Str> {
  pub fn as_view(&self) -> InputReleaseLinkView<'_> {
    InputReleaseLinkView {
      name: self.name.as_ref(),
      url: self.url.as_ref(),
      direct_asset_path: self.direct_asset_path.as_ref().map(|s| s.as_ref()),
      link_type: self.link_type,
    }
  }
}
