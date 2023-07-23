use mongodb::{ Client, bson::doc};
use mongodb::error::Error;
use futures_util::stream::StreamExt;

use crate::models::{NftCollection, Item, Offer};

pub mod fetch {
    use super::*;

    pub async fn get_collections(client: &Client) -> std::result::Result<Vec<NftCollection>, Error> {
        let db: mongodb::Database = client.database("nft-mp");
        let collection: mongodb::Collection<NftCollection> = db.collection::<NftCollection>("collections");
    
        let mut cursor: mongodb::Cursor<NftCollection> = collection.find(doc! {}, None).await?;
    
        let mut collections: Vec<NftCollection> = Vec::new();
    
        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(doc) => {
                    collections.push(doc);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    
        Ok(collections)
    }
    
    pub async fn get_items(client: &Client) -> std::result::Result<Vec<Item>, Error> {
        let db: mongodb::Database = client.database("nft-mp");
        let collection: mongodb::Collection<Item> = db.collection::<Item>("items");
    
        let mut cursor: mongodb::Cursor<Item> = collection.find(doc! {}, None).await?;
    
        let mut items: Vec<Item> = Vec::new();
    
        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(doc) => {
                    items.push(doc);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    
        Ok(items)
    }

    pub async fn get_offers(client: &Client) -> std::result::Result<Vec<Offer>, Error> {
        let db: mongodb::Database = client.database("nft-mp");
        let collection: mongodb::Collection<Offer> = db.collection::<Offer>("offers");
    
        let mut cursor: mongodb::Cursor<Offer> = collection.find(doc! {}, None).await?;
    
        let mut offers: Vec<Offer> = Vec::new();
    
        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(doc) => {
                    offers.push(doc);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    
        Ok(offers)
    }
}