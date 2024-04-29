use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize,Default)]
pub struct User {
  pub  _id: ObjectId,
  pub  username:String,
  pub count: u32
}
#[derive(Debug,Deserialize,Serialize)]
pub struct UserRequest {
  // pub  _id: ObjectId,
  pub  username:String,
  // pub count: u32
}

