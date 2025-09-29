use crate::models::{ApiProduct, Category, Product, ProductFilter};
use reqwest::Client;
use std::error::Error;
use uuid::Uuid;

pub const API_BASE_URL: &str = "https://api.coraza.clothing";

pub async fn get_products() -> Result<Vec<Product>, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/products/public", API_BASE_URL);
    let api_products: Vec<ApiProduct> = client.get(&url).send().await?.json().await?;
    let products: Vec<Product> = api_products.into_iter().map(|p| p.into()).collect();
    Ok(products)
}

pub async fn get_product_by_id(id: Uuid) -> Result<Product, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("{}/products/public/{}", API_BASE_URL, id);
    let api_product: ApiProduct = client.get(&url).send().await?.json().await?;
    Ok(api_product.into())
}

pub async fn get_featured_products() -> Result<Vec<Product>, Box<dyn Error>> {
    // For now, just get all products and take the first few
    let products = get_products().await?;
    Ok(products.into_iter().take(6).collect())
}

pub async fn get_categories() -> Result<Vec<Category>, Box<dyn Error>> {
    let products = get_products().await?;
    let mut category_counts = std::collections::HashMap::new();

    for product in products {
        if let Some(category) = product.category {
            *category_counts.entry(category).or_insert(0) += 1;
        }
    }

    let categories: Vec<Category> = category_counts
        .into_iter()
        .map(|(name, count)| Category {
            slug: name.to_lowercase().replace(' ', "-"),
            name,
            count,
        })
        .collect();

    Ok(categories)
}

pub async fn search_products(filter: ProductFilter) -> Result<Vec<Product>, Box<dyn Error>> {
    let mut products = get_products().await?;

    // Apply filters
    products.retain(|product| {
        // Category filter
        if let Some(ref category) = filter.category {
            if product.category.as_ref() != Some(category) {
                return false;
            }
        }

        // Search filter
        if let Some(ref search) = filter.search {
            let search_lower = search.to_lowercase();
            if !product.name.to_lowercase().contains(&search_lower)
                && !product.description.to_lowercase().contains(&search_lower)
            {
                return false;
            }
        }

        // Price filters
        if let Some(min_price) = filter.min_price {
            if product.price < min_price {
                return false;
            }
        }

        if let Some(max_price) = filter.max_price {
            if product.price > max_price {
                return false;
            }
        }

        // Stock filter
        if filter.in_stock_only && !product.in_stock {
            return false;
        }

        true
    });

    Ok(products)
}
