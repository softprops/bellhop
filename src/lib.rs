//! Bellhop attends to the needs of your jenkins installation

extern crate hyper;
extern crate rustc_serialize;

use std::collections::HashMap;
use hyper::Client;
use hyper::client::{IntoUrl, RequestBuilder};
use hyper::method::Method;
use hyper::header::{Authorization, UserAgent};
use std::fmt;
use std::io::{Read, Result};
use rustc_serialize::json;
use rep::{ Job, Jobs, JobInfo };

pub mod rep;

const AGENT: &'static str = "bellhop/0.1.0";


pub struct JobRef<'a> {
  jenkins: &'a Jenkins<'a>,
  name: &'static str
}

impl<'a> JobRef<'a> {
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

pub struct Jenkins<'a> {
  host: &'static str,
  client: &'a Client,
  token: Option<&'static str>
}

impl<'a> Jenkins<'a> {
  /// Create a new Jenkins client interface
  pub fn new(host: &'static str, client: &'a Client, token: Option<&'static str>) -> Jenkins<'a> {
    Jenkins {
      host: host,
      client: client,
      token: token
    }
  }

  /// Return a references to jobs
  pub fn jobs(&self) -> JobsRef {
    JobsRef { jenkins: self }
  }

  /// Return a reference to a named job
  pub fn job(&self, name: &'static str) -> JobRef {
    JobRef {
      jenkins: self,
      name: name
    }
  }

  fn post(&self, uri: &str, message: &[u8]) -> Result<String> {
    let url = format!("{}{}", self.host, uri);
    self.request(
      self.client.get(
        &url
      ), Some(message)
    )
  }

  fn get(&self, uri: &str) -> Result<String> {
    let url = format!("{}{}", self.host, uri);
    self.request(
      self.client.get(
        &url
      ), None
    )
  }

  fn request<U: IntoUrl>(
    &self, request_builder: RequestBuilder<'a, U>, body: Option<&'a [u8]>) -> Result<String> {
    let builder = request_builder.header(
      UserAgent(AGENT.to_owned())
    );
    let authenticated = match self.token {
      Some(token) =>
        builder.header(
          Authorization(format!("token {}", token))
        ),
      _ =>
        builder
    };
    let mut res = match body {
      Some(ref bod) => authenticated.body(*bod).send().unwrap(),
       _ => authenticated.send().unwrap()
    };
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    Ok(body)
  }
}
