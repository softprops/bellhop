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
  #[allow(non_snake_case)]
  pub displayName: String,
  pub name: String,
  pub url: String,
  pub buildable: bool,
  pub builds: Vec<Build>,
  pub color: String,
  #[allow(non_snake_case)]
  pub firstBuild: Option<Build>,
  #[allow(non_snake_case)]
  pub lastBuild: Option<Build>,
  #[allow(non_snake_case)]
  pub lastCompletedBuild: Option<Build>,
  #[allow(non_snake_case)]
  pub lastFailedBuild: Option<Build>,
  #[allow(non_snake_case)]
  pub lastStableBuild: Option<Build>,
  #[allow(non_snake_case)]
  pub lastSuccsesfulBuild: Option<Build>,
  #[allow(non_snake_case)]
  pub lastUnstableBuild: Option<Build>,
  #[allow(non_snake_case)]
  pub lastUnsuccessfulBuild: Option<Build>,
  #[allow(non_snake_case)]
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

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct BuildInfo {
  pub building: bool,
  pub description: String,
  pub displayName: String,
  pub duration: i64,
  pub estimatedDuration: Option<i64>,
  pub fullDisplayName: String,
  pub id: String,
  pub keepLog: bool,
  pub number: i64,
  pub queueId: i64,
  pub result: String,
  pub timestamp: i64,
  pub url: String,
  pub builtOn: Option<String>
}
