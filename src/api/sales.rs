use actix_web::web;

use crate::service::sales;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/saless")
            .route("", web::post().to(sales::add_sales))
            .route("/{sales_name}", web::get().to(sales::get_sales_by_id))
            .route("/sales/{sales_name}/update",web::put().to(sales:: update_sales_by_name))
            .route("/sales/{sales_name}/delete",web::put().to(sales:: delete_sales_by_name))
    );
}