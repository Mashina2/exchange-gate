mod binance;
mod error;
mod ex_grpc;

use ex_grpc::ex_gate::greeter_server::GreeterServer;

use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), error::GateErr> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();
    let addr = "[::1]:50051".parse().unwrap();
    Server::builder()
        .add_service(GreeterServer::new(ex_grpc::ExGreeter))
        .serve(addr)
        .await?;

    Ok(())
}
