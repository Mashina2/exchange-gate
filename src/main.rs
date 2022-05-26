mod binance;
mod error;
mod ex_grpc;

use ex_grpc::ex_gate::greeter_server::GreeterServer;
use poem::{endpoint::TowerCompatExt, listener::TcpListener, Route, Server};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Route::new().nest_no_strip(
        "/",
        tonic::transport::Server::builder()
            .add_service(GreeterServer::new(ex_grpc::ExGreeter))
            .into_service()
            .compat(),
    );

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
