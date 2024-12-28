use std::sync::Arc;
use crate::data_access::teams_access::TeamAccess;
use crate::grpc::lol_api::com::mckernant1::lol::teams::team_service_server::TeamService;
use crate::grpc::lol_api::com::mckernant1::lol::teams::{
    GetTeamRequest, GetTeamResponse, ListTeamsRequest, ListTeamsResponse,
};
use crate::result::GwenError;
use tonic::{Request, Response, Status};

pub struct TeamController {
    team_access: Arc<TeamAccess>,
}

impl TeamController {
    pub fn new(team_access: Arc<TeamAccess>) -> Self {
        Self {
            team_access
        }
    }
}

#[tonic::async_trait]
impl TeamService for TeamController {
    async fn list_teams(
        &self,
        _request: Request<ListTeamsRequest>,
    ) -> Result<Response<ListTeamsResponse>, Status> {
        let teams = self.team_access.scan_teams().await?;

        Ok(Response::new(ListTeamsResponse { teams }))
    }

    async fn get_team(
        &self,
        request: Request<GetTeamRequest>,
    ) -> Result<Response<GetTeamResponse>, Status> {
        match self
            .team_access
            .get_team(request.into_inner().team_id)
            .await
        {
            Ok(t) => Ok(Response::new(GetTeamResponse { team: Some(t) })),
            Err(GwenError::ItemNotFound { .. }) => {
                Ok(Response::new(GetTeamResponse { team: None }))
            }
            Err(e) => Err(e.into()),
        }
    }
}
