use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add new columns to match the frontend model
        manager
            .alter_table(
                Table::alter()
                    .table(Products::Table)
                    .add_column(ColumnDef::new(Products::Description).string().not_null().default(""))
                    .add_column(ColumnDef::new(Products::ImageUrl).string())
                    .add_column(ColumnDef::new(Products::Category).string())
                    .add_column(ColumnDef::new(Products::InStock).boolean().not_null().default(true))
                    .add_column(
                        ColumnDef::new(Products::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .add_column(
                        ColumnDef::new(Products::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Remove the added columns
        manager
            .alter_table(
                Table::alter()
                    .table(Products::Table)
                    .drop_column(Products::Description)
                    .drop_column(Products::ImageUrl)
                    .drop_column(Products::Category)
                    .drop_column(Products::InStock)
                    .drop_column(Products::CreatedAt)
                    .drop_column(Products::UpdatedAt)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Products {
    Table,
    Description,
    ImageUrl,
    Category,
    InStock,
    CreatedAt,
    UpdatedAt,
}