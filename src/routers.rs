use actix_web::{web, Scope};

pub fn init() -> Scope {
    web::scope("/api").service(init_object_router())
}

fn init_object_router() -> Scope {
    web::scope("/object")
        .route("/list", web::get().to(crate::handlers::list))
        .route("/detail", web::get().to(crate::handlers::detail))
}
