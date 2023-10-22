use actix_web::{ HttpResponse, web, Error};
use mongodb::Database;
use mongodb::bson::doc;

use crate::model::user::*;
use crate::utils::random::*;


pub async fn add_user(data: web::Json<UserRequest>, db: web::Data<Database>) -> Result<HttpResponse, Error> {
    let user_request= data.into_inner();

    let user = User {
        user_id : generate_random_string(10),
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
    path: web::Path<(String,)>,  
    db: web::Data<Database>,    
) -> Result<HttpResponse, Error> {
    let user_name = &path.0; 

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

pub async fn update_user_by_name(
    path: web::Path<(String,)>,
    data: web::Json<UserUpdateRequest>,
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let user_name = &path.0;

    // Get the users collection
    let collection = db.collection::<User>("users");

    // Define a filter to find the user by user name
    let filter = doc! {"user_name": user_name};

    // Define an update document
    let update = doc! {
        "$set": {
            "password": data.password.clone(),
            "mode_of_login": data.mode_of_login.clone(),
            "merchant_name": data.merchant_name.clone(),
        }
    };

    // Use the filter and update to find and update the user in the database
    match collection.update_one(filter, update, None).await {
        Ok(result) => {
            if result.modified_count > 0 {
                Ok(HttpResponse::Ok().json("User updated successfully"))
            } else {
                Ok(HttpResponse::NotFound().json("User not found"))
            }
        }
        Err(_) => Err(actix_web::error::ErrorInternalServerError("Failed to update the user")),
    }
}



pub async fn delete_user_by_name(
    path: web::Path<(String,)>,
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let user_name = &path.0;

    // Get the users collection
    let collection = db.collection::<User>("users");

    // Define a filter to find the user by user name
    let filter = doc! { "user_name": user_name };

    // Use the filter to delete the user from the database
    match collection.delete_one(filter, None).await {
        Ok(result) => {
            if result.deleted_count > 0 {
                Ok(HttpResponse::Ok().json("User deleted successfully"))
            } else {
                Ok(HttpResponse::NotFound().json("User not found"))
            }
        }
        Err(_) => Err(actix_web::error::ErrorInternalServerError("Failed to delete the user")),
    }
}