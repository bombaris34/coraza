use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Products::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Products::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Products::Identifier).string().not_null())
                    .col(ColumnDef::new(Products::Name).string().not_null())
                    .col(ColumnDef::new(Products::Price).double().not_null())
                    .col(ColumnDef::new(Products::Size).string().not_null())
                    .col(ColumnDef::new(Products::Color).string().not_null())
                    .col(
                        ColumnDef::new(Products::IsDiscounted)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Products::DiscountedPrice).double())
                    .col(ColumnDef::new(Products::Frozen).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Products::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Products {
    Table,
    Id,
    Identifier,
    Name,
    Price,
    Size,
    Color,
    IsDiscounted,
    DiscountedPrice,
    Frozen,
}
