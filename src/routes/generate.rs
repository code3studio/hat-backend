use actix_web::{get, post, web::{Data, Json}, Error, HttpResponse};
use mongodb::bson::oid::ObjectId;
use serde::Serialize;

use crate::{model::users_model::{self, User, UserRequest}, services::db::{AverageCount, BestCounter, Database}};

#[post("/user")]
pub async fn generate(db:Data<Database>,req: Json<UserRequest>) -> HttpResponse {
    
   match db.create_user(User{_id:ObjectId::new(),username:req.username.clone(),count:0}).await {
       Ok(user) => HttpResponse::Ok().json(user),
       Err(e) => HttpResponse::InternalServerError().body(e.to_string())
   } 
    
}
#[post("/user_info")]
pub async fn get_userinfo(db:Data<Database>,req: Json<UserRequest>) -> HttpResponse {
    
   match db.get_user(req.username.clone()).await {
       Ok(user) => HttpResponse::Ok().json(user),
       Err(e) => HttpResponse::InternalServerError().body(e.to_string())
   } 
    
}

#[post("/user_count")]
pub async fn user_count(db:Data<Database>,req: Json<UserRequest>) -> HttpResponse {
    
   match db.user_count(req.username.clone()).await {
       Ok(user) => HttpResponse::Ok().json("success"),
       Err(e) => HttpResponse::InternalServerError().body(e.to_string())
   } 
    
}

#[derive(Debug, Serialize)]
pub struct InfoResponse {
    best_counter: Option<BestCounter>,
    average_count: AverageCount,
}

#[get("/info")]
pub async fn get_info(db: Data<Database>) -> HttpResponse {
    let average_count = match db.get_average_count().await {
        Ok(count) => count.unwrap_or_else(|| AverageCount { average_count: 0.0 }),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let best_counter = match db.get_best_counter().await {
        Ok(user) => user,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let response_data = InfoResponse {
        best_counter,
        average_count,
    };

    HttpResponse::Ok().json(response_data)
}



