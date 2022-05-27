use ex_gate::{greeter_client::GreeterClient, BalancesRequest};

use crate::ex_gate::{OrderRequest, PriceRequest};

pub mod ex_gate {
    tonic::include_proto!("ex_gate");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await.unwrap();

    let request = tonic::Request::new(BalancesRequest {
        exchange_name: "Binance".into(),
    });
    let response = client.get_balances(request).await?;
    println!("RESPONSE={:?}", response);

    let request = tonic::Request::new(PriceRequest {
        exchange_name: "Binance".into(),
        symbols: vec![
            "ETHBTC".to_string(),
            "LTCBTC".to_string(),
            "BTCUSDT".to_string(),
        ],
    });
    let response = client.get_prices(request).await?;
    println!("RESPONSE={:?}", response);

    let request = tonic::Request::new(OrderRequest {
        exchange_name: "Binance".into(),
        symbol: "BNBUSDT".into(),
        client_order_id: "1".into(),
    });
    let response = client.get_order(request).await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}
