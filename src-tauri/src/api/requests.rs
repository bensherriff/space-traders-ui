use reqwest::Client;
use serde::{de, Deserialize, Serialize};
use serde_json::Value;

use crate::configuration::Configuration;

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

pub async fn get_request<T: de::DeserializeOwned>(configuration: &Configuration, url: &str, query: Option<Vec<(&str, &String)>>) -> Result<ResponseObject<T>, Box<dyn std::error::Error>> {
  let local_config: &Configuration = configuration;
  let token: &String = &local_config.token;
  let client: &Client = &local_config.client;
  let response: Value;

  match query {
    Some(q) => {
      let mut local_url = format!("{}?", url);
      for (index, item) in q.iter().enumerate() {
        if index < q.len() - 1 {
          local_url = format!("{}{}={}&", local_url, item.0, item.1)
        } else {
          local_url = format!("{}{}={}", local_url, item.0, item.1)
        }
      }
      response = client.get(local_url)
        .bearer_auth(&token)
        .send()
        .await?
        .json::<Value>()
        .await?;
    },
    None => {
      response = client.get(url)
        .bearer_auth(&token)
        .send()
        .await?
        .json::<Value>()
        .await?;
    }
  }
  handle_response(&response)
}

pub async fn post_request<T: de::DeserializeOwned>(configuration: &Configuration, url: &str, body: Option<String>) -> Result<ResponseObject<T>, Box<dyn std::error::Error>> {
  let local_config: &Configuration = configuration;
  let token: &String = &local_config.token;
  let client: Client = Client::new();
  let response: Value;

  match body {
    None => {
      response = client.post(url)
        .bearer_auth(&token)
        .header("Content-Length", "0")
        .send()
        .await?
        .json::<Value>()
        .await?;
    }
    Some(b) => {
      response = client.post(url)
        .bearer_auth(&token)
        .body(b)
        .send()
        .await?
        .json::<Value>()
        .await?;
    }
  }
  handle_response(&response)
}

pub async fn patch_request<T: de::DeserializeOwned>(configuration: &Configuration, url: &str, body: Option<String>) -> Result<ResponseObject<T>, Box<dyn std::error::Error>> {
  let local_config: &Configuration = configuration;
  let token: &String = &local_config.token;
  let client: Client = Client::new();
  let response: Value;

  match body {
    None => {
      response = client.patch(url)
        .bearer_auth(&token)
        .header("Content-Length", "0")
        .send()
        .await?
        .json::<Value>()
        .await?;
    }
    Some(b) => {
      response = client.patch(url)
        .bearer_auth(&token)
        .body(b)
        .send()
        .await?
        .json::<Value>()
        .await?;
    }
  }
  handle_response(&response)
}

fn handle_response<T: de::DeserializeOwned>(response: &Value) -> Result<ResponseObject<T>, Box<dyn std::error::Error>> {
  let _response = response.clone();
  let result: Result<ResponseObjectEvent<T>, _> = serde_json::from_value::<ResponseObjectEvent<T>>(_response);
  match result {
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
          Ok(get_object)
        }
        ResponseObjectEvent::ErrorObject(error_object) => {
          Ok(ResponseObject {
            data: None,
            error: Some(error_object),
            meta: None
          })
        }
        _ => {
          println!("Failed to match ResponseObject\n{:#?}", response);
          Ok(ResponseObject {
            data: None,
            error: Some(ErrorObject {
              code: 0,
              message: "Failed to load object".to_string(),
            }),
            meta: None
          })
        }
      }
    }
    Err(_) => {
      println!("Failed to match deserialized object\n{:#?}", response);
      Ok(ResponseObject {
        data: None,
        error: Some(ErrorObject {
          code: 0,
          message: "Failed to deserialize object".to_string(),
        }),
        meta: None
      })
    }
  }
}

pub fn handle_result<T>(result: Result<ResponseObject<T>, Box<dyn std::error::Error>>) -> ResponseObject<T>{
  match result {
    Ok(r) => r,
    Err(_) => {
      ResponseObject {
        data: None,
        error: Some(ErrorObject {
          code: 0,
          message: "Failed to load object".to_string(),
        }),
        meta: None,
      }
    }
  }
}