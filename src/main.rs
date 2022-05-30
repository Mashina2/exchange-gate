mod binance;
mod error;
mod ex_grpc;


use ex_grpc::ex_gate::greeter_server::GreeterServer;
use tonic::{transport::Server, Request, Status};

#[tokio::main]
async fn main() -> Result<(), error::GateErr> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:3000".parse().unwrap();
    Server::builder()
        .add_service(GreeterServer::with_interceptor(ex_grpc::ExGreeter, check_auth))
        .serve(addr)
        .await?;

    Ok(())
}

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let addr = req.remote_addr().unwrap().ip().to_string();
    println!("client addr: {}", addr);

    let whitelist = std::env::var("WHITELIST").expect("WHITELIST is not set in .env file");
    if !whitelist.contains(&addr) {
        return Err(Status::unauthenticated("No valid auth token"));
    };
    Ok(req)
}
