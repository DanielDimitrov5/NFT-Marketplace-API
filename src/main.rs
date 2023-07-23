#[macro_use]
extern crate rocket;

use mongodb::{bson::doc, options::ClientOptions, Client};
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

mod models;
use models::{ApiDescription, Item, NftCollection, Offer, RouteDescription};

mod services {
    pub mod fetch_data;
}

use services::fetch_data::fetch::{get_collections, get_items, get_offers};

#[get("/")]
fn index() -> Json<ApiDescription> {
    let api_index: ApiDescription = ApiDescription {
        description: "This is a REST API for the NFT-Marketplace smart contract. The API provides several GET endpoints for fetching data.".to_string(),
        github: "https://github.com/DanielDimitrov5/NFT-Marketplace-API".to_string(),
        routes: vec![
            RouteDescription {
                route: "/collections".to_string(),
                description: "Returns all NFT collections.".to_string(),
            },
            RouteDescription {
                route: "/items".to_string(),
                description: "Returns all items.".to_string(),
            },
            RouteDescription {
                route: "/offers".to_string(),
                description: "Returns all offers.".to_string(),
            },
        ],
    };

    Json(api_index)
}

#[get("/collections")]
async fn collections(state: &State<Client>) -> Result<Json<Vec<NftCollection>>, Status> {
    match get_collections(state.inner()).await {
        Ok(collections) => Ok(Json(collections)),
        Err(e) => {
            println!("Error getting collections: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/items")]
async fn items(state: &State<Client>) -> Result<Json<Vec<Item>>, Status> {
    match get_items(state.inner()).await {
        Ok(items) => Ok(Json(items)),
        Err(e) => {
            println!("Error getting items: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/offers")]
async fn offers(state: &State<Client>) -> Result<Json<Vec<Offer>>, Status> {
    match get_offers(state.inner()).await {
        Ok(offers) => Ok(Json(offers)),
        Err(e) => {
            println!("Error getting offers: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::on_ignite("Mongo Client", |rocket: rocket::Rocket<rocket::Build>| async {
            let client_options: ClientOptions = ClientOptions::parse("mongodb://localhost:27017")
                .await
                .unwrap();
            let client = Client::with_options(client_options).unwrap();
            rocket.manage(client)
        }))
        .mount("/", routes![index, collections, items, offers])
}
