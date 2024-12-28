use crate::grpc::lol_api::com::mckernant1::lol::tournament::Tournament;
use crate::util::grpc_timestamp::TimestampExt;
use chrono::{DateTime, DurationRound, TimeDelta, Utc};

pub trait TournamentExt {
    fn is_ongoing(&self) -> bool;
}

impl TournamentExt for Tournament {
    fn is_ongoing(&self) -> bool {
        let now = Utc::now().duration_trunc(TimeDelta::days(1)).unwrap();

        self.start_time
            .map(|it| it.into_datetime())
            .unwrap_or(DateTime::UNIX_EPOCH)
            <= now
            && now
                <= self
                    .end_time
                    .map(|it| it.into_datetime())
                    .unwrap_or(DateTime::UNIX_EPOCH)
    }
}
