use crate::grpc::lol_api::com::mckernant1::lol::matches::match_service_server::MatchService;
use crate::grpc::lol_api::com::mckernant1::lol::matches::{
    GetMatchesForTournamentRequest, GetMatchesForTournamentResponse,
};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct MatchController;

#[tonic::async_trait]
impl MatchService for MatchController {
    async fn get_matches_for_tournament(
        &self,
        request: Request<GetMatchesForTournamentRequest>,
    ) -> Result<Response<GetMatchesForTournamentResponse>, Status> {
        todo!()
    }
}
