use crate::grpc::lol_api::com::mckernant1::lol::leagues::League;
use crate::result::GwenError;
use crate::util::ddb::DdbScanCompletion;
use crate::Result as GwenResult;
use aws_sdk_dynamodb::Client;
use serde_dynamo::aws_sdk_dynamodb_1::from_item;

const LEAGUES_TABLE_NAME: &str = "Leagues";

pub struct LeagueAccess {
    ddb: Client,
}

impl LeagueAccess {
    fn new(ddb: Client) -> LeagueAccess {
        LeagueAccess { ddb }
    }

    async fn scan_leagues(&self) -> GwenResult<Vec<League>> {
        let leagues = self
            .ddb
            .scan()
            .table_name(LEAGUES_TABLE_NAME)
            .into_paginator()
            .complete_scan_paginator::<League>()
            .await?;

        Ok(leagues)
    }

    async fn get_league(&self, league_id: String) -> GwenResult<League> {
        let league_item = self
            .ddb
            .get_item()
            .table_name(LEAGUES_TABLE_NAME)
            .send()
            .await
            .map_err(|it| it.into_service_error())?
            .item
            .ok_or(GwenError::ItemNotFound { key: league_id })?;

        let league: League = from_item(league_item)?;

        Ok(league)
    }
}
