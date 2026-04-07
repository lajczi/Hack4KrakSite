use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(UserPersonalInfo::Table)
                    .add_column(ColumnDef::new(UserPersonalInfo::CtfExperience).string().null())
                    .add_column(ColumnDef::new(UserPersonalInfo::CtfInterestAreas).json().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(UserPersonalInfo::Table)
                    .drop_column(UserPersonalInfo::CtfExperience)
                    .drop_column(UserPersonalInfo::CtfInterestAreas)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum UserPersonalInfo {
    Table,
    CtfExperience,
    CtfInterestAreas,
}
