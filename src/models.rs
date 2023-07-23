use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NftCollection {
    id: String,
    nft_collection: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    item_id: i64,
    nft_contract: String,
    token_id: i64,
    owner: String,
    price: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Offer {
    item_id: i64,
    offerer: String,
    seller: String,
    price: i64,
    is_accepted: bool,
}

#[derive(Debug, serde::Serialize)]
pub struct ApiDescription {
    pub description: String,
    pub github: String,
    pub routes: Vec<RouteDescription>,
}

#[derive(Debug, serde::Serialize)]
pub struct RouteDescription {
    pub route: String,
    pub description: String,
}