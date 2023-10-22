use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Sales {
    pub sales_id: String,
    pub product_id: String,
    pub merchant_id: String,
    pub quantity: String,
    pub date: String,
    pub month: String,
    pub year: String
}

#[derive(Deserialize, Serialize)]
pub struct SalesRequest {
    pub product_id: String,
    pub merchant_id: String,
    pub quantity: String,
    pub date: String,
    pub month: String,
    pub year: String
}

#[derive(Deserialize, Serialize)]
pub struct SalesUpdateRequest {
    pub product_id: String,
    pub merchant_id: String,
    pub quantity: String,
    pub date: String,
    pub month: String,
    pub year: String
}

#[derive(Deserialize, Serialize)]
pub struct DateRange {
    pub from_date: String,
    pub from_month: String,
    pub from_year: String,
    pub to_date: String,
    pub to_month: String,
    pub to_year: String,
}