use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "sys_menu")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub parent_id: Option<i64>,
    pub path: String,
    pub name: String,
    pub component: Option<String>,
    pub title: String,
    pub icon: Option<String>,
    pub sort: i32,
    pub r#type: String,       // 'menu', 'button'
    pub meta: Option<String>, // JSON 字符串，存储 RouteMeta
    pub status: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "Entity", from = "Column::ParentId", to = "Column::Id")]
    Parent,
}

impl ActiveModelBehavior for ActiveModel {}
