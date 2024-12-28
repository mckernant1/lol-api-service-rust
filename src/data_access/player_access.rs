use crate::grpc::lol_api::com::mckernant1::lol::players::Player;
use crate::Result as GwenResult;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use crate::util::ddb::DdbQueryCompletion;

const PLAYERS_TABLE_NAME: &str = "Players";
// const PLAYERS_ID_INDEX: &str = "id-index";
pub struct PlayerAccess {
    ddb: Client,
}

impl PlayerAccess {
    pub fn new(ddb: Client) -> Self {
        Self { ddb }
    }

    /// Retrieves players on a specific team.
    pub async fn get_players_on_team(&self, team_id: String) -> GwenResult<Vec<Player>> {
        let players = self
            .ddb
            .query()
            .table_name(PLAYERS_TABLE_NAME)
            .key_condition_expression("teamId = :desiredTeam")
            .expression_attribute_values(":desiredTeam", AttributeValue::S(team_id))
            .into_paginator()
            .complete_query_paginator::<Player>()
            .await?;

        Ok(players)
    }
}
