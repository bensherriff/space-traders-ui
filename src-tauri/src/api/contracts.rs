use reqwest::Client;
use tauri::State;

use crate::{models::contract::{Contract, ContractAgreementResponse, ContractDelivery, ContractDeliveryResponse}};

use super::requests::{ResponseObject, handle_result, get_request, post_request};

/// List all of your contracts.
#[tauri::command]
pub async fn list_contracts(client: State<'_, Client>, token: String) -> Result<ResponseObject<Vec<Contract>>, ()> {
  let result = handle_result(get_request::<Vec<Contract>>(&client, token, "/my/contracts".to_string(), None).await);
  Ok(result)
}

/// Get the details of a contract by ID.
#[tauri::command]
pub async fn get_contract(client: State<'_, Client>, token: String, id: String) -> Result<ResponseObject<Contract>, ()> {
  let url = format!("/my/contracts/{}", id);
  let result = handle_result(get_request::<Contract>(&client, token, url, None).await);
  Ok(result)
}

/// Accept a contract.
#[tauri::command]
pub async fn accept_contract(client: State<'_, Client>, token: String, symbol: String) -> Result<ResponseObject<ContractAgreementResponse>, ()> {
  let url = format!("/my/contracts/{}/accept", symbol);
  let result = handle_result(post_request::<ContractAgreementResponse>(&client, token, url, None).await);
  Ok(result)
}

/// Deliver cargo on a given contract.
#[tauri::command]
pub async fn deliver_contract(client: State<'_, Client>, token: String, symbol: String, contract_delivery: ContractDelivery) -> Result<ResponseObject<ContractDeliveryResponse>, ()> {
  let url = format!("/my/contracts/{}/deliver", symbol);
  let body = serde_json::json!({
    "tradeSymbol": contract_delivery.trade_symbol,
    "units": contract_delivery.units,
    "shipSymbol": contract_delivery.ship_symbol
  });
  let result = handle_result(post_request::<ContractDeliveryResponse>(&client, token, url, Some(body.to_string())).await);
  Ok(result)
}

/// Fulfill a contract.
#[tauri::command]
pub async fn fulfill_contract(client: State<'_, Client>, token: String, symbol: String) -> Result<ResponseObject<ContractAgreementResponse>, ()> {
  let url = format!("/my/contracts/{}/fulfill", symbol);
  let result = handle_result(post_request::<ContractAgreementResponse>(&client, token, url, None).await);
  Ok(result)
}