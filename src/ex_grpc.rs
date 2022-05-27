use crate::binance;
use ex_gate::{greeter_server::Greeter, BalancesReply, BalancesRequest, PriceRequest, PricesReply};
use tonic::{Request, Response, Status};

use self::ex_gate::{OrderReply, OrderRequest};

pub mod ex_gate {
    tonic::include_proto!("ex_gate");
}

pub struct ExGreeter;

#[tonic::async_trait]
impl Greeter for ExGreeter {
    async fn get_balances(
        &self,
        request: Request<BalancesRequest>,
    ) -> Result<Response<BalancesReply>, Status> {
        let reply: BalancesReply;
        match request.get_ref().exchange_name.as_str() {
            "Binance" => {
                reply = binance::get_account()
                    .await
                    .map_err(|e| Status::unknown(e.to_string()))?;
            }
            _ => return Err(Status::invalid_argument("param exchange_name is wrong")),
        }

        Ok(Response::new(reply))
    }

    async fn get_prices(
        &self,
        request: Request<PriceRequest>,
    ) -> Result<Response<PricesReply>, Status> {
        let reply: PricesReply;
        let params = request.get_ref();
        match params.exchange_name.as_str() {
            "Binance" => {
                reply = binance::get_prices(&params.symbols)
                    .await
                    .map_err(|e| Status::unknown(e.to_string()))?;
            }
            _ => return Err(Status::invalid_argument("param exchange_name is wrong")),
        }
        Ok(Response::new(reply))
    }

    async fn get_order(
        &self,
        request: Request<OrderRequest>,
    ) -> Result<Response<OrderReply>, Status> {
        let reply: OrderReply;
        let params = request.get_ref();
        match params.exchange_name.as_str() {
            "Binance" => {
                reply = binance::get_order(&params.symbol, &params.client_order_id)
                    .await
                    .map_err(|e| Status::unknown(e.to_string()))?;
            }
            _ => return Err(Status::invalid_argument("param exchange_name is wrong")),
        }
        Ok(Response::new(reply))
    }
}
