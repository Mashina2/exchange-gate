use ex_gate::{greeter_client::GreeterClient, BalancesRequest};

pub mod ex_gate {
    tonic::include_proto!("ex_gate");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut client = GreeterClient::connect("http://[::1]:50051")
        .await
        .unwrap();
    let request = tonic::Request::new(BalancesRequest {
        exchange_name: "Binance".into(),
    });
    let response = client.get_balances(request).await?;
    println!("RESPONSE={:?}", response);
    Ok(())
}
