use std::sync::Arc;
use crate::data_access::league_access::LeagueAccess;
use crate::data_access::tournament_access::TournamentAccess;
use crate::grpc::lol_api::com::mckernant1::lol::tournament::tournament_service_server::TournamentService;
use crate::grpc::lol_api::com::mckernant1::lol::tournament::{
    GetMostRecentTournamentRequest, GetMostRecentTournamentResponse, GetOngoingTournamentsRequest,
    GetOngoingTournamentsResponse, GetTournamentRequest, GetTournamentResponse,
    GetTournamentsForLeagueRequest, GetTournamentsForLeagueResponse,
};
use crate::result::GwenError;
use crate::util::grpc_timestamp::TimestampExt;
use crate::util::tournament_ext::TournamentExt;
use chrono::{Duration, Utc};
use tonic::{Request, Response, Status};

pub struct TournamentController {
    tournament_access: Arc<TournamentAccess>,
    league_access: Arc<LeagueAccess>,
}

impl TournamentController {
    pub fn new(
        tournament_access: Arc<TournamentAccess>,
        league_access: Arc<LeagueAccess>
    ) -> Self {
        Self {
            tournament_access,
            league_access
        }
    }
}


#[tonic::async_trait]
impl TournamentService for TournamentController {
    async fn get_tournament(
        &self,
        request: Request<GetTournamentRequest>,
    ) -> Result<Response<GetTournamentResponse>, Status> {
        match self
            .tournament_access
            .get_tournament_by_id(request.into_inner().tournament_id)
            .await
        {
            Ok(t) => Ok(Response::new(GetTournamentResponse {
                tournament: Some(t),
            })),
            Err(GwenError::ItemNotFound { .. }) => {
                Ok(Response::new(GetTournamentResponse { tournament: None }))
            }
            Err(e) => Err(e.into()),
        }
    }

    async fn get_tournaments_for_league(
        &self,
        request: Request<GetTournamentsForLeagueRequest>,
    ) -> Result<Response<GetTournamentsForLeagueResponse>, Status> {
        let league_id = request.into_inner().league_id;

        self.league_access.get_league(league_id.clone()).await?;

        let tournaments = self
            .tournament_access
            .get_tournaments_for_league(league_id)
            .await?;

        Ok(Response::new(GetTournamentsForLeagueResponse {
            tournaments,
        }))
    }

    async fn get_ongoing_tournaments(
        &self,
        _request: Request<GetOngoingTournamentsRequest>,
    ) -> Result<Response<GetOngoingTournamentsResponse>, Status> {
        let tournaments = self.tournament_access.scan_tournaments().await?;

        let tournaments = tournaments
            .into_iter()
            .filter(|it| {
                it.start_time
                    .map(|ts| ts.into_datetime() < Utc::now())
                    .unwrap_or(false)
                    && it
                        .end_time
                        .map(|ts| ts.into_datetime() > Utc::now())
                        .unwrap_or(false)
            })
            .collect::<Vec<_>>();

        Ok(Response::new(GetOngoingTournamentsResponse { tournaments }))
    }

    async fn get_most_recent_tournament(
        &self,
        request: Request<GetMostRecentTournamentRequest>,
    ) -> Result<Response<GetMostRecentTournamentResponse>, Status> {
        let league_id = request.into_inner().league_id;

        self.league_access.get_league(league_id.clone()).await?;

        let mut tournaments = self
            .tournament_access
            .get_tournaments_for_league(league_id.clone())
            .await?;

        tournaments.retain(|it| {
                it.start_time
                    .map(|ts| ts.into_datetime() - Duration::days(7) < Utc::now())
                    .unwrap_or(false)
            });

        tournaments.sort_by_cached_key(|it| it.start_time.map(|ts| ts.into_datetime()));

        if league_id == "WCS" {
            tournaments.retain(|it| it.is_official.unwrap_or(false))
        }

        let tournament = tournaments.into_iter().find(|it| it.is_ongoing());

        Ok(Response::new(GetMostRecentTournamentResponse { tournament }))
    }
}
