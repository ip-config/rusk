use phoenix::{
    rpc::{self, rusk_client::RuskClient, PublicKey},
    SecretKey,
};
use tonic::transport::{Channel, Error};

pub async fn create_transaction(
    sk: SecretKey,
    value: u64,
    fee: u64,
    recipient: PublicKey,
    obfuscated: bool,
) -> Result<rpc::Transaction, Box<dyn std::error::Error>> {
    let mut client = client().await?;
    let request = tonic::Request::new(sk.view_key().into());
    let response = client.full_scan_owned_notes(request).await?;

    let request = tonic::Request::new(rpc::NewTransactionRequest {
        sk: Some(sk.into()),
        inputs: response.into_inner().notes,
        recipient: Some(recipient),
        value,
        fee,
        obfuscated,
    });
    let response = client.new_transaction(request).await?;
    Ok(response.into_inner())
}

pub async fn create_bid(
    sk: SecretKey,
    value: u64,
    fee: u64,
) -> Result<rpc::ContractCall, Box<dyn std::error::Error>> {
    let mut client = client().await?;
    let request = tonic::Request::new(sk.view_key().into());
    let response = client.full_scan_owned_notes(request).await?;

    let request = tonic::Request::new(rpc::BidTransactionRequest {
        m: [0u8; 32].to_vec(),
        expiration_height: 100,
        pk: [5u8; 32].to_vec(),
        seed: [10u8; 32].to_vec(),
        tx: Some(rpc::NewTransactionRequest {
            sk: Some(sk.into()),
            inputs: response.into_inner().notes,
            recipient: Some(sk.public_key().into()),
            value,
            fee,
            obfuscated: true,
        }),
    });
    let response = client.new_bid(request).await?;
    Ok(rpc::ContractCall {
        contract_call: Some(rpc::contract_call::ContractCall::Bid(
            response.into_inner(),
        )),
    })
}

pub async fn validate_state_transition(
    tx: rpc::Transaction,
) -> Result<rpc::ValidateStateTransitionResponse, Box<dyn std::error::Error>> {
    let mut client = client().await?;
    let request = tonic::Request::new(rpc::ValidateStateTransitionRequest {
        calls: vec![rpc::ContractCall {
            contract_call: Some(rpc::contract_call::ContractCall::Tx(tx)),
        }],
        current_height: 1,
    });
    let response = client.validate_state_transition(request).await?;

    Ok(response.into_inner())
}

pub async fn client() -> Result<RuskClient<Channel>, Error> {
    Ok(RuskClient::connect("http://127.0.0.1:8080").await?)
}
