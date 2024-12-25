use crate::grpc::lol_api::com::mckernant1::lol::teams::Team;
use crate::result::GwenError;
use crate::Result as GwenResult;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use serde_dynamo::from_item;
use crate::util::ddb::DdbScanCompletion;

const TEAMS_TABLE_NAME: &str = "Teams";

pub struct TeamService {
    ddb: Client,
}

impl TeamService {
    pub fn new(ddb: Client) -> Self {
        Self { ddb }
    }

    /// Retrieves a team by its ID.
    pub async fn get_team(&self, team_id: String) -> GwenResult<Team> {
        let response = self
            .ddb
            .get_item()
            .table_name(TEAMS_TABLE_NAME)
            .key("teamId", AttributeValue::S(team_id.to_string()))
            .send()
            .await
            .map_err(|it| it.into_service_error())?;

        match response.item {
            Some(item) => Ok(from_item(item)?),
            None => Err(GwenError::ItemNotFound { key: team_id }),
        }
    }

    /// Scans all teams in the DynamoDB table.
    pub async fn scan_teams(&self) -> GwenResult<Vec<Team>> {
        let teams = self
            .ddb
            .scan()
            .table_name(TEAMS_TABLE_NAME)
            .into_paginator()
            .complete_scan_paginator::<Team>()
            .await?;

        Ok(teams)
    }
}
