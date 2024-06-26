use sea_orm::{ ActiveModelTrait, EnumIter, Iterable, Set };
use sea_orm_migration::prelude::*;

use crate::database::entities::shop_item;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // prettier-ignore
        manager.create_table(
            Table::create()
                .table(ShopItem::Table)
                .if_not_exists()
                .col(ColumnDef::new(ShopItem::ItemId).integer().not_null().primary_key())
                .col(ColumnDef::new(ShopItem::ItemType).enumeration(Alias::new("type"), ShopItemType::iter()))
                .col(ColumnDef::new(ShopItem::CostType).integer().not_null())
                .col(ColumnDef::new(ShopItem::NormalPrice).integer().not_null())
                .col(ColumnDef::new(ShopItem::DiscountPrice).integer().not_null())
                .col(ColumnDef::new(ShopItem::Order).integer().not_null())
                .col(ColumnDef::new(ShopItem::BeginSaleTime).big_unsigned().not_null().default(0))
                .col(ColumnDef::new(ShopItem::DiscountBeginTime).big_unsigned().not_null().default(0))
                .col(ColumnDef::new(ShopItem::DiscountEndTime).big_unsigned().not_null().default(0))
                .to_owned()
        ).await?;

        let db = manager.get_connection();

        let characters = vec![
            shop_item::ActiveModel {
                item_id: Set(20090),
                item_type: Set(Some("Character".to_string())),
                cost_type: Set(2),
                normal_price: Set(1000),
                discount_price: Set(1000),
                order: Set(1),
                ..Default::default()
            },
            shop_item::ActiveModel {
                item_id: Set(40320),
                item_type: Set(Some("Character".to_string())),
                cost_type: Set(2),
                normal_price: Set(300),
                discount_price: Set(300),
                order: Set(0),
                ..Default::default()
            },
            shop_item::ActiveModel {
                item_id: Set(20060),
                item_type: Set(Some("Character".to_string())),
                cost_type: Set(2),
                normal_price: Set(300),
                discount_price: Set(300),
                order: Set(0),
                ..Default::default()
            },
            shop_item::ActiveModel {
                item_id: Set(40010),
                item_type: Set(Some("Character".to_string())),
                cost_type: Set(2),
                normal_price: Set(1888),
                discount_price: Set(1888),
                order: Set(2),
                ..Default::default()
            },
            shop_item::ActiveModel {
                item_id: Set(40090),
                item_type: Set(Some("Character".to_string())),
                cost_type: Set(2),
                normal_price: Set(300),
                discount_price: Set(300),
                order: Set(0),
                ..Default::default()
            },
            shop_item::ActiveModel {
                item_id: Set(20050),
                item_type: Set(Some("Character".to_string())),
                cost_type: Set(2),
                normal_price: Set(1000),
                discount_price: Set(1000),
                order: Set(1),
                ..Default::default()
            },
            shop_item::ActiveModel {
                item_id: Set(30040),
                item_type: Set(Some("Character".to_string())),
                cost_type: Set(2),
                normal_price: Set(1888),
                discount_price: Set(1888),
                order: Set(2),
                ..Default::default()
            },
            shop_item::ActiveModel {
                item_id: Set(40250),
                item_type: Set(Some("Character".to_string())),
                cost_type: Set(2),
                normal_price: Set(300),
                discount_price: Set(300),
                order: Set(0),
                ..Default::default()
            },
            shop_item::ActiveModel {
                item_id: Set(40150),
                item_type: Set(Some("Character".to_string())),
                cost_type: Set(2),
                normal_price: Set(1000),
                discount_price: Set(1000),
                order: Set(1),
                ..Default::default()
            },
            shop_item::ActiveModel {
                item_id: Set(20040),
                item_type: Set(Some("Character".to_string())),
                cost_type: Set(2),
                normal_price: Set(1000),
                discount_price: Set(1000),
                order: Set(1),
                ..Default::default()
            }
        ];

        for char in characters {
            char.insert(db).await?;
        }

        let songs = vec![
            shop_item::ActiveModel {
                item_id: Set(63204),
                item_type: Set(Some("Song".to_string())),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(0),
                ..Default::default()
            },
            shop_item::ActiveModel {
                item_id: Set(68008),
                item_type: Set(Some("Song".to_string())),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(2),
                ..Default::default()
            },
            shop_item::ActiveModel {
                item_id: Set(62005),
                item_type: Set(Some("Song".to_string())),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(0),
                ..Default::default()
            },
            shop_item::ActiveModel {
                item_id: Set(63103),
                item_type: Set(Some("Song".to_string())),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(0),
                ..Default::default()
            },
            shop_item::ActiveModel {
                item_id: Set(63123),
                item_type: Set(Some("Song".to_string())),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(0),
                ..Default::default()
            },
            shop_item::ActiveModel {
                item_id: Set(80002),
                item_type: Set(Some("Song".to_string())),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(2),
                ..Default::default()
            },
            shop_item::ActiveModel {
                item_id: Set(62006),
                item_type: Set(Some("Song".to_string())),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(0),
                ..Default::default()
            },
            shop_item::ActiveModel {
                item_id: Set(63122),
                item_type: Set(Some("Song".to_string())),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(0),
                ..Default::default()
            },
            shop_item::ActiveModel {
                item_id: Set(69008),
                item_type: Set(Some("Song".to_string())),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(2),
                ..Default::default()
            },
            shop_item::ActiveModel {
                item_id: Set(68108),
                item_type: Set(Some("Song".to_string())),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(0),
                ..Default::default()
            }
        ];

        for song in songs {
            song.insert(db).await?;
        }

        for theme_id in vec![2, 6] {
            (shop_item::ActiveModel {
                item_id: Set(theme_id),
                item_type: Set(Some("Theme".to_string())),
                cost_type: Set(2),
                normal_price: Set(1000),
                discount_price: Set(1000),
                order: Set(0),
                ..Default::default()
            }).insert(db).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ShopItem::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
pub enum ShopItem {
    Table,
    ItemId,
    ItemType,
    CostType,
    NormalPrice,
    DiscountPrice,
    Order,
    BeginSaleTime,
    DiscountBeginTime,
    DiscountEndTime,
}

#[derive(Iden, EnumIter)]
pub enum ShopItemType {
    Character,
    Song,
    Theme,
}
