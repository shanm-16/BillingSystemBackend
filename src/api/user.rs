use actix_web::web;

use crate::service::user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(user::add_user))
            .route("/{user_name}", web::get().to(user::get_user_by_name))
            .route("/user/{user_name}/update",web::put().to(user:: update_user_by_name))
            .route("/user/{user_name}/delete",web::put().to(user:: delete_user_by_name))
    );
}


