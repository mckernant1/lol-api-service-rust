use crate::grpc::lol_api::com::mckernant1::lol::teams::team_service_server::TeamService;
use crate::grpc::lol_api::com::mckernant1::lol::teams::{
    GetTeamRequest, GetTeamResponse, ListTeamsRequest, ListTeamsResponse,
};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct TeamController;

#[tonic::async_trait]
impl TeamService for TeamController {
    async fn list_teams(
        &self,
        request: Request<ListTeamsRequest>,
    ) -> Result<Response<ListTeamsResponse>, Status> {
        todo!()
    }

    async fn get_team(
        &self,
        request: Request<GetTeamRequest>,
    ) -> Result<Response<GetTeamResponse>, Status> {
        todo!()
    }
}
