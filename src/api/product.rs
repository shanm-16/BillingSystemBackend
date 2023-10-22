use actix_web::web;

use crate::service::product;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::post().to(product::add_product))
            .route("/{product_name}", web::get().to(product::get_product_by_name))
            .route("/product/{product_name}/update",web::put().to(product:: update_product_by_name))
            .route("/product/{product_name}/delete",web::put().to(product:: delete_product_by_name))
    );
}
