use aws_sdk_dynamodb::operation::get_item::GetItemError;
use aws_sdk_dynamodb::operation::query::QueryError;
use aws_sdk_dynamodb::operation::scan::ScanError;
use log::error;
use thiserror::Error;
use tonic::{Code, Status};

pub type Result<T> = std::result::Result<T, GwenError>;

#[derive(Error, Debug)]
pub enum GwenError {
    #[error("Failed to scan DynamoDb")]
    DynamoDbScan(#[from] ScanError),
    #[error("Failed to get from dynamodb")]
    DynamoDbGet(#[from] GetItemError),
    #[error("Failed to query from dynamodb")]
    DynamoDbQuery(#[from] QueryError),
    #[error("Failed to convert from dynamodb json")]
    Convert(#[from] serde_dynamo::Error),
    #[error("Item not found for key: {key:?}")]
    ItemNotFound { key: String },
}

impl From<GwenError> for Status {
    fn from(value: GwenError) -> Self {
        match value {
            GwenError::ItemNotFound { .. } => Status::new(Code::NotFound, format!("{}", value)),
            _ => {
                error!("Internal error has occurred {value:?}");
                Status::new(Code::Internal, "An Internal Error has Occurred")
            }
        }
    }
}
