use crate::models::{Product, ProductData};
use anyhow::Result;
use std::fs;

pub async fn load_products(filename: &str) -> Result<ProductData> {
    let contents = fs::read_to_string(filename)?;
    let product_data: ProductData = serde_json::from_str(&contents)?;
    Ok(product_data)
}

pub fn filter_products_by_type(products: &[Product], allowed_type: Option<&str>) -> Vec<Product> {
    match allowed_type {
        Some(product_type) => products
            .iter()
            .filter(|product| product.product_type == product_type)
            .cloned()
            .collect(),
        None => products.to_vec(),
    }
}