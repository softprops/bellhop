//! Rust representations of Jenkins data structures

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Parameter {
  pub name: String
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Build {
  pub number: i64,
  pub url: String
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct JobInfo {
  pub displayName: String,
  pub name: String,
  pub url: String,
  pub buildable: bool,
  pub builds: Vec<Build>,
  pub color: String,
  pub firstBuild: Option<Build>,
  pub lastBuild: Option<Build>,
  pub lastCompletedBuild: Option<Build>,
  pub lastFailedBuild: Option<Build>,
  pub lastStableBuild: Option<Build>,
  pub lastSuccsesfulBuild: Option<Build>,
  pub lastUnstableBuild: Option<Build>,
  pub lastUnsuccessfulBuild: Option<Build>,
  pub nextBuildNumber: i64
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Job {
  pub name: String,
  pub url: String,
  pub color: String
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Jobs {
  pub jobs: Vec<Job>
}
