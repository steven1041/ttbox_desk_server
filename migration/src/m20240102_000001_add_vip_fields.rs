use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 先添加允许NULL的字段
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(
                        ColumnDef::new(Users::IsVip)
                            .boolean()
                            .not_null()
                            .default(false)
                    )
                    .add_column(
                        ColumnDef::new(Users::VipStartTime)
                            .date_time()
                            .null()
                    )
                    .add_column(
                        ColumnDef::new(Users::VipEndTime)
                            .date_time()
                            .null()
                    )
                    .add_column(
                        ColumnDef::new(Users::VipLevel)
                            .integer()
                            .not_null()
                            .default(0)
                    )
                    .add_column(
                        ColumnDef::new(Users::CreatedAt)
                            .date_time()
                            .null()
                    )
                    .add_column(
                        ColumnDef::new(Users::UpdatedAt)
                            .date_time()
                            .null()
                    )
                    .to_owned(),
            )
            .await?;

        // 为现有数据设置默认时间值
        let update = Query::update()
            .table(Users::Table)
            .values([
                (Users::IsVip, false.into()),
                (Users::VipLevel, 0.into()),
                (Users::CreatedAt, Expr::current_timestamp().into()),
                (Users::UpdatedAt, Expr::current_timestamp().into()),
            ])
            .to_owned();

        manager.exec_stmt(update).await?;

        // 将时间字段改为NOT NULL
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .modify_column(
                        ColumnDef::new(Users::CreatedAt)
                            .date_time()
                            .not_null()
                    )
                    .modify_column(
                        ColumnDef::new(Users::UpdatedAt)
                            .date_time()
                            .not_null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::IsVip)
                    .drop_column(Users::VipStartTime)
                    .drop_column(Users::VipEndTime)
                    .drop_column(Users::VipLevel)
                    .drop_column(Users::CreatedAt)
                    .drop_column(Users::UpdatedAt)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Email,
    Password,
    IsVip,
    VipStartTime,
    VipEndTime,
    VipLevel,
    CreatedAt,
    UpdatedAt,
}
