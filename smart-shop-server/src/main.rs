use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use urlencoding;
mod claims;
use serde_json;
use claims::Claims;

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
struct PublicResponse {
    message: String,
}

#[get("/public")]
fn public() -> Json<PublicResponse> {
    Json(PublicResponse {
        message: "This endpoint is open to anyone".to_string(),
    })
}

#[derive(Serialize)]
struct PrivateResponse {
    message: String,
    user: String,
}

// More details on Rocket request guards can be found here
// https://rocket.rs/v0.5-rc/guide/requests/#request-guards
#[get("/private")]
fn private(user: Claims) -> Json<PrivateResponse> {
    Json(PrivateResponse {
        message: "The `Claims` request guard ensures only valid JWTs can access this endpoint"
            .to_string(),
        user: user.name,
    })
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[derive(Clone)]
struct AppConfig {
    woolworth: ApiKey,
    coles: ApiKey,
}

// Add this struct to store configuration
#[derive(Clone)]
struct ApiKey {
    api_key: String,
    api_secret: String,
}

/// Tries to authenticate a user. Successful authentications get a JWT
#[post("/login", data = "<login>")]
fn login(
    login: Json<LoginRequest>,
    config: &rocket::State<AppConfig>,
) -> Result<Json<LoginResponse>, Custom<String>> {
    if login.username != config.woolworth.api_key || login.password != config.woolworth.api_secret {
        return Err(Custom(
            Status::Unauthorized,
            "account was not found".to_string(),
        ));
    }

    let claim = Claims::from_name(&login.username);
    let response = LoginResponse {
        token: claim.into_token()?,
    };

    Ok(Json(response))
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_rocket::ShuttleRocket {
    // Get Woolworth secrets
    let woolworth = ApiKey {
        api_key: secrets
            .get("WOOLWORTH_API_KEY")
            .expect("WOOLWORTH_API_KEY is required"),
        api_secret: secrets
            .get("WOOLWORTH_API_SECRET")
            .expect("WOOLWORTH_API_SECRET is required"),
    };

    // Get Coles secrets
    let coles = ApiKey {
        api_key: secrets
            .get("COLES_API_KEY")
            .expect("COLES_API_KEY is required"),
        api_secret: secrets
            .get("COLES_API_SECRET")
            .expect("COLES_API_SECRET is required"),
    };

    let rocket = rocket::build()
        .mount("/", routes![public, private, login, product_search])
        .manage(AppConfig { woolworth, coles });

    Ok(rocket.into())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ColesProduct {
    barcode: Option<String>,
    product_name: String,
    product_brand: String,
    product_size: Option<String>,
    current_price: f64,
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct WoolworthsProduct {
    barcode: Option<String>,
    product_name: String,
    product_brand: String,
    product_size: Option<String>,
    current_price: f64,
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ColesApiResponse {
    query: String,
    results: Option<Vec<ColesProduct>>,
    total_results: Option<i32>,
    total_pages: Option<i32>,
    current_page: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct WoolworthsApiResponse {
    query: String,
    results: Option<Vec<WoolworthsProduct>>,
    total_results: Option<i32>,
    total_pages: Option<i32>,
    current_page: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ProductSearchResponse {
    name: String,
    coles: Option<serde_json::Value>,
    woolworths: Option<serde_json::Value>,
}

#[get("/product-search?<name>")]
async fn product_search(
    name: String,
    config: &rocket::State<AppConfig>,
) -> Result<Json<ProductSearchResponse>, String> {
    println!("name: {}", name);
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    
    headers.insert(
        "x-rapidapi-key",
        HeaderValue::from_str(&config.woolworth.api_key).unwrap(),
    );

    // Coles API call
    let coles_response = client
        .get(format!(
            "https://coles-product-price-api.p.rapidapi.com/coles/product-search/?page=1&page_size=20&query={}",
            name
        ))
        .headers(headers.clone())
        .header("x-rapidapi-host", "coles-product-price-api.p.rapidapi.com")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    // Debug print raw Coles response
    let coles_raw_json: serde_json::Value = coles_response.json().await.map_err(|e| e.to_string())?;
    println!("Coles Raw Response: {}", serde_json::to_string_pretty(&coles_raw_json).unwrap());
    
    // Convert to your struct
    // let coles_results = serde_json::from_value::<ColesApiResponse>(coles_raw_json).ok();

    // Woolworths API call
    let woolworths_response = client
        .get(format!(
            "https://woolworths-products-api.p.rapidapi.com/woolworths/product-search/?page=1&page_size=100&query={}",
            name
        ))
        .headers(headers)
        .header("x-rapidapi-host", "woolworths-products-api.p.rapidapi.com")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    // Debug print raw Woolworths response
    let woolworths_raw_json: serde_json::Value = woolworths_response.json().await.map_err(|e| e.to_string())?;
    println!("Woolworths Raw Response: {}", serde_json::to_string_pretty(&woolworths_raw_json).unwrap());
    
    // Convert to your struct
    // let woolworths_results = serde_json::from_value::<ColesApiResponse>(woolworths_raw_json).ok();

    // Construct the response
    Ok(Json(ProductSearchResponse {
        name,
        coles: Some(coles_raw_json),
        woolworths: Some(woolworths_raw_json),
    }))
}
