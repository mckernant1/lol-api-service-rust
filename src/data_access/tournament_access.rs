use crate::grpc::lol_api::com::mckernant1::lol::tournament::Tournament;
use crate::util::ddb::{DdbQueryCompletion, DdbScanCompletion};
use crate::Result as GwenResult;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use crate::result::GwenError;

const TOURNAMENTS_TABLE_NAME: &str = "Tournaments";
const TOURNAMENT_INDEX: &str = "tournamentId-index";

pub struct TournamentService {
    ddb: Client,
}

impl TournamentService {
    pub fn new(ddb: Client) -> Self {
        Self { ddb }
    }

    /// Scans all tournaments in the DynamoDB table.
    pub async fn scan_tournaments(&self) -> GwenResult<Vec<Tournament>>
    {
        let tournaments = self
            .ddb
            .scan()
            .table_name(TOURNAMENTS_TABLE_NAME)
            .into_paginator()
            .complete_scan_paginator::<Tournament>()
            .await?;

        Ok(tournaments)
    }

    /// Retrieves tournaments for a specific league.
    pub async fn get_tournaments_for_league(&self, league_id: String) -> GwenResult<Vec<Tournament>>
    {
        let tournaments = self
            .ddb
            .query()
            .table_name(TOURNAMENTS_TABLE_NAME)
            .key_condition_expression("leagueId = :desiredLeague")
            .expression_attribute_values(
                ":desiredLeague",
                AttributeValue::S(league_id.to_string()),
            )
            .into_paginator()
            .complete_query_paginator::<Tournament>()
            .await?;

        Ok(tournaments)
    }

    /// Retrieves a tournament by its ID.
    pub async fn get_tournament_by_id(&self, tournament_id: String) -> GwenResult<Tournament>
    {
        let paginator = self
            .ddb
            .query()
            .table_name(TOURNAMENTS_TABLE_NAME)
            .index_name(TOURNAMENT_INDEX)
            .key_condition_expression("tournamentId = :desiredTourney")
            .expression_attribute_values(
                ":desiredTourney",
                AttributeValue::S(tournament_id.to_string()),
            )
            .into_paginator()
            .complete_query_paginator::<Tournament>()
            .await?;


        match paginator.into_iter().next() {
            Some(tourney) => Ok(tourney),
            None => Err(GwenError::ItemNotFound { key: tournament_id }),
        }
    }

}
