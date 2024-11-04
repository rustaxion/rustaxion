use sea_orm::{ActiveModelTrait, Set};
use sea_orm_migration::{prelude::*, schema::*};

use crate::database;

use super::m002_player::Player;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Theme::Table)
                    .if_not_exists()
                    .col(pk_auto(Theme::Id))
                    .col(string(Theme::Name))
                    .col(string(Theme::Description))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Player::Table)
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_player-selected_theme_id")
                            .from_tbl(Player::Table)
                            .from_col(Player::SelectedThemeId)
                            .to_tbl(Theme::Table)
                            .to_col(Theme::Id),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        type ThemeModel = database::entities::theme::ActiveModel;

        let themes = vec![
            ThemeModel {
                id: Set(1),
                name: Set("猎户座".into()),
                description: Set("LowPoly".into()),
            },
            ThemeModel {
                id: Set(2),
                name: Set("AquaX".into()),
                description: Set("BluePolygon".into()),
            },
            ThemeModel {
                id: Set(3),
                name: Set("法尔空".into()),
                description: Set("BlackGold".into()),
            },
            ThemeModel {
                id: Set(4),
                name: Set("瓦尔基里".into()),
                description: Set("Red".into()),
            },
            ThemeModel {
                id: Set(5),
                name: Set("阿波罗".into()),
                description: Set("PolyNet".into()),
            },
            ThemeModel {
                id: Set(6),
                name: Set("沙漠之鹰".into()),
                description: Set("Green".into()),
            },
            ThemeModel {
                id: Set(7),
                name: Set("White".into()),
                description: Set("White".into()),
            },
            ThemeModel {
                id: Set(8),
                name: Set("TRION".into()),
                description: Set("White".into()),
            },
            ThemeModel {
                id: Set(9),
                name: Set("赤焰三角".into()),
                description: Set("RedWhite".into()),
            },
            ThemeModel {
                id: Set(10),
                name: Set("Pink".into()),
                description: Set("Pink".into()),
            },
            ThemeModel {
                id: Set(11),
                name: Set("WonderParade".into()),
                description: Set("Wonder Parade".into()),
            },
            ThemeModel {
                id: Set(12),
                name: Set("BiliBili".into()),
                description: Set("Bili".into()),
            },
            ThemeModel {
                id: Set(13),
                name: Set("Ilem".into()),
                description: Set("ilem".into()),
            },
            ThemeModel {
                id: Set(14),
                name: Set("巨蟹座".into()),
                description: Set("Cancer".into()),
            },
        ];

        for theme in themes {
            theme.insert(db).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Player::Table)
                    .drop_foreign_key(Alias::new("fk_player-selected_theme_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Theme::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Theme {
    Table,
    Id,
    Name,
    Description,
}
