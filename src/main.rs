pub mod reg;
pub mod auth;
#[macro_use] extern crate rocket;

pub mod mate;
pub mod prod;
pub mod db;

use mate::*;
use prod::*;

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

// use rocket::serde::{Serialize, Deserialize, json::{Json, Value}};
use rocket_db_pools::Database;
use crate::db::DbMeuBanco;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .attach(db::DbMeuBanco::init())
        .mount("/", routes![ 
            hello,  
            //produtos, ver mod prod
            form_prod, 
            newprod,
            getprod,
            getallprod,   
            
            //materiais ver mod mate
            form_mate,
            newmate,
            getmate,
            getallmate,
            ]
    
    )
        .launch()
        .await;
}

