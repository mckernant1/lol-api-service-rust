use crate::Result;
use aws_sdk_dynamodb::operation::query::paginator::QueryPaginator;
use aws_sdk_dynamodb::operation::scan::paginator::ScanPaginator;
use log::debug;
use serde::Deserialize;
use serde_dynamo::from_items;

pub trait DdbScanCompletion {
    async fn complete_scan_paginator<'a, T>(self) -> Result<Vec<T>>
    where
        T: Deserialize<'a>;
}

impl DdbScanCompletion for ScanPaginator {
    async fn complete_scan_paginator<'a, T: Deserialize<'a>>(self) -> Result<Vec<T>> {
        let result = self.items()
            .send()
            .collect::<std::result::Result<Vec<_>, _>>()
            .await
            .map_err(|it| it.into_service_error())?;
        debug!("Found items {result:?}");
        Ok(from_items(result)?)
    }
}

pub trait DdbQueryCompletion {
    async fn complete_query_paginator<'a, T>(self) -> Result<Vec<T>>
    where
        T: Deserialize<'a>;
}

impl DdbQueryCompletion for QueryPaginator {
    async fn complete_query_paginator<'a, T: Deserialize<'a>>(self) -> Result<Vec<T>> {
        let results: Vec<_> = self.send()
            .collect::<std::result::Result<Vec<_>, _>>()
            .await
            .map_err(|it| it.into_service_error())?
            .into_iter()
            .flat_map(|it| it.items.unwrap_or_default())
            .collect();

        Ok(from_items(results)?)
    }
}
