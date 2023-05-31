use crate::{models::contract::{Contract, ContractAgreementResponse, ContractDelivery, ContractDeliveryResponse}};

use super::requests::{ResponseObject, handle_result, get_request, post_request};

/// List all of your contracts.
#[tauri::command]
pub async fn list_contracts(token: String) -> ResponseObject<Vec<Contract>> {
  handle_result(get_request::<Vec<Contract>>(token, "/my/contracts".to_string(), None).await)
}

/// Get the details of a contract by ID.
#[tauri::command]
pub async fn get_contract(token: String, id: String) -> ResponseObject<Contract> {
  let url = format!("/my/contracts/{}", id);
  handle_result(get_request::<Contract>(token, url, None).await)
}

/// Accept a contract.
#[tauri::command]
pub async fn accept_contract(token: String, symbol: String) -> ResponseObject<ContractAgreementResponse> {
  let url = format!("/my/contracts/{}/accept", symbol);
  handle_result(post_request::<ContractAgreementResponse>(token, url, None).await)
}

/// Deliver cargo on a given contract.
#[tauri::command]
pub async fn deliver_contract(token: String, symbol: String, contract_delivery: ContractDelivery) -> ResponseObject<ContractDeliveryResponse> {
  let url = format!("/my/contracts/{}/deliver", symbol);
  let body = serde_json::json!({
    "tradeSymbol": contract_delivery.trade_symbol,
    "units": contract_delivery.units,
    "shipSymbol": contract_delivery.ship_symbol
  });
  handle_result(post_request::<ContractDeliveryResponse>(token, url, Some(body.to_string())).await)
}

/// Fulfill a contract.
#[tauri::command]
pub async fn fulfill_contract(token: String, symbol: String) -> ResponseObject<ContractAgreementResponse> {
  let url = format!("/my/contracts/{}/fulfill", symbol);
  handle_result(post_request::<ContractAgreementResponse>(token, url, None).await)
}