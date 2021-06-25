#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod schema;
mod models;

use diesel::prelude::*;
use std::env;
use schema::users::dsl as user_dsl;
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::{Json};
use diesel::result::Error::NotFound;
use serde::{Serialize};

#[derive(Serialize)]
struct ApiError {
    error: String,
    error_message: String
}

impl ApiError {
    fn new(error: String, error_message: String) -> ApiError {
        ApiError {
            error: error,
            error_message: error_message
        }
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    } 
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[get("/")]
async fn get_users() -> status::Custom<content::Json<String>> {
    
    let database_connection = establish_connection();
    let result = user_dsl::users.load::<models::User>(&database_connection);
    
    match result {
        Ok(data) => {
            let data_json = serde_json::to_string(&data).expect("Error serializing json");
            status::Custom(Status::Ok, content::Json(data_json))
        }
        Err(e) => {
            let api_error = ApiError::new(e.to_string(), "Error listing users".to_string()).to_json();
            status::Custom(Status::InternalServerError, content::Json(api_error))
        }
    }

}

#[get("/<user_id>")]
async fn get_user(user_id: i32) -> status::Custom<content::Json<String>> {
    
    let database_connection = establish_connection();
    let result = user_dsl::users.find(user_id).first::<models::User>(&database_connection);

    match result {
        Ok(data) => {
            let data_json = serde_json::to_string(&data).expect("Error serializing json");
            status::Custom(Status::Ok, content::Json(data_json)) 
        },
        Err(e) => {
            match e {
                NotFound => { 
                    let api_error = ApiError::new(e.to_string(), "Error listing user".to_string()).to_json();
                    status::Custom(Status::NotFound, content::Json(api_error))
                }
                _ =>  {
                    let api_error = ApiError::new(e.to_string(), "Error listing user".to_string()).to_json();
                    status::Custom(Status::InternalServerError, content::Json(api_error))
                }
            }
        }
        
    }

}

#[post("/", format = "application/json", data = "<new_user>")]
async fn create_user(new_user: Json<models::NewUser>) -> status::Custom<content::Json<String>> {
    
    let new_user = new_user.clone();
    let database_connection = establish_connection();
    
    let result = diesel::insert_into(user_dsl::users).values(&new_user).execute(&database_connection);

    match result {
        Ok(_data) => {
            status::Custom(Status::Ok, content::Json(String::from("")))
        },
        Err(e) => {
            let api_error = ApiError::new(e.to_string(), "Error creating user".to_string()).to_json();
            status::Custom(Status::InternalServerError, content::Json(api_error))
        }
    }
}

#[delete("/<user_id>")]
async fn delete_user(user_id: i32) -> status::Custom<content::Json<String>> {
    
    let database_connection = establish_connection();
    let result = diesel::delete(user_dsl::users.find(user_id)).execute(&database_connection);

    match result {
        Ok(_data) => {
            status::Custom(Status::Ok, content::Json(String::from("")))
        },
        Err(e) => {
            let api_error = ApiError::new(e.to_string(), "Error deleting user".to_string()).to_json();
            status::Custom(Status::InternalServerError, content::Json(api_error))
        }
    }
}

#[put("/<user_id>", format = "application/json", data = "<updated_user>")]
async fn update_user(user_id: i32, updated_user: Json<models::NewUser>) -> status::Custom<content::Json<String>> {
    
    let updated_user = updated_user.clone();
    let database_connection = establish_connection();
    let result = diesel::update(user_dsl::users.find(user_id)).set(&updated_user).execute(&database_connection);
    
    match result {
        Ok(_data) => {
            status::Custom(Status::Ok, content::Json(String::from("")))
        },
        Err(e) => {
            let api_error = ApiError::new(e.to_string(), "Error updating user".to_string()).to_json();
            status::Custom(Status::InternalServerError, content::Json(api_error))
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/users", routes![get_user, get_users, create_user, delete_user, update_user])        
}