use actix_web::{ HttpResponse, web, Error};
use mongodb::Database;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use mongodb::bson::{Bson};
use mongodb::bson;

use crate::model::sales::*;
use crate::utils::random::*;


pub async fn add_sales(data: web::Json<SalesRequest>, db: web::Data<Database>) -> Result<HttpResponse, Error> {
    let sales_request= data.into_inner();

    let sales = Sales {
        sales_id : generate_random_string(10),
        product_id: sales_request.product_id,
        merchant_id: sales_request.merchant_id,
        quantity: sales_request.quantity,
        date: sales_request.date,
        month: sales_request.month,
        year: sales_request.year
    };
    
    let collection = db.collection::<Sales>("sales");

    let insert_result = collection.insert_one(sales, None).await;

    match insert_result {
        Ok(_) => Ok(HttpResponse::Ok().json("sales added successfully")),
        Err(_) => Err(actix_web::error::ErrorInternalServerError("Failed to add the sales")),
    }
}

pub async fn get_sales_by_id(
    path: web::Path<(String,)>,  
    db: web::Data<Database>,    
) -> Result<HttpResponse, Error> {
    let sales_id = &path.0; 

    // Get the saless collection
    let collection = db.collection::<Sales>("sales");

    // Define a filter to find the sales by sales name
    let filter = doc! {"sales_id": sales_id};

    // Use the filter to find the sales in the database
    match collection.find_one(filter, None).await {
        Ok(sales_option) => {
            // Check if the sales was found
            if let Some(sales) = sales_option {
                Ok(HttpResponse::Ok().json(sales))
            } else {
                Ok(HttpResponse::NotFound().json("Sales not found"))
            }
        }
        Err(_) => Err(actix_web::error::ErrorInternalServerError("Failed to find the sales")),
    }
}

pub async fn update_sales_by_name(
    path: web::Path<(String,)>,
    data: web::Json<SalesUpdateRequest>,
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let sales_name = &path.0;

    // Get the saless collection
    let collection = db.collection::<Sales>("sales");

    // Define a filter to find the sales by sales name
    let filter = doc! {"sales_name": sales_name};

    // Define an update document
    let update = doc! {
        "$set": {
            "product_id": data.product_id.clone(),
            "merchant_id": data.merchant_id.clone(),
            "quantity": data.quantity.clone(),
            "date": data.date.clone(),
            "month": data.month.clone(),
            "year": data.year.clone(),

        }
    };

    // Use the filter and update to find and update the sales in the database
    match collection.update_one(filter, update, None).await {
        Ok(result) => {
            if result.modified_count > 0 {
                Ok(HttpResponse::Ok().json("Sales updated successfully"))
            } else {
                Ok(HttpResponse::NotFound().json("Sales not found"))
            }
        }
        Err(_) => Err(actix_web::error::ErrorInternalServerError("Failed to update the sales")),
    }
}



pub async fn delete_sales_by_name(
    path: web::Path<(String,)>,
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let sales_name = &path.0;

    // Get the saless collection
    let collection = db.collection::<Sales>("sales");

    // Define a filter to find the sales by sales name
    let filter = doc! { "sales_name": sales_name };

    // Use the filter to delete the sales from the database
    match collection.delete_one(filter, None).await {
        Ok(result) => {
            if result.deleted_count > 0 {
                Ok(HttpResponse::Ok().json("Sales deleted successfully"))
            } else {
                Ok(HttpResponse::NotFound().json("Sales not found"))
            }
        }
        Err(_) => Err(actix_web::error::ErrorInternalServerError("Failed to delete the sales")),
    }
}

// Todo: Implement date range sales.