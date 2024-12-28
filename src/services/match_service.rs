use crate::data_access::match_access::MatchAccess;
use crate::data_access::tournament_access::TournamentAccess;
use crate::grpc::lol_api::com::mckernant1::lol::matches::match_service_server::MatchService;
use crate::grpc::lol_api::com::mckernant1::lol::matches::{
    GetMatchesForTournamentRequest, GetMatchesForTournamentResponse,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct MatchController {
    match_access: Arc<MatchAccess>,
    tournament_access: Arc<TournamentAccess>,
}

impl MatchController {
    pub fn new(match_access: Arc<MatchAccess>, tournament_access: Arc<TournamentAccess>) -> Self {
        Self {
            match_access,
            tournament_access,
        }
    }
}

#[tonic::async_trait]
impl MatchService for MatchController {
    async fn get_matches_for_tournament(
        &self,
        request: Request<GetMatchesForTournamentRequest>,
    ) -> Result<Response<GetMatchesForTournamentResponse>, Status> {
        let tournament_id = request.into_inner().tournament_id;

        self.tournament_access
            .get_tournament_by_id(tournament_id.clone())
            .await?;

        let matches = self
            .match_access
            .get_matches_for_tournament(tournament_id)
            .await?;

        Ok(Response::new(GetMatchesForTournamentResponse { matches }))
    }
}
