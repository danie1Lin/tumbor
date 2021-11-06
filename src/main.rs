mod pb;
mod engine;
use std::{borrow::Borrow, sync::{Arc}};

use axum::{Router, extract::Path, handler::get};
use base64::{URL_SAFE_NO_PAD, encode_config};
use bytes::Bytes;
use lru::LruCache;
use pb::*;
use percent_encoding::{NON_ALPHANUMERIC, percent_decode, percent_decode_str};
use reqwest::StatusCode;
use serde::Deserialize;
use tokio::sync::Mutex;
use tracing_subscriber::fmt::format;

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}
type Cache = Arc<Mutex<LruCache<u64, bytes::Bytes>>>;

#[tokio::main]
async fn main() {
    let cache: Cache = Arc::new(Mutex::new(LruCache::new(1024)));
    let app = Router::new()
    .route("/image/:spec/:url",get(generate))
    .layer(axum::AddExtensionLayer::new(cache));

    test();

    let addr = "127.0.0.1:3000".parse().unwrap();
    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn generate(Path(Params { spec, url}): Path<Params>) -> Result<String, StatusCode>{
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec.as_str().try_into().map_err(|_| StatusCode::BAD_REQUEST)?;
    let resp = reqwest::get(url.into()).await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    let e : engine::Photon = resp.bytes().await().map_err(|_| StatusCode::BAD_GATEWAY)?;
    Ok(format!("spec: {:?}, url: {}", spec, url))
}

fn test() {
    let spec = ImageSpec{
        specs: [
            Spec{data: Some(spec::Data::Crop(Crop{x1: 50, x2: 100, y1: 50, y2:100}))},
        ].to_vec(),
    };

    let s: String = spec.into();
    let url = percent_encoding::percent_encode("https://www.google.com/url?sa=i&url=https%3A%2F%2Fclickme.net%2F51696&psig=AOvVaw2trfmuMRtE3sfWy-jqPb1R&ust=1636255621714000&source=images&cd=vfe&ved=0CAgQjRxqFwoTCLCNvarlgvQCFQAAAAAdAAAAABAJ".as_bytes(), NON_ALPHANUMERIC).to_string();
    println!("http://localhost:3000/image/{}/{}",s, url);
}