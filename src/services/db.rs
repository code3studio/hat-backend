use std::result;

use actix_web::Error;
use futures_util::StreamExt;
use mongodb::{
    bson::{self, doc, Bson}, options::{ FindOneAndUpdateOptions, IndexOptions, InsertOneOptions, ReturnDocument}, results::InsertOneResult, Client, Collection, IndexModel
};
use serde::{Deserialize, Serialize};

use crate::model::users_model::User;

#[derive(Clone)]
pub struct Database {
    pub user: Collection<User>,
}
#[derive(Debug, Deserialize,Serialize)]
pub struct BestCounter {
    pub username: String,
    pub count: u32,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct AverageCount {
    pub average_count: f64,
}


impl Database {
    pub async fn init() -> Self {
        let url = "mongodb+srv://zhongxi1992:1FIZfgsoYDkS0Bg3@cluster0.x56nkq9.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0"
            .to_string();

        let client = Client::with_uri_str(url.clone()).await.unwrap();

        let db = client.database("hat");

        let user = db.collection("user");

        // Create a unique index on the 'username' field
        let index_model = IndexModel    ::builder()
            .keys(doc! {"username": 1})
            .options(IndexOptions::builder().unique(true).build())
            .build();

        user.create_index(index_model, None).await.expect("Failed to create unique index on username");

        Database { user }
    }

    pub async fn create_user(&self, user: User) -> Result<InsertOneResult, Error> {
        let options = InsertOneOptions::builder().bypass_document_validation(false).build();
        let result = self
            .user
            .insert_one(user, Some(options))
            .await
            .expect("Error creating user");
        Ok(result)
    }
    pub async fn user_count(&self, username: String) -> Result<(), Error> {
        let filter = doc! {"username": username};
        let update = doc! {"$inc": {"count": 1}};
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let result = self
            .user
            .find_one_and_update(filter, update, Some(options))
            .await.expect("error");
        Ok(())
        
    }

    pub async fn get_best_counter(&self) -> Result<Option<BestCounter>, Error> {
        let pipeline = vec![
            doc! {
                "$sort": { "count": -1 }
            },
            doc! {
                "$limit": 1
            },
        ];

        let best_counter = self
            .user
            .aggregate(pipeline, None)
            .await
            .map_err(|e| todo!()).unwrap()
            .next()
            .await;

        match best_counter {
            Some(Ok(doc)) => {
                let best_counter: BestCounter = bson::from_bson(Bson::Document(doc)).unwrap();
                Ok(Some(best_counter))
            }
            _ => Ok(None),
        }
    }

    pub async fn get_average_count(&self) -> Result<Option<AverageCount>, Error> {
        let pipeline = vec![
            doc! {
                "$group": {
                    "_id": null,
                    "average_count": {
                        "$avg": "$count"
                    }
                }
            },
            doc! {
                "$project": {
                    "_id": 0,
                    "average_count": 1
                }
            },
        ];

        let average_count = self
            .user
            .aggregate(pipeline, None)
            .await
            .map_err(|e| todo!()).unwrap()
            .next()
            .await;

        match average_count {
            Some(Ok(doc)) => {
                let average_count: AverageCount = bson::from_bson(Bson::Document(doc)).unwrap();
                Ok(Some(average_count))
            }
            _ => Ok(None),
        }
    }

    pub async fn get_user(&self, username: String) -> Result<User, Error> {
        let filter = doc! {"username": username};
        let result = self.user.find_one(filter, None).await.ok().expect("User not found").unwrap();
        Ok(result) 
    }
    
    // Other methods...
}
