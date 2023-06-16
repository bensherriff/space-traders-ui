use core::time;
use std::thread;
use log::warn;
use reqwest::{Client, RequestBuilder};
use serde::{de, Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseObject<T> {
  pub data: Option<T>,
  pub error: Option<ErrorObject>,
  pub meta: Option<MetaObject>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorObject {
  pub code: u64,
  pub message: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MetaObject {
  pub total: u64,
  pub page: u64,
  pub limit: u64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ResponseObjectEvent<T> {
  Object(T),
  GetObject(ResponseObject<T>),
  ErrorObject(ErrorObject),
  None
}

pub struct Request {
  pub client: Client,
  pub base_url: String,
  pub max_attemps: u64
}

impl Request {
  pub async fn get_request<T: de::DeserializeOwned>(&self, token: String, url: String, query: Option<Vec<(&str, String)>>) -> ResponseObject<T> {
    let attempt: u64 = 0;

    while attempt < self.max_attemps {
      let mut uri: String = format!("{}{}", self.base_url, url);
      let _token = token.to_owned();
      let _query = query.to_owned();

      match _query {
        Some(q) => {
          uri = format!("{}?", uri);
          for (index, item) in q.iter().enumerate() {
            if index < q.len() - 1 {
              uri = format!("{}{}={}&", uri, item.0, item.1)
            } else {
              uri = format!("{}{}={}", uri, item.0, item.1)
            }
          }
        },
        None => {}
      }
      let result = self.send(self.client.get(uri).bearer_auth(_token)).await;
      match &result.error {
        Some(error) => {
          if error.code == 429 {
            thread::sleep(time::Duration::from_millis(attempt * 500));
            continue;
          } else if error.code == 502 {
            thread::sleep(time::Duration::from_millis(10000));
            continue;
          }
        }
        None => return result
      }
    }
    ResponseObject { data: None, error: Some(ErrorObject { code: 9999, message: "".to_string() }), meta: None }
  }
  
  pub async fn post_request<T: de::DeserializeOwned>(&self, token: String, url: String, body: Option<String>) -> ResponseObject<T> {
    let attempt: u64 = 0;
    
    while attempt < self.max_attemps {
      let uri: String = format!("{}{}", self.base_url, url);
      let _token = token.to_owned();
      let _body = body.to_owned();
      let result = match _body {
        None => {
          self.send(self.client.post(uri).bearer_auth(_token).header("Content-Length", "0"))
        }
        Some(b) => {
          self.send(self.client.post(uri).bearer_auth(_token).header("Content-Type", "application/json".to_string()).body(b))
        }
      }.await;
      match &result.error {
        Some(error) => {
          if error.code == 429 {
            thread::sleep(time::Duration::from_millis(attempt * 500));
            continue;
          } else if error.code == 502 {
            thread::sleep(time::Duration::from_millis(10000));
            continue;
          }
        }
        None => return result
      }
    }
    ResponseObject { data: None, error: Some(ErrorObject { code: 9999, message: "".to_string() }), meta: None }
  }
  
  pub async fn patch_request<T: de::DeserializeOwned>(&self, token: String, url: String, body: Option<String>) -> ResponseObject<T> {
    let attempt: u64 = 0;
    
    while attempt < self.max_attemps {
      let uri: String = format!("{}{}", self.base_url, url);
      let _token = token.to_owned();
      let _body = body.to_owned();
      let result = match _body {
        None => {
          self.send(self.client.patch(uri).bearer_auth(_token).header("Content-Length", "0"))
        }
        Some(b) => {
          self.send(self.client.patch(uri).bearer_auth(_token).header("Content-Type", "application/json".to_string()).body(b))
        }
      }.await;
      match &result.error {
        Some(error) => {
          if error.code == 429 {
            thread::sleep(time::Duration::from_millis(attempt * 500));
            continue;
          } else if error.code == 502 {
            thread::sleep(time::Duration::from_millis(10000));
            continue;
          }
        }
        None => return result
      }
    }
    ResponseObject { data: None, error: Some(ErrorObject { code: 9999, message: "".to_string() }), meta: None }
  }
  
  async fn send<T: de::DeserializeOwned>(&self, request_builder: RequestBuilder) -> ResponseObject<T> {
    let response = match match request_builder
      .send()
      .await {
        Ok(r) => r,
        Err(err) => return ResponseObject { data: None, error: Some(ErrorObject { code: 9999, message: format!("{}", err) }), meta: None }
      }
      .json::<Value>()
      .await {
        Ok(r) => r,
        Err(err) => return ResponseObject { data: None, error: Some(ErrorObject { code: 9999, message: format!("{}", err) }), meta: None }
      };

      let result: Result<ResponseObjectEvent<T>, _> = serde_json::from_value::<ResponseObjectEvent<T>>(response);
      let response_object: Result<ResponseObject<T>, Box<dyn std::error::Error>> = match result {
        Ok(deserialized) => {
          match deserialized {
            ResponseObjectEvent::Object(object) => {
              Ok(ResponseObject {
                data: Some(object),
                error: None,
                meta: None
              })
            }
            ResponseObjectEvent::GetObject(get_object) => {
              match &get_object.error {
                Some(error) => warn!("{}", error.message),
                None => {}
              };
              Ok(get_object)
            }
            ResponseObjectEvent::ErrorObject(error_object) => {
              warn!("{}", error_object.message);
              Ok(ResponseObject {
                data: None,
                error: Some(error_object),
                meta: None
              })
            }
            _ => {
              warn!("Unable to match ResponseObject");
              Ok(ResponseObject {
                data: None,
                error: Some(ErrorObject {
                  code: 9999,
                  message: "Unable to match response object".to_string(),
                }),
                meta: None
              })
            }
          }
        }
        Err(err) => {
          warn!("Failed to deserialized object\n{}", err);
          Ok(ResponseObject {
            data: None,
            error: Some(ErrorObject {
              code: 9999,
              message: "Failed to deserialize object".to_string(),
            }),
            meta: None
          })
        }
      };
  
      match response_object {
        Ok(r) => r,
        Err(err) => {
          warn!("{}", err);
          ResponseObject {
            data: None,
            error: Some(ErrorObject {
              code: 9999,
              message: format!("{}", err),
            }),
            meta: None,
          }
        }
      }
  }
}