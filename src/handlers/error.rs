// use crate::errors::ApiError;
use actix_web::{
  dev,
  http::{header, HeaderValue},
  middleware::errhandlers::ErrorHandlerResponse,
  Result,
};

pub fn bad_request<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
  let new_res = res.map_body(|resphead, _respbody| {
    resphead.headers_mut().insert(
      header::CONTENT_TYPE,
      HeaderValue::from_static("application/json"),
    );
    dev::ResponseBody::Other(dev::Body::Message(Box::new(r#"{"errors":["bad_request"]"#)))
  });
  Ok(ErrorHandlerResponse::Response(new_res))
}

pub fn not_found<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
  let new_res = res.map_body(|resphead, _respbody| {
    resphead.headers_mut().insert(
      header::CONTENT_TYPE,
      HeaderValue::from_static("application/json"),
    );
    dev::ResponseBody::Other(dev::Body::Message(Box::new(r#"{"errors":["not_found"]}"#)))
  });
  Ok(ErrorHandlerResponse::Response(new_res))
}

pub fn internal_server_error<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
  let new_res = res.map_body(|resphead, _respbody| {
    resphead.headers_mut().insert(
      header::CONTENT_TYPE,
      HeaderValue::from_static("application/json"),
    );
    dev::ResponseBody::Other(dev::Body::Message(Box::new(
      r#"{"errors":["internal_server_error"]}"#,
    )))
  });
  Ok(ErrorHandlerResponse::Response(new_res))
}
