use actix_web::{post, web::{Data, Json}, Error, HttpResponse};

use crate::{model::users_model::{self, UserRequest}, services::db::Database};

#[post("/user")]
pub async fn generate(db:Data<Database>,req: Json<UserRequest>) -> HttpResponse {
    
   
    // Ok(HttpResponse::Ok().json("success"))
    HttpResponse::Ok().finish()
}
