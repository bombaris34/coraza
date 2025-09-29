use actix_multipart::Multipart;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use futures::TryStreamExt;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::Deserialize;
use std::fs;
use std::io::Write;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::{
    action_logs::log_action,
    auth::AdminAuthorized,
    db::establish_connection,
    models::product::{
        ActiveModel as ProductActiveModel, Entity as Product, Model as ProductModel,
    },
};

#[derive(Deserialize)]
pub struct CreateProductRequest {
    name: String,
    description: String,
    price: f64,
    image_url: Option<String>,
    category: Option<String>,
    in_stock: bool,
}

#[derive(Deserialize)]
pub struct UpdateProductRequest {
    name: Option<String>,
    description: Option<String>,
    price: Option<f64>,
    image_url: Option<String>,
    category: Option<String>,
    in_stock: Option<bool>,
}

#[derive(Deserialize)]
pub struct LegacyProductRequest {
    identifier: String,
    name: String,
    price: f64,
    size: String,
    color: String,
    is_discounted: bool,
    discounted_price: Option<f64>,
}

#[get("/products")]
async fn list_products(_admin: AdminAuthorized) -> impl Responder {
    let db = establish_connection().await;
    let products = Product::find().all(&db).await.unwrap();
    HttpResponse::Ok().json(products)
}

#[get("/public")]
async fn list_products_public() -> impl Responder {
    let db = establish_connection().await;
    let products = Product::find().all(&db).await.unwrap();
    HttpResponse::Ok().json(products)
}

#[get("/public/{id}")]
async fn get_product_public(id: web::Path<Uuid>) -> impl Responder {
    let db = establish_connection().await;
    let product = Product::find_by_id(id.into_inner()).one(&db).await.unwrap();
    
    if let Some(product) = product {
        HttpResponse::Ok().json(product)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[post("/products")]
async fn create_product(admin: AdminAuthorized, req: web::Json<CreateProductRequest>) -> impl Responder {
    let db = establish_connection().await;
    let now = chrono::Utc::now().into();
    let new_product = ProductActiveModel {
        id: Set(Uuid::new_v4()),
        identifier: Set(format!("PROD-{}", Uuid::new_v4())), // Generate unique identifier
        name: Set(req.name.clone()),
        description: Set(req.description.clone()),
        price: Set(req.price),
        size: Set(String::new()), // Default empty for now
        color: Set(String::new()), // Default empty for now
        image_url: Set(req.image_url.clone()),
        category: Set(req.category.clone()),
        in_stock: Set(req.in_stock),
        is_discounted: Set(false), // Default
        discounted_price: Set(None), // Default
        frozen: Set(false),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let product: ProductModel = new_product.insert(&db).await.unwrap();
    log_action(
        Some(admin.id),
        serde_json::json!({"action":"create_product","id": product.id}),
    )
    .await;
    HttpResponse::Ok().json(product)
}

#[put("/products/{id}")]
async fn update_product(
    admin: AdminAuthorized,
    id: web::Path<Uuid>,
    req: web::Json<UpdateProductRequest>,
) -> impl Responder {
    let db = establish_connection().await;
    let product = Product::find_by_id(id.into_inner()).one(&db).await.unwrap();

    if let Some(product) = product {
        let mut product: ProductActiveModel = product.into();
        
        if let Some(name) = &req.name {
            product.name = Set(name.clone());
        }
        if let Some(description) = &req.description {
            product.description = Set(description.clone());
        }
        if let Some(price) = req.price {
            product.price = Set(price);
        }
        if let Some(image_url) = &req.image_url {
            product.image_url = Set(Some(image_url.clone()));
        }
        if let Some(category) = &req.category {
            product.category = Set(Some(category.clone()));
        }
        if let Some(in_stock) = req.in_stock {
            product.in_stock = Set(in_stock);
        }
        
        product.updated_at = Set(chrono::Utc::now().into());
        
        let updated_product: ProductModel = product.update(&db).await.unwrap();
        log_action(
            Some(admin.id),
            serde_json::json!({"action":"update_product","id": updated_product.id}),
        )
        .await;
        HttpResponse::Ok().json(updated_product)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[delete("/products/{id}")]
async fn delete_product(admin: AdminAuthorized, id: web::Path<Uuid>) -> impl Responder {
    let db = establish_connection().await;
    let pid = id.into_inner();
    let result = Product::delete_by_id(pid).exec(&db).await;

    match result {
        Ok(delete_result) => {
            if delete_result.rows_affected == 1 {
                log_action(
                    Some(admin.id),
                    serde_json::json!({"action":"delete_product","id": pid}),
                )
                .await;
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/products/{id}/freeze")]
async fn freeze_product(admin: AdminAuthorized, id: web::Path<Uuid>) -> impl Responder {
    let db = establish_connection().await;
    let product_id = id.into_inner();
    if let Some(product) = Product::find_by_id(product_id).one(&db).await.unwrap() {
        let mut product_model: ProductActiveModel = product.into();
        product_model.frozen = Set(true);
        product_model.update(&db).await.unwrap();

        log_action(
            Some(admin.id),
            serde_json::json!({"action":"freeze_product","id": product_id}),
        )
        .await;
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[post("/products/{id}/unfreeze")]
async fn unfreeze_product(admin: AdminAuthorized, id: web::Path<Uuid>) -> impl Responder {
    let db = establish_connection().await;
    let product_id = id.into_inner();
    if let Some(product) = Product::find_by_id(product_id).one(&db).await.unwrap() {
        let mut product_model: ProductActiveModel = product.into();
        product_model.frozen = Set(false);
        product_model.update(&db).await.unwrap();

        log_action(
            Some(admin.id),
            serde_json::json!({"action":"unfreeze_product","id": product_id}),
        )
        .await;
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[post("/products/upload-image")]
async fn upload_image(_admin: AdminAuthorized, mut payload: Multipart) -> impl Responder {
    // Create uploads directory if it doesn't exist
    fs::create_dir_all("uploads").unwrap_or_default();
    
    while let Some(mut field) = payload.try_next().await.unwrap() {
        let content_disposition = field.content_disposition();
        
        if let Some(filename) = content_disposition.get_filename() {
            // Generate unique filename
            let file_extension = std::path::Path::new(filename)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("jpg");
            let unique_filename = format!("{}.{}", Uuid::new_v4(), file_extension);
            let filepath = format!("uploads/{}", unique_filename);
            
            // Save file
            let mut f = web::block(move || std::fs::File::create(&filepath))
                .await
                .unwrap()
                .unwrap();
            
            while let Some(chunk) = field.try_next().await.unwrap() {
                f = web::block(move || f.write_all(&chunk).map(|_| f))
                    .await
                    .unwrap()
                    .unwrap();
            }
            
            let file_url = format!("/uploads/{}", unique_filename);
            return HttpResponse::Ok().json(serde_json::json!({
                "url": file_url
            }));
        }
    }
    
    HttpResponse::BadRequest().json(serde_json::json!({
        "error": "No file provided"
    }))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_products_public)
        .service(get_product_public);
}

pub fn init_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_products)
        .service(create_product)
        .service(update_product)
        .service(delete_product)
        .service(freeze_product)
        .service(unfreeze_product)
        .service(upload_image);
}
