use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_dynamodb::Client;

pub async fn aws_config() -> SdkConfig {
    aws_config::defaults(BehaviorVersion::latest()).load().await
}

pub async fn ddb() -> Client {
    Client::new(&aws_config().await)
}
