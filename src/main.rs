#![feature(proc_macro_hygiene)]
#![allow(non_upper_case_globals)]
#![deny(unused_imports)]

use crate::{
    error::{HtmlError, JsonError, PointercrateError},
    middleware::etag::Etag,
    state::PointercrateState,
};
use actix_files::{Files, NamedFile};
use actix_web::{
    middleware::{Logger, NormalizePath},
    web,
    web::{route, scope, JsonConfig, PathConfig, QueryConfig},
    App, HttpRequest, HttpServer, HttpResponse,
};
use rustls::{Certificate, PrivateKey, ServerConfig};
use std::net::SocketAddr;
use std::fs::File as StdFile;
use std::io::{self, BufReader};
use std::sync::Arc;

use api::{
    auth,
    demonlist::{demon, misc, player, record, submitter},
    user,
};

#[macro_use]
mod util;
mod api;
mod cistring;
mod config;
mod documentation;
mod error;
mod extractor;
mod gd;
mod middleware;
mod model;
mod permissions;
mod ratelimit;
mod state;
mod video;
mod view;

#[cfg(test)]
mod test;

pub type Result<T> = std::result::Result<T, PointercrateError>;

pub type ApiResult<T> = std::result::Result<T, JsonError>;
pub type ViewResult<T> = std::result::Result<T, HtmlError>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv::dotenv().expect("Failed to initialize .env file!");

    let application_state = PointercrateState::initialize().await;

    // Load your certificate and private key
    let certs = load_certs("cert.pem")?;
    let key = load_private_key("key.pem")?;

    let tls_config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let tls_config = Arc::new(tls_config);

    HttpServer::new(move || {
        let json_config = JsonConfig::default().error_handler(|error, request| PointercrateError::from(error).dynamic(request.headers()).into());
        let path_config = PathConfig::default().error_handler(|error, request| PointercrateError::from(error).dynamic(request.headers()).into());
        let query_config = QueryConfig::default().error_handler(|error, request| PointercrateError::from(error).dynamic(request.headers()).into());

        App::new()
            .app_data(json_config)
            .app_data(path_config)
            .app_data(query_config)
            .wrap(Etag)
            .wrap(Logger::default())
            .wrap(NormalizePath::default())
            .app_data(application_state.clone())
            .service(Files::new("/static2", "./static2").use_etag(true))
            .route(
                "/robots.txt",
                web::get().to(|req: HttpRequest| NamedFile::open("robots.txt").unwrap().into_response(&req).unwrap()),
            )
            .service(view::home::index)
            .service(view::login::index)
            .service(view::login::post)
            .service(view::login::register)
            .service(view::demonlist::page)
            .service(view::demonlist::index)
            .service(view::account::index)
            .service(view::documentation::index)
            .service(view::documentation::topic)
            .service(view::documentation::guildelines_index)
            .service(view::documentation::guidelines_topic)
            .service(
                scope("/api/v1")
                    .service(misc::list_information)
                    .service(
                        scope("/auth")
                            .service(auth::register)
                            .service(auth::delete_me)
                            .service(auth::get_me)
                            .service(auth::invalidate)
                            .service(auth::login)
                            .service(auth::patch_me),
                    )
                    .service(
                        scope("/users")
                            .service(user::paginate)
                            .service(user::get)
                            .service(user::delete)
                            .service(user::patch),
                    )
                    .service(
                        scope("/submitters")
                            .service(submitter::get)
                            .service(submitter::paginate)
                            .service(submitter::patch),
                    )
                    .service(
                        scope("/demons")
                            .service(demon::v1::get)
                            .service(demon::v1::paginate)
                            .service(demon::v1::patch)
                            .service(demon::v1::delete_creator)
                            .service(demon::v1::post_creator)
                            .service(demon::post),
                    )
                    .service(
                        scope("/records")
                            .service(record::delete)
                            .service(record::get)
                            .service(record::paginate)
                            .service(record::patch)
                            .service(record::submit)
                            .service(record::add_note)
                            .service(record::patch_note)
                            .service(record::delete_note),
                    )
                    .service(
                        scope("/players")
                            .service(player::patch)
                            .service(player::paginate)
                            .service(player::ranking)
                            .service(player::get),
                    ),
            )
            .service(
                scope("/api/v2").service(
                    scope("/demons")
                        .service(demon::v2::paginate_listed)
                        .service(demon::v2::get)
                        .service(demon::v2::paginate)
                        .service(demon::v2::patch)
                        .service(demon::v2::delete_creator)
                        .service(demon::v2::post_creator)
                        .service(demon::post),
                ),
            )
            .default_service(route().to(api::handle_404_or_405))
    })
    .bind_rustls(SocketAddr::from(([0, 0, 0, 0], config::port())), tls_config)?
    .run()
    .await?;

    Ok(())
}

fn load_certs(filename: &str) -> io::Result<Vec<Certificate>> {
    let certfile = StdFile::open(filename)?;
    let mut reader = BufReader::new(certfile);
    rustls_pemfile::certs(&mut reader)
        .map(|mut certs| certs.drain(..).map(Certificate).collect())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid certificate"))
}

fn load_private_key(filename: &str) -> io::Result<PrivateKey> {
    let keyfile = StdFile::open(filename)?;
    let mut reader = BufReader::new(keyfile);
    let keys = rustls_pemfile::rsa_private_keys(&mut reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid private key"))?;
    keys.into_iter().next().map(PrivateKey).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "no private key found"))
}
