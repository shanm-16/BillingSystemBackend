use actix_web::{ HttpResponse, web, Error};
use mongodb::Database;
use mongodb::bson::doc;

use crate::model::user::*;


pub async fn add_user(data: web::Json<UserRequest>, db: web::Data<Database>) -> Result<HttpResponse, Error> {
    let user_request= data.into_inner();

    let user = User {
        password : user_request.password,
        mode_of_login : user_request.mode_of_login,
        merchant_name : user_request.merchant_name,
        user_name : user_request.user_name,
    };
    
    let collection = db.collection::<User>("users");

    let insert_result = collection.insert_one(user, None).await;

    match insert_result {
        Ok(_) => Ok(HttpResponse::Ok().json("user added successfully")),
        Err(_) => Err(actix_web::error::ErrorInternalServerError("Failed to add the user")),
    }
}

pub async fn get_user_by_name(
    path: web::Path<(String,)>,  // Capture the user_name path parameter
    db: web::Data<Database>,    // Inject the database connection
) -> Result<HttpResponse, Error> {
    let user_name = &path.0; // Extract the user name from the path parameter

    // Get the users collection
    let collection = db.collection::<User>("users");

    // Define a filter to find the user by user name
    let filter = doc! {"user_name": user_name};

    // Use the filter to find the user in the database
    match collection.find_one(filter, None).await {
        Ok(user_option) => {
            // Check if the user was found
            if let Some(user) = user_option {
                Ok(HttpResponse::Ok().json(user))
            } else {
                Ok(HttpResponse::NotFound().json("User not found"))
            }
        }
        Err(_) => Err(actix_web::error::ErrorInternalServerError("Failed to find the user")),
    }
}

