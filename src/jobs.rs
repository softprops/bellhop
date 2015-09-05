use super::Jenkins;
use rustc_serialize::json;
use std::collections::HashMap;
use rep::{ BuildInfo, Job, Jobs, JobInfo };
use std::io::{Read, Result};

pub struct JobRef<'a> {
  jenkins: &'a Jenkins<'a>,
  name: &'static str
}

impl<'a> JobRef<'a> {
  pub fn new(jenkins: &'a Jenkins<'a>, name: &'static str) -> JobRef<'a> {
    JobRef { jenkins: jenkins, name: name }
  }

  pub fn info(&self) -> Result<JobInfo> {
    let body = try!(
      self.jenkins.get(
        &format!("/job/{}/api/json", self.name)
      )
    );
    Ok(json::decode::<JobInfo>(&body).unwrap())
  }

  pub fn stop(&self, build: i64) -> Result<()> {
    let path = format!("/job/{}/{}/stop", self.name, build);
    self.jenkins.post(
      &path,
      &vec![]
    ).map(|_| ())
  }

  pub fn disable(&self) -> Result<String> {
    self.act("disable")
  }

  pub fn enable(&self) -> Result<String> {
    self.act("enable")
  }

  fn act(&self, action: &'static str) -> Result<String> {
    self.jenkins.post(
      &format!("/job/{}/{}", self.name, action),
      &vec![]
    )
  }

  pub fn last(&self) -> Result<BuildInfo> {
    let body = try!(
      self.jenkins.get(
        &format!(
          "/job/{}/lastBuild/api/json", self.name
        )
      )
    );
    Ok(json::decode::<BuildInfo>(&body).unwrap())
  }

  pub fn last_completed(&self) -> Result<BuildInfo> {
    let body = try!(
      self.jenkins.get(
        &format!(
          "/job/{}/lastCompletedBuild/api/json", self.name
        )
      )
    );
    Ok(json::decode::<BuildInfo>(&body).unwrap())
  }

  pub fn build(&self, params: Option<HashMap<&'static str, &'static str>>) -> Result<()> {
    let uri = match params {
      Some(args) => {
        let path = format!(
          "/job/{}/buildWithParameters", self.name
        );
        let mut query = vec![];
        for (k, v) in args {
          query.push(
            vec![k, v].connect("=")
          )
        }
        vec![
          path,
          query.connect("&")
        ].connect("?")
      },
      _ => {
        format!(
          "/job/{}/build/api/json", self.name
        )
      }
    };
    self.jenkins.post(&uri, &vec![]).map(|_| ())
  }
}

pub struct JobsRef<'a> {
  jenkins: &'a Jenkins<'a>
}

impl<'a> JobsRef<'a> {
  pub fn new(jenkins: &'a Jenkins<'a>) -> JobsRef<'a> {
    JobsRef { jenkins: jenkins }
  }

  pub fn list(&self) -> Result<Vec<Job>> {
    let body = try!(
      self.jenkins.get(
        "/api/json"
      )
    );
    let parsed = json::decode::<Jobs>(&body).unwrap();
    Ok(parsed.jobs)
  }
}
