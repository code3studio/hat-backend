
use actix_web::Error;
use mongodb::{ results::InsertOneResult, Client, Collection};

use crate::model::users_model::User;
#[derive(Clone)]
pub struct Database {
    pub user:Collection<User>
}

impl Database {
    pub async fn init() -> Self {
        let url = "mongodb+srv://zhongxi1992:1FIZfgsoYDkS0Bg3@cluster0.x56nkq9.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0".to_string();
        
      let client =   Client::with_uri_str(url.clone()).await.unwrap();
           
       
        let db = client.database("hat");

        let user = db.collection("user");

       Database {
        user
    }
    }

    pub async fn create_user(&self,user:User)-> Result<InsertOneResult,Error> {
        let result = self.user.insert_one(user, None).await.ok().expect("Error creating user");
        Ok(result)
    }

    // pub async fn check_connection() -> Result<()> {
    //     let url = "mongodb://149.51.230.248:27017".to_string();
    //     let client = Client::with_uri_str(url.clone()).await?;
    //     client.list_database_names(None,None).await?;
    //     Ok(())
    // }
}