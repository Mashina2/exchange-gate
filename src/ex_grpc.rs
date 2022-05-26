use crate::binance;
use ex_gate::{greeter_server::Greeter, BalancesReply, BalancesRequest};
use tonic::{Request, Response, Status};

pub mod ex_gate {
    tonic::include_proto!("ex_gate");
}

pub struct ExGreeter;

#[tonic::async_trait]
impl Greeter for ExGreeter {
    async fn get_balances(
        &self,
        _request: Request<BalancesRequest>,
    ) -> Result<Response<BalancesReply>, Status> {
        let reply = binance::get_account()
            .await
            .map_err(|e| Status::unknown(e.to_string()))?;
        Ok(Response::new(reply))
    }
}
