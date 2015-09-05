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

  pub fn info(&self, job: &'static str) -> Result<JobInfo> {
    let body = try!(
      self.get(
        &format!("/job/{}/api/json", job)
      )
    );
    Ok(json::decode::<JobInfo>(&body).unwrap())
  }

  pub fn stop(&self, job: &'static str, build: i64) -> Result<()> {
    let path = format!("/job/{}/{}/stop", job, build);
    self.post(
      &path,
      &vec![]
    ).map(|_| ())
  }

  pub fn build(&self, job: &'static str, params: Option<HashMap<&'static str, &'static str>>) -> Result<()> {
    let uri = match params {
      Some(args) => {
        let path = format!(
          "/job/{}/buildWithParameters", job
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
          "/job/{}/build/api/json", job
        )
      }
    };
    self.post(&uri, &vec![]).map(|_| ())
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
