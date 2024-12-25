use crate::grpc::lol_api::com::mckernant1::lol::leagues::league_service_server::LeagueServiceServer;
use crate::grpc::lol_api::com::mckernant1::lol::matches::match_service_server::MatchServiceServer;
pub use result::Result;
use tonic::transport::Server;

mod config;
mod data_access;
mod grpc;
mod result;
mod services;
mod util;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    Server::builder()
        .accept_http1(true)
        .add_service(LeagueServiceServer::new(services::LeagueController))
        .add_service(MatchServiceServer::new(services::MatchController))
        .serve("127.0.0.1:3000".parse()?)
        .await?;

    Ok(())
}
