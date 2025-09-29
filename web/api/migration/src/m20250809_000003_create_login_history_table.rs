use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LoginHistory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LoginHistory::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LoginHistory::UserId).uuid().not_null())
                    .col(ColumnDef::new(LoginHistory::Success).boolean().not_null())
                    .col(ColumnDef::new(LoginHistory::IpAddress).string())
                    .col(
                        ColumnDef::new(LoginHistory::LoginTime)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-login_history-user_id")
                            .from(LoginHistory::Table, LoginHistory::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LoginHistory::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum LoginHistory {
    Table,
    Id,
    UserId,
    Success,
    IpAddress,
    LoginTime,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
