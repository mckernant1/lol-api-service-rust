use chrono::{DateTime, Utc};
use crate::grpc::lol_api::google::protobuf::Timestamp;

pub trait TimestampExt {
    fn into_datetime(self) -> DateTime<Utc>;
}


impl TimestampExt for Timestamp {
    fn into_datetime(self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.seconds, self.nanos as u32).unwrap()
    }
}

