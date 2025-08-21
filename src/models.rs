use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub price_xof: i32,
    #[serde(rename = "type")]
    pub product_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductData {
    pub products: Vec<Product>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentRequestItem {
    pub uuid: String,
    pub qty: i32,
}

#[derive(Debug, Deserialize)]
pub struct PaymentRequest {
    pub products: Vec<PaymentRequestItem>,
}

#[derive(Debug, Serialize)]
pub struct PayTechRequest {
    pub item_name: String,
    pub item_price: i32,
    pub ref_command: String,
    pub command_name: String,
    pub currency: String,
    pub env: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipn_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PayTechResponse {
    pub success: i32,
    pub token: Option<String>,
    pub redirect_url: Option<String>,
    pub message: Option<String>,
    // Additional possible fields from PayTech API
    pub payment_url: Option<String>,
    pub payment_token: Option<String>,
    pub url: Option<String>,
    pub redirect: Option<String>,
    // Handle potential error fields (can be string or array)
    pub error: Option<serde_json::Value>,
    pub errors: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct PayTechIPNCallback {
    pub type_event: String,
    pub ref_command: String,
    pub item_name: String,
    pub item_price: String,
    pub final_item_price: String,
    pub currency: String,
    pub custom_field: String,
    pub payment_method: String,
    pub api_key_sha256: String,
    pub api_secret_sha256: String,
    pub hmac_compute: String,
    pub client_phone: String,
    pub client_email: String,
}

#[derive(Debug, Serialize)]
pub struct OrderDetail {
    pub uuid: String,
    pub name: String,
    pub qty: i32,
    pub unit_price: i32,
    pub subtotal: i32,
}

#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub order_details: Vec<OrderDetail>,
    pub total_price: i32,
    pub currency: String,
    pub payment_url: String,
    pub payment_token: String,
    pub ref_command: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct CallbackResponse {
    pub status: String,
    pub event: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_command: Option<String>,
}