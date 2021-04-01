use crate::handler::default::*;
use crate::handler::graphql::*;
use crate::handler::query::*;
use actix_files as fs;
use actix_web::http::{header, Method, StatusCode};
use actix_web::{error, guard, web, HttpRequest, HttpResponse};
use std::io;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(favicon)
        // register simple route, handle all methods
        .service(welcome)
        // with path parameters
        .service(web::resource("/models/{name}").route(web::get().to(with_param)))
        // async response body
        .service(web::resource("/async-body/{name}").route(web::get().to(response_body)))
        .service(query)
        .service(query_str)
        .service(
            web::resource("/test").to(|req: HttpRequest| match *req.method() {
                Method::GET => HttpResponse::Ok(),
                Method::POST => HttpResponse::MethodNotAllowed(),
                _ => HttpResponse::NotFound(),
            }),
        )
        .service(web::resource("/error").to(|| async {
            error::InternalError::new(
                io::Error::new(io::ErrorKind::Other, "test"),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }))
        .route("/graphql", web::post().to(graphql))
        .route("/graphiql", web::get().to(graphql_playground));
}

pub fn config_static(cfg: &mut web::ServiceConfig) {
    // static files
    cfg.service(fs::Files::new("/static", "../../static").show_files_listing())
        // redirect
        .service(web::resource("/").route(web::get().to(|req: HttpRequest| {
            println!("{:?}", req);
            HttpResponse::Found()
                .header(header::LOCATION, "static/welcome.html")
                .finish()
        })));
}
