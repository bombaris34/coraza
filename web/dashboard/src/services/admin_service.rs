use super::BASE_URL;
use crate::models::product::{CreateProduct, UpdateProduct};
use crate::models::{Product, User};
use reqwest::Client;
use std::error::Error;
use uuid::Uuid;

pub async fn get_products(token: &str) -> Result<Vec<Product>, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/admin/products", BASE_URL);
    let products = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;
    Ok(products)
}

pub async fn create_product(
    token: &str,
    product: CreateProduct,
) -> Result<Product, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/admin/products", BASE_URL);
    let product = client
        .post(&url)
        .bearer_auth(token)
        .json(&product)
        .send()
        .await?
        .json()
        .await?;
    Ok(product)
}

pub async fn update_product(
    token: &str,
    id: Uuid,
    product: UpdateProduct,
) -> Result<Product, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/admin/products/{}", BASE_URL, id);
    let product = client
        .put(&url)
        .bearer_auth(token)
        .json(&product)
        .send()
        .await?
        .json()
        .await?;
    Ok(product)
}

pub async fn delete_product(token: &str, id: Uuid) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/admin/products/{}", BASE_URL, id);
    client.delete(&url).bearer_auth(token).send().await?;
    Ok(())
}

pub async fn get_logs(token: &str) -> Result<Vec<serde_json::Value>, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/admin/logs", BASE_URL);
    let res = client.get(&url).bearer_auth(token).send().await?;
    Ok(res.json().await?)
}

pub async fn get_users(token: &str) -> Result<Vec<User>, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/admin/users", BASE_URL);
    let res = client.get(&url).bearer_auth(token).send().await?;
    Ok(res.json().await?)
}

pub async fn upload_image(
    token: &str,
    file_data: Vec<u8>,
    filename: &str,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/admin/products/upload-image", BASE_URL);

    let form = reqwest::multipart::Form::new().part(
        "image",
        reqwest::multipart::Part::bytes(file_data)
            .file_name(filename.to_string())
            .mime_str("image/*")?,
    );

    let response = client
        .post(&url)
        .bearer_auth(token)
        .multipart(form)
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;

    if let Some(url) = json.get("url").and_then(|u| u.as_str()) {
        Ok(url.to_string())
    } else {
        Err("Failed to get image URL from response".into())
    }
}
