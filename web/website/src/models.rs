use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// API Product model (matches the API response)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiProduct {
    pub id: Uuid,
    pub identifier: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub size: String,
    pub color: String,
    pub image_url: Option<String>,
    pub category: Option<String>,
    pub in_stock: bool,
    pub is_discounted: bool,
    pub discounted_price: Option<f64>,
    pub frozen: bool,
}

impl From<ApiProduct> for Product {
    fn from(api_product: ApiProduct) -> Self {
        Product {
            id: api_product.id,
            name: api_product.name,
            description: api_product.description, // Use the actual description from API
            price: if api_product.is_discounted {
                api_product.discounted_price.unwrap_or(api_product.price)
            } else {
                api_product.price
            },
            image_url: api_product.image_url, // Use the actual image URL from API
            category: api_product.category.or(Some("Clothing".to_string())), // Use API category or default
            in_stock: api_product.in_stock && !api_product.frozen, // Respect both flags
            created_at: Utc::now(), // Placeholder - API doesn't provide timestamps yet
            updated_at: Utc::now(), // Placeholder - API doesn't provide timestamps yet
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub image_url: Option<String>,
    pub category: Option<String>,
    pub in_stock: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Category {
    pub name: String,
    pub slug: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFilter {
    pub category: Option<String>,
    pub search: Option<String>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub in_stock_only: bool,
}
