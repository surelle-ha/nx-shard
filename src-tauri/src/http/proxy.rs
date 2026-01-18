use axum::{
    Router,
    body::Body,
    extract::Query,
    http::{header, HeaderMap, HeaderValue, Response, StatusCode},
    routing::get,
};
use bytes::Bytes;
use reqwest::Client;
use std::{collections::HashMap, net::SocketAddr};

pub async fn start_proxy() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/hls", get(proxy_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 5199));
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("✅ HLS proxy running at http://{}", addr);
    
    axum::serve(listener, app).await?;
    Ok(())
}

async fn proxy_handler(
    headers: HeaderMap,  // ✅ This IS valid in Axum - ChatGPT was wrong
    Query(params): Query<HashMap<String, String>>,
) -> Result<Response<Body>, StatusCode> {
    let url = params.get("url").ok_or(StatusCode::BAD_REQUEST)?;

    let client = Client::new();
    let mut req = client
        .get(url)
        .header("Referer", "https://megacloud.tv")
        .header("Origin", "https://megacloud.tv")
        .header("User-Agent", "Mozilla/5.0");

    // Forward Range header if present
    if let Some(range) = headers.get(header::RANGE) {
        req = req.header(header::RANGE, range);
    }

    let res = req.send().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    
    let status = res.status();
    let res_headers = res.headers().clone();
    
    // Check if it's an m3u8 file
    let content_type = res_headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    
    let is_m3u8 = content_type.contains("mpegurl") 
        || content_type.contains("m3u8")
        || url.ends_with(".m3u8");

    let mut response = Response::builder().status(status);

    // ✅ Add CORS headers (ChatGPT was right about this)
    response = response
        .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(header::ACCESS_CONTROL_EXPOSE_HEADERS, "Content-Length, Content-Range, Accept-Ranges");

    // Forward important headers
    for (key, value) in &res_headers {
        let key_str = key.as_str();
        if key_str == "content-type"
            || key_str == "content-range"
            || key_str == "accept-ranges"
            || key_str == "content-length"
        {
            response = response.header(key, value);
        }
    }

    if is_m3u8 {
        // Rewrite m3u8 playlists
        let text = res.text().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
        let rewritten = rewrite_m3u8(&text, url);
        
        response = response.header(header::CONTENT_TYPE, "application/vnd.apple.mpegurl");
        Ok(response.body(Body::from(rewritten)).unwrap())
    } else {
        // Stream video segments
        let body = Body::from_stream(res.bytes_stream());
        Ok(response.body(body).unwrap())
    }
}

fn rewrite_m3u8(content: &str, base_url: &str) -> String {
    let base = base_url.rfind('/').map(|i| &base_url[..=i]).unwrap_or(base_url);
    
    content
        .lines()
        .map(|line| {
            if line.starts_with('#') || line.trim().is_empty() {
                line.to_string()
            } else if line.starts_with("http://") || line.starts_with("https://") {
                // Absolute URL - proxy it
                format!("http://127.0.0.1:5199/hls?url={}", urlencoding::encode(line))
            } else {
                // Relative URL - resolve and proxy
                let full_url = format!("{}{}", base, line);
                format!("http://127.0.0.1:5199/hls?url={}", urlencoding::encode(&full_url))
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}