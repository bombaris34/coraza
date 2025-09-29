use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ActivationKeys::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ActivationKeys::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ActivationKeys::Key).string().not_null())
                    .col(ColumnDef::new(ActivationKeys::ProductId).uuid().not_null())
                    .col(
                        ColumnDef::new(ActivationKeys::DurationDays)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ActivationKeys::IsRedeemed)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ActivationKeys::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ActivationKeys::GeneratedBy).uuid())
                    .col(ColumnDef::new(ActivationKeys::RedeemedBy).uuid())
                    .col(ColumnDef::new(ActivationKeys::IsFree).boolean().not_null())
                    .col(
                        ColumnDef::new(ActivationKeys::PricePaid)
                            .double()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ActivationKeys::OrderId).uuid())
                    .col(ColumnDef::new(ActivationKeys::ReplacedBy).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-activation_keys-product_id")
                            .from(ActivationKeys::Table, ActivationKeys::ProductId)
                            .to(Products::Table, Products::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-activation_keys-generated_by")
                            .from(ActivationKeys::Table, ActivationKeys::GeneratedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-activation_keys-redeemed_by")
                            .from(ActivationKeys::Table, ActivationKeys::RedeemedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ActivationKeys::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ActivationKeys {
    Table,
    Id,
    Key,
    ProductId,
    DurationDays,
    IsRedeemed,
    CreatedAt,
    GeneratedBy,
    RedeemedBy,
    IsFree,
    PricePaid,
    OrderId,
    ReplacedBy,
}

#[derive(DeriveIden)]
enum Products {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
