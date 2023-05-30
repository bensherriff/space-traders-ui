use crate::{configuration, models::contract::{Contract, ContractAgreementResponse, ContractDelivery, ContractDeliveryResponse}};

use super::requests::{ResponseObject, handle_result, get_request, post_request};

/// List all of your contracts.
pub async fn list_contracts(configuration: &configuration::Configuration) -> ResponseObject<Vec<Contract>> {
  let local_configuration = configuration;
  let url = format!("{}/my/contracts", local_configuration.base_url);
  handle_result(get_request::<Vec<Contract>>(configuration, &url, None).await)
}

/// Get the details of a contract by ID.
pub async fn get_contract(configuration: &configuration::Configuration, id: &String) -> ResponseObject<Contract> {
  let local_configuration = configuration;
  let url = format!("{}/my/contracts/{}", local_configuration.base_url, id);
  handle_result(get_request::<Contract>(configuration, &url, None).await)
}

/// Accept a contract.
pub async fn accept_contract(configuration: &configuration::Configuration, symbol: String) -> ResponseObject<ContractAgreementResponse> {
  let local_configuration = configuration;
  let url = format!("{}/my/contracts/{}/accept", local_configuration.base_url, symbol);
  handle_result(post_request::<ContractAgreementResponse>(configuration, &url, None).await)
}

/// Deliver cargo on a given contract.
pub async fn deliver_contract(configuration: &configuration::Configuration, symbol: String, contract_delivery: ContractDelivery) -> ResponseObject<ContractDeliveryResponse> {
  let local_configuration = configuration;
  let url = format!("{}/my/contracts/{}/deliver", local_configuration.base_url, symbol);
  let body = serde_json::json!({
    "tradeSymbol": contract_delivery.trade_symbol,
    "units": contract_delivery.units,
    "shipSymbol": contract_delivery.ship_symbol
  });
  handle_result(post_request::<ContractDeliveryResponse>(configuration, &url, Some(body.to_string())).await)
}

/// Fulfill a contract.
pub async fn fulfill_contract(configuration: &configuration::Configuration, symbol: String) -> ResponseObject<ContractAgreementResponse> {
  let local_configuration = configuration;
  let url = format!("{}/my/contracts/{}/fulfill", local_configuration.base_url, symbol);
  handle_result(post_request::<ContractAgreementResponse>(configuration, &url, None).await)
}