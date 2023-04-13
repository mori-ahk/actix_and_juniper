use actix_web::{
    App,
    HttpResponse,
    HttpServer,
    Error,
    main,
    web::{self, Data},
    middleware,
};

use std::env;
mod schema;
mod database;
mod actix_graphql_handlers;

async fn graphiql_route() -> Result<HttpResponse, Error> {
    actix_graphql_handlers::graphiql_handler("/graphql", None).await
}

async fn playground_route() -> Result<HttpResponse, Error> {
    actix_graphql_handlers::playground_handler("/graphql", None).await
}

async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<schema::Schema>,
) -> Result<HttpResponse, Error> {
    let context = database::Database::new();
    actix_graphql_handlers::graphql_handler(&schema, &context, req, payload).await
}

#[main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_INFO", "info");
   
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema::schema()))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(graphql_route)),
            )
            .service(web::resource("/playground").route(web::get().to(playground_route)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql_route)))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
    .await
}
