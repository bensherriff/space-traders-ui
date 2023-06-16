use tauri::State;

use crate::{models::contract::{Contract, ContractAgreementResponse, ContractDelivery, ContractDeliveryResponse}, DataState};

use super::requests::{ResponseObject};

/// List all of your contracts.
#[tauri::command]
pub async fn list_contracts(state: State<'_, DataState>, token: String) -> Result<ResponseObject<Vec<Contract>>, ()> {
  let result = state.request.get_request::<Vec<Contract>>(token, "/my/contracts".to_string(), None).await;
  Ok(result)
}

/// Get the details of a contract by ID.
#[tauri::command]
pub async fn get_contract(state: State<'_, DataState>, token: String, id: String) -> Result<ResponseObject<Contract>, ()> {
  let url = format!("/my/contracts/{}", id);
  let result = state.request.get_request::<Contract>(token, url, None).await;
  Ok(result)
}

/// Accept a contract.
#[tauri::command]
pub async fn accept_contract(state: State<'_, DataState>, token: String, symbol: String) -> Result<ResponseObject<ContractAgreementResponse>, ()> {
  let url = format!("/my/contracts/{}/accept", symbol);
  let result = state.request.post_request::<ContractAgreementResponse>(token, url, None).await;
  Ok(result)
}

/// Deliver cargo on a given contract.
#[tauri::command]
pub async fn deliver_contract(state: State<'_, DataState>, token: String, symbol: String, contract_delivery: ContractDelivery) -> Result<ResponseObject<ContractDeliveryResponse>, ()> {
  let url = format!("/my/contracts/{}/deliver", symbol);
  let body = serde_json::json!({
    "tradeSymbol": contract_delivery.trade_symbol,
    "units": contract_delivery.units,
    "shipSymbol": contract_delivery.ship_symbol
  });
  let result = state.request.post_request::<ContractDeliveryResponse>(token, url, Some(body.to_string())).await;
  Ok(result)
}

/// Fulfill a contract.
#[tauri::command]
pub async fn fulfill_contract(state: State<'_, DataState>, token: String, symbol: String) -> Result<ResponseObject<ContractAgreementResponse>, ()> {
  let url = format!("/my/contracts/{}/fulfill", symbol);
  let result = state.request.post_request::<ContractAgreementResponse>(token, url, None).await;
  Ok(result)
}