use crate::grpc::lol_api::com::mckernant1::lol::matches::Match;
use crate::util::ddb::DdbQueryCompletion;
use crate::Result as GwenResult;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use mckernant1_tools::iter::distinct::DistinctExt;

const MATCHES_TABLE_NAME: &str = "Matches";

pub struct MatchAccess {
    ddb: Client,
}

impl MatchAccess {
    pub(crate) fn new(ddb: Client) -> Self {
        Self {
            ddb
        }
    }
}

impl MatchAccess {


    pub async fn get_matches_for_tournament(
        &self,
        tournament_id: String,
    ) -> GwenResult<Vec<Match>> {
        let matches = self
            .ddb
            .query()
            .table_name(MATCHES_TABLE_NAME)
            .key_condition_expression("tournamentId = :desiredTourney")
            .expression_attribute_values(":desiredTourney", AttributeValue::S(tournament_id))
            .into_paginator()
            .complete_query_paginator::<Match>()
            .await?;


        let distinct_matches: Vec<Match> = matches
            .into_iter()
            .distinct_by(|it| {
                (
                    it.red_team_id.clone(),
                    it.blue_team_id.clone(),
                    it.start_time.clone(),
                )
            })
            .collect();

        Ok(distinct_matches)
    }
}
