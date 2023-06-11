use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

#[derive(PartialEq, Debug, Eq, Deserialize, Serialize)]
pub struct AccountResponse {
    pub account: Account,
}

#[derive(PartialEq, Debug, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: i64,
    pub app_id: App,
    pub api_key: String,
    pub signout: Link,
    pub permissions: Link,
    pub ratelimit: Link,
    pub metafields: Link,
}

#[derive(PartialEq, Debug, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum App {
    Id(String),
    None(bool)
}

#[derive(PartialEq, Debug, Eq, Deserialize, Serialize)]
pub struct Link {
    pub resource: ResourceLink
}

#[derive(PartialEq, Debug, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceLink {
    pub url: String,
    pub link: String,
}

pub struct AccountPermissions {
    pub content: Permission,
    pub products: Permission,
    pub customers: Permission,
    pub orders: Permission,
    pub settings: Permission,
    pub tracking: Permission,
}

pub struct Permission {
    pub read: bool,
    pub write: bool,
}

pub struct RateLimits {
    pub limit_5_min: RateLimit,
    pub limit_hour: RateLimit,
    pub limit_day: RateLimit,
}

pub struct RateLimit {
    pub limit: i64,
    pub remaining: i64,
    pub reset: i64,
    pub reset_time: PrimitiveDateTime,
}