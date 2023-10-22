use actix_web::{web, Error, HttpResponse};
use mongodb::bson::doc;
use mongodb::Database;

use crate::model::product::*;

pub async fn add_product(
    data: web::Json<ProductRequest>,
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let product_request = data.into_inner();

    let product = Product {
        merchant_id: product_request.merchant_id,
        product_name: product_request.product_name,
        price: product_request.price,
        quantity: product_request.quantity
    };

    let collection = db.collection::<Product>("products");

    let insert_result = collection.insert_one(product, None).await;

    match insert_result {
        Ok(_) => Ok(HttpResponse::Ok().json("Product added successfully")),
        Err(_) => Err(actix_web::error::ErrorInternalServerError(
            "Failed to add the Product",
        )),
    }
}

pub async fn get_product_by_name(
    path: web::Path<(String,)>,
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let product_name = &path.0;

    // Get the products collection
    let collection = db.collection::<Product>("product");

    // Define a filter to find the Product by Product name
    let filter = doc! {"product_name": product_name};

    // Use the filter to find the Product in the database
    match collection.find_one(filter, None).await {
        Ok(product_option) => {
            // Check if the Product was found
            if let Some(product) = product_option {
                Ok(HttpResponse::Ok().json(product))
            } else {
                Ok(HttpResponse::NotFound().json("Product not found"))
            }
        }
        Err(_) => Err(actix_web::error::ErrorInternalServerError(
            "Failed to find the Product",
        )),
    }
}

pub async fn update_product_by_name(
    path: web::Path<(String,)>,
    data: web::Json<ProductUpdateRequest>,
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let product_name = &path.0;

    // Get the products collection
    let collection = db.collection::<Product>("products");

    // Define a filter to find the Product by Product name
    let filter = doc! {"product_name": product_name};

    // Define an update document
    let update = doc! {
        "$set": {
            "merchant_id": data.merchant_id.clone(),
            "product_name": data.product_name.clone(),
            "price": data.price.clone(),
        }
    };

    // Use the filter and update to find and update the Product in the database
    match collection.update_one(filter, update, None).await {
        Ok(result) => {
            if result.modified_count > 0 {
                Ok(HttpResponse::Ok().json("Product updated successfully"))
            } else {
                Ok(HttpResponse::NotFound().json("Product not found"))
            }
        }
        Err(_) => Err(actix_web::error::ErrorInternalServerError(
            "Failed to update the Product",
        )),
    }
}

pub async fn delete_product_by_name(
    path: web::Path<(String,)>,
    db: web::Data<Database>,
) -> Result<HttpResponse, Error> {
    let product_name = &path.0;

    // Get the products collection
    let collection = db.collection::<Product>("products");

    // Define a filter to find the Product by Product name
    let filter = doc! { "product_name": product_name };

    // Use the filter to delete the Product from the database
    match collection.delete_one(filter, None).await {
        Ok(result) => {
            if result.deleted_count > 0 {
                Ok(HttpResponse::Ok().json("Product deleted successfully"))
            } else {
                Ok(HttpResponse::NotFound().json("Product not found"))
            }
        }
        Err(_) => Err(actix_web::error::ErrorInternalServerError(
            "Failed to delete the Product",
        )),
    }
}
