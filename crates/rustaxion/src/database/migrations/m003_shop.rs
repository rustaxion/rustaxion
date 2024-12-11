use extension::postgres::Type;
use sea_orm::{ActiveModelTrait, EnumIter, Iterable, Set};
use sea_orm_migration::{prelude::*, schema::*};

use crate::database::{self, entities::shop_item};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("shop_item_type"))
                    .values(ShopItemType::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ShopItem::Table)
                    .if_not_exists()
                    .col(pk_auto(ShopItem::ItemId))
                    .col(enumeration(
                        ShopItem::ItemType,
                        Alias::new("shop_item_type"),
                        ShopItemType::iter(),
                    ))
                    .col(integer(ShopItem::CostType))
                    .col(integer(ShopItem::NormalPrice))
                    .col(integer(ShopItem::DiscountPrice))
                    .col(integer(ShopItem::Order))
                    .col(timestamp_with_time_zone(ShopItem::BeginSaleTime))
                    .col(timestamp_with_time_zone(ShopItem::DiscountBeginTime))
                    .col(timestamp_with_time_zone(ShopItem::DiscountEndTime))
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        let time = chrono::DateTime::from_timestamp(1730681814, 0)
            .unwrap()
            .fixed_offset();

        let end_time = chrono::DateTime::from_timestamp(1730681814 + 31536000, 0)
            .unwrap()
            .fixed_offset();

        let characters = vec![
            shop_item::ActiveModel {
                item_id: Set(20090),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Character),
                cost_type: Set(2),
                normal_price: Set(1000),
                discount_price: Set(1000),
                order: Set(1),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
            shop_item::ActiveModel {
                item_id: Set(40320),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Character),
                cost_type: Set(2),
                normal_price: Set(300),
                discount_price: Set(300),
                order: Set(0),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
            shop_item::ActiveModel {
                item_id: Set(20060),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Character),
                cost_type: Set(2),
                normal_price: Set(300),
                discount_price: Set(300),
                order: Set(0),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
            shop_item::ActiveModel {
                item_id: Set(40010),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Character),
                cost_type: Set(2),
                normal_price: Set(1888),
                discount_price: Set(1888),
                order: Set(2),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
            shop_item::ActiveModel {
                item_id: Set(40090),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Character),
                cost_type: Set(2),
                normal_price: Set(300),
                discount_price: Set(300),
                order: Set(0),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
            shop_item::ActiveModel {
                item_id: Set(20050),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Character),
                cost_type: Set(2),
                normal_price: Set(1000),
                discount_price: Set(1000),
                order: Set(1),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
            shop_item::ActiveModel {
                item_id: Set(30040),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Character),
                cost_type: Set(2),
                normal_price: Set(1888),
                discount_price: Set(1888),
                order: Set(2),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
            shop_item::ActiveModel {
                item_id: Set(40250),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Character),
                cost_type: Set(2),
                normal_price: Set(300),
                discount_price: Set(300),
                order: Set(0),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
            shop_item::ActiveModel {
                item_id: Set(40150),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Character),
                cost_type: Set(2),
                normal_price: Set(1000),
                discount_price: Set(1000),
                order: Set(1),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
            shop_item::ActiveModel {
                item_id: Set(20040),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Character),
                cost_type: Set(2),
                normal_price: Set(1000),
                discount_price: Set(1000),
                order: Set(1),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
        ];

        for char in characters {
            char.insert(db).await?;
        }

        let songs = vec![
            shop_item::ActiveModel {
                item_id: Set(63204),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Song),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(0),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
            shop_item::ActiveModel {
                item_id: Set(68008),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Song),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(2),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
            shop_item::ActiveModel {
                item_id: Set(62005),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Song),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(0),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
            shop_item::ActiveModel {
                item_id: Set(63103),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Song),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(0),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
            shop_item::ActiveModel {
                item_id: Set(63123),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Song),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(0),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
            shop_item::ActiveModel {
                item_id: Set(80002),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Song),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(2),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
            shop_item::ActiveModel {
                item_id: Set(62006),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Song),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(0),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
            shop_item::ActiveModel {
                item_id: Set(63122),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Song),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(0),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
            shop_item::ActiveModel {
                item_id: Set(69008),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Song),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(2),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
            shop_item::ActiveModel {
                item_id: Set(68108),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Song),
                cost_type: Set(2),
                normal_price: Set(200),
                discount_price: Set(200),
                order: Set(0),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            },
        ];

        for song in songs {
            song.insert(db).await?;
        }

        for theme_id in vec![2, 6] {
            (shop_item::ActiveModel {
                item_id: Set(theme_id),
                item_type: Set(database::entities::sea_orm_active_enums::ShopItemType::Theme),
                cost_type: Set(2),
                normal_price: Set(1000),
                discount_price: Set(1000),
                order: Set(0),
                begin_sale_time: Set(time),
                discount_begin_time: Set(time),
                discount_end_time: Set(end_time),
            })
            .insert(db)
            .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ShopItem::Table).to_owned())
            .await
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
