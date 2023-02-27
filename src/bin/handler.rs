use geo_ip::utils::api_helper::ApiHelper;
use lambda_http::{http::StatusCode, run, service_fn, Error, IntoResponse, Request};
use maxminddb::geoip2;
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::{net::IpAddr, str::FromStr};
use tokio::io::AsyncReadExt;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CACHED_COUNTRY_BUFFER: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![]));
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .without_time()
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .init();

    let config = aws_config::load_from_env().await;
    let s3_client = aws_sdk_s3::Client::new(&config);

    run(service_fn(|event: Request| {
        function_handler(&s3_client, event)
    }))
    .await
}

pub async fn function_handler(
    client: &aws_sdk_s3::Client,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    println!("{:?}", &event);
    let header_ip_address = event.headers().get("x-forwarded-for");
    let bucket_name = std::env::var("BUCKET_NAME").expect("BUCKET_NAME must be set");
    if let Some(header_ip_address) = header_ip_address {
        let mut buf = Vec::new();
        if CACHED_COUNTRY_BUFFER.lock().unwrap().is_empty() {
            let mut stream = client
                .get_object()
                .bucket(bucket_name)
                .key("GeoIP2-Country.mmdb")
                .send()
                .await?
                .body
                .into_async_read();

            stream.read_to_end(&mut buf).await?;

            CACHED_COUNTRY_BUFFER.lock().unwrap().append(&mut buf);
        }

        let reader =
            maxminddb::Reader::from_source(CACHED_COUNTRY_BUFFER.lock().unwrap().clone()).unwrap();

        let ip_address = IpAddr::from_str(header_ip_address.to_str().unwrap()).unwrap();
        let country: geoip2::Country = reader.lookup(ip_address).unwrap();
        if let Some(country) = country.country {
            if let Some(iso_code) = country.iso_code {
                let body = json!({ "countrycode": iso_code }).to_string();
                return Ok(ApiHelper::response(
                    StatusCode::OK,
                    body,
                    "application/json".to_string(),
                ));
            }
        }

        return Ok(ApiHelper::response(
            StatusCode::FORBIDDEN,
            json!({"message": "IP not recognized"}).to_string(),
            "application/json".to_string(),
        ));
    }
    Ok(ApiHelper::response(
        StatusCode::FORBIDDEN,
        json!({"message": "IP is not present in the header request"}).to_string(),
        "application/json".to_string(),
    ))
}
