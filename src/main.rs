use crate::config::ddb;
use crate::data_access::league_access::LeagueAccess;
use crate::data_access::match_access::MatchAccess;
use crate::data_access::player_access::PlayerAccess;
use crate::data_access::teams_access::TeamAccess;
use crate::data_access::tournament_access::TournamentAccess;
use crate::grpc::lol_api::com::mckernant1::lol::leagues::league_service_server::LeagueServiceServer;
use crate::grpc::lol_api::com::mckernant1::lol::matches::match_service_server::MatchServiceServer;
use crate::grpc::lol_api::com::mckernant1::lol::players::player_service_server::PlayerServiceServer;
use crate::grpc::lol_api::com::mckernant1::lol::teams::team_service_server::TeamServiceServer;
use crate::grpc::lol_api::com::mckernant1::lol::tournament::tournament_service_server::TournamentServiceServer;
pub use result::Result;
use std::sync::Arc;
use tonic::transport::Server;

mod config;
mod data_access;
mod grpc;
mod result;
mod services;
mod util;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    env_logger::init();

    let refl = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(grpc::lol_api::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    let ddb = ddb().await;
    let league_access = Arc::new(LeagueAccess::new(ddb.clone()));
    let match_access = Arc::new(MatchAccess::new(ddb.clone()));
    let player_access = Arc::new(PlayerAccess::new(ddb.clone()));
    let teams_access = Arc::new(TeamAccess::new(ddb.clone()));
    let tournament_access = Arc::new(TournamentAccess::new(ddb));

    Server::builder()
        .accept_http1(true)
        .add_service(refl)
        .add_service(LeagueServiceServer::new(services::LeagueController::new(
            league_access.clone(),
        )))
        .add_service(MatchServiceServer::new(services::MatchController::new(
            match_access,
            tournament_access.clone(),
        )))
        .add_service(PlayerServiceServer::new(services::PlayerController::new(
            teams_access.clone(),
            player_access,
        )))
        .add_service(TeamServiceServer::new(services::TeamController::new(
            teams_access,
        )))
        .add_service(TournamentServiceServer::new(
            services::TournamentController::new(tournament_access, league_access),
        ))
        .serve("127.0.0.1:3000".parse()?)
        .await?;

    Ok(())
}
