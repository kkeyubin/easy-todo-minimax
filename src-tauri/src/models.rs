use serde::{Deserialize, Serialize};

/// 便签主表。type 字段在 SQL 里是关键字，所以 rename 一下。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sticky {
    pub id: i64,
    #[serde(rename = "type")]
    pub sticky_type: String,
    pub title: String,
    pub color: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub font_size: i32,
    pub pinned: i32,
    pub z_order: i32,
    pub content: String,
    pub image_path: Option<String>,
    pub link_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// 局部更新 patch。None = 不更新。
/// image_path / link_url 用双重 Option：外层 None=不动，内层 None=置空。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StickyPatch {
    #[serde(rename = "type")]
    pub sticky_type: Option<String>,
    pub title: Option<String>,
    pub color: Option<String>,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub font_size: Option<i32>,
    pub pinned: Option<i32>,
    pub z_order: Option<i32>,
    pub content: Option<String>,
    pub image_path: Option<Option<String>>,
    pub link_url: Option<Option<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub sticky_id: i64,
    pub text: String,
    pub done: i32,
    pub sort_order: i32,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TodoPatch {
    pub text: Option<String>,
    pub done: Option<i32>,
    pub sort_order: Option<i32>,
}
