use core::time;
use std::thread;
use log::{warn, trace};
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
    trace!("GET {}", uri);
    self.send(self.client.get(uri).bearer_auth(_token)).await
  }
  
  pub async fn post_request<T: de::DeserializeOwned>(&self, token: String, url: String, body: Option<String>) -> ResponseObject<T> {
    let uri: String = format!("{}{}", self.base_url, url);
    let _token = token.to_owned();
    let _body = body.to_owned();
    trace!("POST {} {:?}", uri, _body);
    match _body {
      None => {
        self.send(self.client.post(uri).bearer_auth(_token).header("Content-Length", "0"))
      }
      Some(b) => {
        self.send(self.client.post(uri).bearer_auth(_token).header("Content-Type", "application/json".to_string()).body(b))
      }
    }.await
  }
  
  pub async fn patch_request<T: de::DeserializeOwned>(&self, token: String, url: String, body: Option<String>) -> ResponseObject<T> {
    let uri: String = format!("{}{}", self.base_url, url);
    let _token = token.to_owned();
    let _body = body.to_owned();
    trace!("PATCH {} {:?}", uri, _body);
    match _body {
      None => {
        self.send(self.client.patch(uri).bearer_auth(_token).header("Content-Length", "0"))
      }
      Some(b) => {
        self.send(self.client.patch(uri).bearer_auth(_token).header("Content-Type", "application/json".to_string()).body(b))
      }
    }.await
  }
  
  // TODO: Add promise response (rate limiter)
  async fn send<T: de::DeserializeOwned>(&self, request_builder: RequestBuilder) -> ResponseObject<T> {
    let mut attempt = 0;
    while attempt < self.max_attemps {
      attempt += 1;
      let rb = request_builder.try_clone().unwrap();
      let response = match match rb
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
                Some(error) => trace!("{}", error.message),
                None => {}
              };
              Ok(get_object)
            }
            ResponseObjectEvent::ErrorObject(error_object) => {
              trace!("{}", error_object.message);
              Ok(ResponseObject {
                data: None,
                error: Some(error_object),
                meta: None
              })
            }
            ResponseObjectEvent::None => {
              Ok(ResponseObject {
                data: None,
                error: None,
                meta: None
              })
            }
          }
        }
        Err(err) => {
          Ok(ResponseObject {
            data: None,
            error: Some(ErrorObject {
              code: 9999,
              message: format!("Failed to deserialize object: {}", err).to_string(),
            }),
            meta: None
          })
        }
      };
    
      match response_object {
        Ok(r) => {
          let exp: u64 = 2;
          match &r.error {
            Some(error) => {
              let timeout = exp.pow(attempt as u32);
              // Rate Limiter
              if error.code == 429 {
                thread::sleep(time::Duration::from_secs(timeout));
                continue;
              // DDoS Protection
              } else if error.code == 502 {
                thread::sleep(time::Duration::from_secs(timeout + (10 * attempt as u64)));
                continue;
              // Cooldown Conflict
              } else if error.code == 4000 {
                let mut parts = error.message.split_whitespace().map(|s| s.parse::<i32>());
                match parts.next() {
                  Some(Ok(remainder)) => {
                    warn!("Action is still on cooldown for {} seconds.", remainder);
                    thread::sleep(time::Duration::from_secs(remainder.to_owned() as u64));
                  }
                  _ => {
                    warn!("Unable to determine cooldown time, defaulting to {} seconds.", timeout);
                    thread::sleep(time::Duration::from_secs(timeout));
                  }
                }
              } else {
                warn!("{}: {}", error.code, error.message);
                return r
              }
            }
            None => return r
          }
        },
        Err(err) => {
          warn!("Error handling response: {}", err);
          return ResponseObject {
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
    ResponseObject { data: None, error: Some(ErrorObject { code: 9999, message: "Unable to send request".to_string() }), meta: None }
  }
}