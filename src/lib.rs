//! Bellhop attends to the needs of your jenkins installation

extern crate hyper;
extern crate rustc_serialize;

use hyper::Client;
use hyper::client::{IntoUrl, RequestBuilder};
use hyper::method::Method;
use hyper::header::{Authorization, UserAgent};
use std::io::{Read, Result};

pub mod jobs;
pub mod rep;

use jobs::{JobRef, JobsRef};

const AGENT: &'static str = "bellhop/0.1.0";

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
    JobsRef::new(self)
  }

  /// Return a reference to a named job
  pub fn job(&self, name: &'static str) -> JobRef {
    JobRef::new(self, name)
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
