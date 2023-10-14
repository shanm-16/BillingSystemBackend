use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct Product {
    pub product_id: String,
    pub merchant_id: String,
    pub product_name: String,
    pub price: String,
    pub quantity: String,
}

#[derive(Deserialize, Serialize)]
pub struct ProductRequest {
    pub merchant_id: String,
    pub product_name: String,
    pub price: String,
    pub quantity: String,
}

#[derive(Deserialize, Serialize)]
pub struct ProductUpdateRequest {
    pub merchant_id: String,
    pub product_name: String,
    pub price: String,
    pub quantity: String,
}