use actix_web::{get, HttpResponse, Responder};
use chrono::{Duration, Utc};
use sea_orm::{entity::*, query::*};
use serde::Serialize;

use crate::{
    auth::AdminAuthorized,
    db::establish_connection,
    models::user,
};

#[derive(Serialize)]
pub struct RegistrationStats {
    date: String,
    count: i64,
}

#[get("/registrations")]
async fn get_registration_stats(_admin: AdminAuthorized) -> impl Responder {
    let db = establish_connection().await;
    let thirty_days_ago = Utc::now() - Duration::days(30);

    let data = user::Entity::find()
        .filter(user::Column::CreatedAt.gte(thirty_days_ago))
        .all(&db)
        .await
        .unwrap();

    let mut stats: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
    for user in data {
        let date = user.created_at.format("%Y-%m-%d").to_string();
        *stats.entry(date).or_insert(0) += 1;
    }

    let mut stats_vec: Vec<RegistrationStats> = stats
        .into_iter()
        .map(|(date, count)| RegistrationStats { date, count })
        .collect();

    stats_vec.sort_by(|a, b| a.date.cmp(&b.date));

    HttpResponse::Ok().json(stats_vec)
}

