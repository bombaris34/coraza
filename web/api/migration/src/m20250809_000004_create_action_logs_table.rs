use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ActionLogs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ActionLogs::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ActionLogs::UserId).uuid())
                    .col(ColumnDef::new(ActionLogs::ActionData).json().not_null())
                    .col(
                        ColumnDef::new(ActionLogs::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-action_logs-user_id")
                            .from(ActionLogs::Table, ActionLogs::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ActionLogs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ActionLogs {
    Table,
    Id,
    UserId,
    ActionData,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
