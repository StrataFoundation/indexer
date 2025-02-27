//! GraphQL server to read from `holaplex-indexer`

#![deny(
    clippy::suspicious,
    clippy::style,
    missing_debug_implementations,
    missing_copy_implementations
)]
#![warn(clippy::pedantic, clippy::cargo, missing_docs)]

use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{http, middleware, web, App, Error, HttpResponse, HttpServer};
use indexer_core::{clap, clap::Parser, db, db::Pool, prelude::*, ServerOpts};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use crate::schema::{AppContext, Schema};

mod schema;

#[derive(Parser)]
struct Opts {
    #[clap(flatten)]
    server: ServerOpts,

    #[clap(long, env)]
    twitter_bearer_token: Option<String>,

    #[clap(long, env)]
    asset_proxy_endpoint: String,

    #[clap(long, env)]
    asset_proxy_count: u8,
}

struct GraphiqlData {
    uri: String,
}

struct RedirectData {
    route: &'static str,
    new_route: &'static str,
}

pub(crate) struct SharedData {
    schema: Schema,
    pub db: Arc<Pool>,
    pub asset_proxy_endpoint: String,
    pub asset_proxy_count: u8,
    pub twitter_bearer_token: String,
}

#[allow(clippy::unused_async)]
async fn graphiql(data: web::Data<GraphiqlData>) -> HttpResponse {
    let html = graphiql_source(&data.uri, None);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[allow(clippy::unused_async)]
async fn redirect_version(data: web::Data<RedirectData>) -> HttpResponse {
    HttpResponse::MovedPermanently()
        .insert_header(("Location", data.new_route))
        .body(format!(
            "API route {} deprecated, please use {}",
            data.route, data.new_route
        ))
}

async fn graphql(
    data: web::Data<SharedData>,
    req: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = AppContext::new(data.clone().into_inner());
    let resp = req.execute(&data.schema, &ctx).await;

    Ok(HttpResponse::Ok().json(&resp))
}

fn main() {
    indexer_core::run(|| {
        let Opts {
            server,
            twitter_bearer_token,
            asset_proxy_endpoint,
            asset_proxy_count,
        } = Opts::parse();

        let (addr,) = server.into_parts();
        info!("Listening on {}", addr);

        let twitter_bearer_token = twitter_bearer_token.unwrap_or_else(String::new);

        // TODO: db_ty indicates if any actions that mutate the database can be run
        let (db, _db_ty) =
            db::connect(db::ConnectMode::Read).context("Failed to connect to Postgres")?;
        let db = Arc::new(db);

        let shared = web::Data::new(SharedData {
            schema: schema::create(),
            db,
            asset_proxy_endpoint,
            asset_proxy_count,
            twitter_bearer_token,
        });

        let version_extension = "/v1";

        let redirect_data = web::Data::new(RedirectData {
            route: "/v0",
            new_route: "/v1",
        });

        // Should look something like "/..."
        let graphiql_data = web::Data::new(GraphiqlData {
            uri: version_extension.to_owned(),
        });
        assert!(graphiql_data.uri.starts_with('/'));

        actix_web::rt::System::new()
            .block_on(
                HttpServer::new(move || {
                    App::new()
                        .wrap(middleware::Logger::default())
                        .wrap(
                            Cors::default()
                                .allow_any_origin()
                                .allowed_methods(vec!["GET", "POST"])
                                .allowed_headers(vec![
                                    http::header::AUTHORIZATION,
                                    http::header::ACCEPT,
                                ])
                                .allowed_header(http::header::CONTENT_TYPE)
                                .max_age(3600),
                        )
                        .service(
                            web::resource(version_extension)
                                .app_data(shared.clone())
                                .route(web::post().to(graphql)),
                        )
                        .service(
                            web::resource(redirect_data.route)
                                .app_data(redirect_data.clone())
                                .to(redirect_version),
                        )
                        .service(
                            web::resource("/graphiql")
                                .app_data(graphiql_data.clone())
                                .route(web::get().to(graphiql)),
                        )
                })
                .bind(addr)?
                .run(),
            )
            .context("Actix server failed to run")
    });
}
