use log::warn;
use reqwest::Client;
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

pub async fn get_request<T: de::DeserializeOwned>(client: &Client, token: String, url: String, query: Option<Vec<(&str, String)>>) -> Result<ResponseObject<T>, Box<dyn std::error::Error>> {
  let response: Value;
  let base_url: String = "https://api.spacetraders.io/v2".to_string();
  let uri: String = format!("{}{}", base_url, url);

  match query {
    Some(q) => {
      let mut _uri = format!("{}?", uri);
      for (index, item) in q.iter().enumerate() {
        if index < q.len() - 1 {
          _uri = format!("{}{}={}&", _uri, item.0, item.1)
        } else {
          _uri = format!("{}{}={}", _uri, item.0, item.1)
        }
      }
      response = client.get(_uri)
        .bearer_auth(token)
        .send()
        .await?
        .json::<Value>()
        .await?;
    },
    None => {
      response = client.get(uri)
        .bearer_auth(token)
        .send()
        .await?
        .json::<Value>()
        .await?;
    }
  }
  handle_response(&response)
}

pub async fn post_request<T: de::DeserializeOwned>(client: &Client, token: String, url: String, body: Option<String>) -> Result<ResponseObject<T>, Box<dyn std::error::Error>> {
  let response: Value;
  let base_url: String = "https://api.spacetraders.io/v2".to_string();
  let uri: String = format!("{}{}", base_url, url);

  match body {
    None => {
      response = client.post(uri)
        .bearer_auth(token)
        .header("Content-Length", "0")
        .send()
        .await?
        .json::<Value>()
        .await?;
    }
    Some(b) => {
      response = client.post(uri)
        .bearer_auth(token)
        .header("Content-Length", b.len())
        .header("Content-Type", "application/json".to_string())
        .body(b)
        .send()
        .await?
        .json::<Value>()
        .await?;
    }
  }
  handle_response(&response)
}

pub async fn patch_request<T: de::DeserializeOwned>(client: &Client, token: String, url: String, body: Option<String>) -> Result<ResponseObject<T>, Box<dyn std::error::Error>> {
  let response: Value;
  let base_url: String = "https://api.spacetraders.io/v2".to_string();
  let uri: String = format!("{}{}", base_url, url);

  match body {
    None => {
      response = client.patch(uri)
        .bearer_auth(token)
        .header("Content-Length", "0")
        .send()
        .await?
        .json::<Value>()
        .await?;
    }
    Some(b) => {
      response = client.patch(uri)
        .bearer_auth(token)
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
  let _response: Value = response.clone();
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
          warn!("Failed to match ResponseObject\n{:#?}", response);
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
      warn!("Failed to match deserialized object\n{:#?}", response);
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