//! Bellhop attends to the needs of your jenkins installation

extern crate hyper;
extern crate rustc_serialize;

use hyper::Client;
use hyper::client::{IntoUrl, RequestBuilder};
use hyper::method::Method;
use hyper::header::{Authorization, UserAgent};
use std::fmt;
use std::io::{Read, Result};
use rustc_serialize::json;

const AGENT: &'static str = "bellhop/0.1.0";

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Job {
  pub name: String,
  pub url: String,
  pub color: String
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Jobs {
  jobs: Vec<Job>
}

pub struct Jenkins<'a> {
  host: &'static str,
  client: &'a Client,
  token: Option<&'static str>
}

impl<'a> Jenkins<'a> {
  pub fn new(host: &'static str, client: &'a Client, token: Option<&'static str>) -> Jenkins<'a> {
    Jenkins {
      host: host,
      client: client,
      token: token
    }
  }

  pub fn jobs(&self) -> Result<Vec<Job>> {
    let body = try!(
      self.get(
        "/api/json"
      )
    );
    let parsed = json::decode::<Jobs>(&body).unwrap();
    Ok(parsed.jobs)
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
