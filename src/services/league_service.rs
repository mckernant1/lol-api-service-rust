use std::sync::Arc;
use crate::data_access::league_access::LeagueAccess;
use crate::grpc::lol_api::com::mckernant1::lol::leagues::league_service_server::LeagueService;
use crate::grpc::lol_api::com::mckernant1::lol::leagues::{
    GetLeagueRequest, GetLeagueResponse, ListLeaguesRequest, ListLeaguesResponse,
};
use crate::result::GwenError;
use tonic::{Request, Response, Status};

pub struct LeagueController {
    league_access: Arc<LeagueAccess>,
}

impl LeagueController {
    pub(crate) fn new(league_access: Arc<LeagueAccess>) -> Self {
        Self { league_access }
    }
}

#[tonic::async_trait]
impl LeagueService for LeagueController {
    async fn list_leagues(
        &self,
        _request: Request<ListLeaguesRequest>,
    ) -> Result<Response<ListLeaguesResponse>, Status> {
        let leagues = self.league_access.scan_leagues().await?;

        Ok(Response::new(ListLeaguesResponse { leagues }))
    }

    async fn get_league(
        &self,
        request: Request<GetLeagueRequest>,
    ) -> Result<Response<GetLeagueResponse>, Status> {
        match self
            .league_access
            .get_league(request.into_inner().league_id)
            .await
        {
            Ok(l) => Ok(Response::new(GetLeagueResponse { league: Some(l) })),
            Err(GwenError::ItemNotFound { .. }) => {
                Ok(Response::new(GetLeagueResponse { league: None }))
            }
            Err(e) => Err(e.into())
        }
    }
}
