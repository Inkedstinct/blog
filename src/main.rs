use actix_web::{get, post, rt::time::sleep, web, App, HttpResponse, HttpServer, Responder};
use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};


mod homepage;
pub use crate::homepage::home;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let stdout_log = tracing_subscriber::fmt::layer()
    .pretty(); 
    tracing_subscriber::registry()
    .with(
        stdout_log
            // Add an `INFO` filter to the stdout logging layer
            .with_filter(filter::LevelFilter::INFO)
            // Combine the filtered `stdout_log` layer with the
            // `debug_log` layer, producing a new `Layered` layer.
            /* .and_then(debug_log)
            // Add a filter to *both* layers that rejects spans and
            // events whose targets start with `metrics`.
            //.with_filter(filter::filter_fn(|metadata| {
                !metadata.target().starts_with("metrics")
            })*/
        )
    //.with(
    //    // Add a filter to the metrics label that *only* enables
    //    // events whose targets start with `metrics`.
    //    metrics_layer.with_filter(filter::filter_fn(|metadata| {
    //        metadata.target().starts_with("metrics")
    //    }))
    //)
    .init();

    let port = std::env::var("BLOG_PORT").unwrap_or_else(|_|{"9876".to_owned()});
    let address = std::env::var("BLOG_ADDRESS").unwrap_or_else(|_| {"127.0.0.1".to_owned()});
    
    

    let app = || { App::new().service(home) };
    // event!(Level::INFO, context="Starting web server", port = port, address = address);
    HttpServer::new(app)
        .bind(("127.0.0.1", 10101))?
        .workers(1)
        .run()
        .await
} 