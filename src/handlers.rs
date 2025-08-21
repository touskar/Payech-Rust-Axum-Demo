use crate::config::Config;
use crate::models::*;
use crate::products::{filter_products_by_type, load_products};
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::Json,
};
use hmac::{Hmac, Mac};
use serde_json::json;
use sha2::Sha256;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

pub async fn root() -> Json<serde_json::Value> {
    Json(json!({
        "message": "Welcome to PayTech Rust Axum Server!"
    }))
}

pub async fn list_products(
    Form(params): Form<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let allowed_type = params.get("allowed_type").map(|s| s.as_str());

    let products = load_products("products.json").await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
    })?;

    let filtered_products = filter_products_by_type(&products.products, allowed_type);

    Ok(Json(json!({
        "products": filtered_products,
        "count": filtered_products.len()
    })))
}

pub async fn request_payment(
    State(config): State<Arc<Config>>,
    Json(payment_req): Json<PaymentRequest>,
) -> Result<Json<PaymentResponse>, (StatusCode, Json<ErrorResponse>)> {
    let products = load_products("products.json").await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
    })?;

    let mut product_map = HashMap::new();
    for product in &products.products {
        product_map.insert(product.uuid.clone(), product.clone());
    }

    let mut total_price = 0;
    let mut order_details = Vec::new();

    for item in &payment_req.products {
        if item.qty <= 0 {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Invalid quantity for product: {}", item.uuid),
                }),
            ));
        }

        let product = product_map.get(&item.uuid).ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Product not found: {}", item.uuid),
                }),
            )
        })?;

        let item_total = product.price_xof * item.qty;
        total_price += item_total;

        order_details.push(OrderDetail {
            uuid: item.uuid.clone(),
            name: product.name.clone(),
            qty: item.qty,
            unit_price: product.price_xof,
            subtotal: item_total,
        });
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let ref_command = format!("CMD_{}", timestamp);
    let command_name = if order_details.is_empty() {
        "Commande en ligne".to_string()
    } else {
        format!("Achat de {} article(s)", order_details.len())
    };

    let paytech_req = PayTechRequest {
        item_name: command_name.clone(),
        item_price: total_price,
        ref_command: ref_command.clone(),
        command_name,
        currency: "XOF".to_string(),
        env: config.paytech_env.clone(),
        ipn_url: Some(config.paytech_ipn_url.clone()),
        success_url: Some(config.paytech_success_url.clone()),
        cancel_url: Some(config.paytech_cancel_url.clone()),
    };

    println!("Sending PayTech request: {}", serde_json::to_string_pretty(&paytech_req).unwrap_or_default());

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/payment/request-payment", config.paytech_base_url))
        .header("Content-Type", "application/json")
        .header("API_KEY", &config.paytech_api_key)
        .header("API_SECRET", &config.paytech_api_secret)
        .json(&paytech_req)
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to contact payment gateway: {}", e),
                }),
            )
        })?;

    let status = response.status();
    println!("PayTech API HTTP Status: {}", status);

    // Get response text first for debugging
    let response_text = response.text().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to read payment response: {}", e),
            }),
        )
    })?;

    println!("PayTech API Response: {}", response_text);

    // Try to parse as generic JSON first to handle any structure
    let json_response: serde_json::Value = serde_json::from_str(&response_text).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to parse payment response as JSON: {}. Response was: {}", e, response_text),
            }),
        )
    })?;

    println!("Parsed JSON structure: {}", serde_json::to_string_pretty(&json_response).unwrap_or_default());

    let paytech_resp: PayTechResponse = serde_json::from_str(&response_text).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to parse payment response: {}. Response was: {}", e, response_text),
            }),
        )
    })?;

    if paytech_resp.success != 1 {
        let mut error_msg = paytech_resp.message.clone().unwrap_or_default();
        
        // Extract error message from error field (can be string or array)
        if let Some(error_value) = &paytech_resp.error {
            match error_value {
                serde_json::Value::String(s) => {
                    if !error_msg.is_empty() {
                        error_msg.push_str(": ");
                    }
                    error_msg.push_str(s);
                },
                serde_json::Value::Array(arr) => {
                    let error_list: Vec<String> = arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(|s| s.to_string())
                        .collect();
                    if !error_list.is_empty() {
                        if !error_msg.is_empty() {
                            error_msg.push_str(": ");
                        }
                        error_msg.push_str(&error_list.join(", "));
                    }
                },
                _ => {
                    if !error_msg.is_empty() {
                        error_msg.push_str(": ");
                    }
                    error_msg.push_str(&error_value.to_string());
                }
            }
        }
        
        if error_msg.is_empty() {
            error_msg = "Unknown payment error".to_string();
        }
        
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Payment request failed: {}", error_msg),
            }),
        ));
    }

    // Extract payment URL and token, trying different possible field names
    let payment_url = paytech_resp.redirect_url
        .or(paytech_resp.payment_url)
        .or(paytech_resp.url)
        .or(paytech_resp.redirect)
        .unwrap_or_else(|| "".to_string());
    
    let payment_token = paytech_resp.token
        .or(paytech_resp.payment_token)
        .unwrap_or_else(|| "".to_string());

    Ok(Json(PaymentResponse {
        order_details,
        total_price,
        currency: "XOF".to_string(),
        payment_url,
        payment_token,
        ref_command,
    }))
}

pub async fn paytech_callback(
    State(config): State<Arc<Config>>,
    Json(callback): Json<PayTechIPNCallback>,
) -> Result<Json<CallbackResponse>, (StatusCode, Json<ErrorResponse>)> {
    let message = format!(
        "{}|{}|{}",
        callback.final_item_price, callback.ref_command, config.paytech_api_key
    );

    let mut mac = HmacSha256::new_from_slice(config.paytech_api_secret.as_bytes()).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("HMAC initialization failed: {}", e),
            }),
        )
    })?;

    mac.update(message.as_bytes());
    let expected_hmac = hex::encode(mac.finalize().into_bytes());

    if expected_hmac != callback.hmac_compute {
        println!(
            "HMAC verification failed. Expected: {}, Got: {}",
            expected_hmac, callback.hmac_compute
        );
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid signature".to_string(),
            }),
        ));
    }

    if callback.type_event != "sale_complete" {
        println!(
            "Ignoring event type: {} for order {}",
            callback.type_event, callback.ref_command
        );
        return Ok(Json(CallbackResponse {
            status: "ignored".to_string(),
            event: callback.type_event,
            ref_command: None,
        }));
    }

    println!(
        "Payment successful for order {} - Amount: {}",
        callback.ref_command, callback.final_item_price
    );

    Ok(Json(CallbackResponse {
        status: "received".to_string(),
        event: callback.type_event,
        ref_command: Some(callback.ref_command),
    }))
}