use std::time::{SystemTime, UNIX_EPOCH};

use ex_gate::{greeter_client::GreeterClient, BalancesRequest};

use crate::ex_gate::{GetOrderRequest, PriceRequest, CreateOrderRequest};

pub mod ex_gate {
    tonic::include_proto!("ex_gate");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut client = GreeterClient::connect("http://127.0.0.1:3000").await.unwrap();

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

    let client_order_id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();

    let request = tonic::Request::new(CreateOrderRequest {
        exchange_name: "Binance".into(),
        symbol: "BTCUSDT".into(),
        quantity: "0.01".into(),
        side: "SELL".into(),
        client_order_id: client_order_id.clone(),
    });
    let response = client.create_market_order(request).await?;
    println!("RESPONSE={:?}", response);

    let request = tonic::Request::new(GetOrderRequest {
        exchange_name: "Binance".into(),
        symbol: "BTCUSDT".into(),
        client_order_id: client_order_id,
    });
    let response = client.get_order(request).await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}
