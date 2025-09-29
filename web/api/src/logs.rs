use actix_web::{get, web, HttpResponse, Responder};
use sea_orm::{EntityTrait, QueryOrder};

use crate::{
    auth::AdminAuthorized,
    db::establish_connection,
    models::action_log::{Entity as ActionLog, Column as ActionLogColumn},
};

#[get("/logs")]
async fn list_logs(_admin: AdminAuthorized) -> impl Responder {
    let db = establish_connection().await;
    let logs = ActionLog::find()
        .order_by_desc(ActionLogColumn::CreatedAt)
        .all(&db)
        .await
        .unwrap_or_default();
    HttpResponse::Ok().json(logs)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_logs);
}
