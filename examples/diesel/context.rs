use std::collections::{HashMap};
use std::str;
use smallvec::SmallVec;

use thruster::{Context, Response, Request};

pub struct Ctx {
  pub body: String,
  pub method: String,
  pub path: String,
  pub request_body: String,
  pub params: HashMap<String, String>,
  pub headers: SmallVec<[(String, String); 8]>,
  pub status_code: u32,
  pub response: Response
}

impl Ctx {
  pub fn set_header(&mut self, key: &str, val: &str) {
    self.response.header(key, val);
  }
}

impl Context for Ctx {
  type Response = Response;

  fn get_response(mut self) -> Response {
    self.response.status_code(200, "OK");

    self.response.body(&self.body);

    self.response
  }

  fn set_body(&mut self, body: Vec<u8>) {
    self.body = str::from_utf8(&body).unwrap_or("").to_owned();
  }
}

pub fn generate_context(request: Request) -> Ctx {
  let method = request.method().to_owned();
  let path = request.path().to_owned();
  let request_body = request.body().to_owned();

  Ctx {
    body: "".to_owned(),
    method: method,
    path: path,
    params: request.params,
    request_body: request_body,
    headers: SmallVec::new(),
    status_code: 200,
    response: Response::new()
  }
}
