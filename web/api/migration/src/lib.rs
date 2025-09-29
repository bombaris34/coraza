pub use sea_orm_migration::prelude::*;

mod m20250809_000001_create_users_table;
mod m20250809_000002_create_products_table;
mod m20250809_000003_create_login_history_table;
mod m20250809_000004_create_action_logs_table;
mod m20250809_000005_create_activation_keys_table;
mod m20250809_000006_update_products_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250809_000001_create_users_table::Migration),
            Box::new(m20250809_000002_create_products_table::Migration),
            Box::new(m20250809_000003_create_login_history_table::Migration),
            Box::new(m20250809_000004_create_action_logs_table::Migration),
            Box::new(m20250809_000005_create_activation_keys_table::Migration),
            Box::new(m20250809_000006_update_products_table::Migration),
        ]
    }
}
