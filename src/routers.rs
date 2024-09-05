use actix_web::{web, Scope};

pub fn init() -> Scope {
    web::scope("/api").service(init_pic_router())
}

fn init_pic_router() -> Scope {
    web::scope("/pic")
        .route("/list", web::get().to(crate::handlers::list_pic))
        .route("/detail/{id}", web::get().to(crate::handlers::detail))
}
