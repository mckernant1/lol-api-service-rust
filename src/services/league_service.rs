use crate::grpc::lol_api::com::mckernant1::lol::leagues::league_service_server::LeagueService;
use crate::grpc::lol_api::com::mckernant1::lol::leagues::{
    GetLeagueRequest, GetLeagueResponse, ListLeaguesRequest, ListLeaguesResponse,
};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct LeagueController;

#[tonic::async_trait]
impl LeagueService for LeagueController {
    async fn list_leagues(
        &self,
        request: Request<ListLeaguesRequest>,
    ) -> Result<Response<ListLeaguesResponse>, Status> {
        todo!()
    }

    async fn get_league(
        &self,
        request: Request<GetLeagueRequest>,
    ) -> Result<Response<GetLeagueResponse>, Status> {
        todo!()
    }
}
