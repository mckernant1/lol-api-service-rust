use std::sync::Arc;
use crate::data_access::player_access::PlayerAccess;
use crate::data_access::teams_access::TeamAccess;
use crate::grpc::lol_api::com::mckernant1::lol::players::player_service_server::PlayerService;
use crate::grpc::lol_api::com::mckernant1::lol::players::{
    GetPlayersOnTeamRequest, GetPlayersOnTeamResponse,
};
use tonic::{Request, Response, Status};

pub struct PlayerController {
    team_access: Arc<TeamAccess>,
    player_access: Arc<PlayerAccess>,
}

impl PlayerController {
    pub fn new(
        team_access: Arc<TeamAccess>,
        player_access: Arc<PlayerAccess>
    ) -> Self {
        Self {
            team_access,
            player_access
        }
    }
}

#[tonic::async_trait]
impl PlayerService for PlayerController {
    async fn get_players_on_team(
        &self,
        request: Request<GetPlayersOnTeamRequest>,
    ) -> Result<Response<GetPlayersOnTeamResponse>, Status> {
        let team_id = request.into_inner().team_id;

        self.team_access.get_team(team_id.clone()).await?;

        let players = self.player_access.get_players_on_team(team_id).await?;

        Ok(Response::new(GetPlayersOnTeamResponse { players }))
    }
}
