use crate::grpc::lol_api::com::mckernant1::lol::players::player_service_server::PlayerService;
use crate::grpc::lol_api::com::mckernant1::lol::players::{
    GetPlayersOnTeamRequest, GetPlayersOnTeamResponse,
};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct PlayerController;

#[tonic::async_trait]
impl PlayerService for PlayerController {
    async fn get_players_on_team(
        &self,
        request: Request<GetPlayersOnTeamRequest>,
    ) -> Result<Response<GetPlayersOnTeamResponse>, Status> {
        todo!()
    }
}
