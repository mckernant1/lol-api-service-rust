use crate::grpc::lol_api::com::mckernant1::lol::tournament::tournament_service_server::TournamentService;
use crate::grpc::lol_api::com::mckernant1::lol::tournament::{
    GetMostRecentTournamentRequest, GetMostRecentTournamentResponse, GetOngoingTournamentsRequest,
    GetOngoingTournamentsResponse, GetTournamentRequest, GetTournamentResponse,
    GetTournamentsForLeagueRequest, GetTournamentsForLeagueResponse,
};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct TournamentController;

#[tonic::async_trait]
impl TournamentService for TournamentController {
    async fn get_tournament(
        &self,
        request: Request<GetTournamentRequest>,
    ) -> Result<Response<GetTournamentResponse>, Status> {
        todo!()
    }

    async fn get_tournaments_for_league(
        &self,
        request: Request<GetTournamentsForLeagueRequest>,
    ) -> Result<Response<GetTournamentsForLeagueResponse>, Status> {
        todo!()
    }

    async fn get_ongoing_tournaments(
        &self,
        request: Request<GetOngoingTournamentsRequest>,
    ) -> Result<Response<GetOngoingTournamentsResponse>, Status> {
        todo!()
    }

    async fn get_most_recent_tournament(
        &self,
        request: Request<GetMostRecentTournamentRequest>,
    ) -> Result<Response<GetMostRecentTournamentResponse>, Status> {
        todo!()
    }
}
