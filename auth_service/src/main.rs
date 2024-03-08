use axum::{routing::get, Router};
use consulrs::client::{ConsulClient, ConsulClientSettingsBuilder};
use consulrs::api::check::common::AgentServiceCheckBuilder;
use consulrs::api::service::requests::RegisterServiceRequest;
use consulrs::service;
mod util;

#[tokio::main]
async fn main() {

    let port = (*util::constants::PORT).clone();
    let _port:u32 = port.parse().unwrap();
    let address = (*util::constants::ADDRESS).clone();
    let consul = (*util::constants::CONSUL).clone();


    // Create a client
    let client = ConsulClient::new(
        ConsulClientSettingsBuilder::default()
            .address(consul)
            .build()
            .unwrap()
    ).unwrap();



    // Create a service named "my_service" with a health check that queries the
    // service via HTTP every 10 seconds.
    service::register(
        &client,
        "auth_service",
        Some(
            RegisterServiceRequest::builder()
                .id(format!("auth_service-{}",port))
                .name(format!("auth_service-{}",port))
                .address(&address)
                .port(_port)
                .check(
                    AgentServiceCheckBuilder::default()
                        .name("health_check")
                        .interval("10s")
                        .http(format!("http://{address}:{port}/health"))
                        .status("passing")
                        .build()
                        .unwrap(),
                ),
        ),
    )
    .await.unwrap();



    server(&port).await;
}



pub async fn server(port: &str) {
   
   let portval = port.to_string();
   
   
    // build our application with a single route
    let app = Router::new()
    .route("/health", get(|| async move { "good"}))
    .route("/api/v1", get(|| async move { format!("Service Port: {portval}")}))
    .route("/api/v1/about", get(|| async move { "This is about page"}));


    // run it with hyper on localhost:3000
    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}