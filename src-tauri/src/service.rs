use crate::models::{Sticky, StickyPatch, Todo, TodoPatch};
use crate::storage::Storage;
use anyhow::Result;
use chrono::Utc;
use sqlx::FromRow;

pub struct StickiesService {
    storage: Storage,
}

impl StickiesService {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    /// W1 阶段：阻塞版本给 setup hook 用（DB 还没建连接池时序问题）。
    pub fn list_sync(&self) -> Result<Vec<Sticky>> {
        tauri::async_runtime::block_on(async { self.list().await })
    }

    pub async fn list(&self) -> Result<Vec<Sticky>> {
        let rows = sqlx::query_as::<_, StickyRow>(
            "SELECT id, type AS sticky_type, title, color, x, y, width, height, \
                    font_size, pinned, z_order, content, image_path, link_url, \
                    created_at, updated_at \
             FROM stickies ORDER BY z_order DESC, id ASC",
        )
        .fetch_all(&self.storage.pool)
        .await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn get(&self, id: i64) -> Result<Sticky> {
        let row = sqlx::query_as::<_, StickyRow>(
            "SELECT id, type AS sticky_type, title, color, x, y, width, height, \
                    font_size, pinned, z_order, content, image_path, link_url, \
                    created_at, updated_at \
             FROM stickies WHERE id = ?",
        )
        .bind(id)
        .fetch_one(&self.storage.pool)
        .await?;
        Ok(row.into())
    }

    pub async fn create(&self, sticky_type: String, x: i32, y: i32) -> Result<Sticky> {
        let now = Utc::now().to_rfc3339();
        let result = sqlx::query(
            "INSERT INTO stickies (type, x, y, created_at, updated_at) \
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&sticky_type)
        .bind(x)
        .bind(y)
        .bind(&now)
        .bind(&now)
        .execute(&self.storage.pool)
        .await?;
        let id = result.last_insert_rowid();
        self.get(id).await
    }

    pub async fn update(&self, id: i64, patch: StickyPatch) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        let mut sql = String::from("UPDATE stickies SET updated_at = ?");
        if patch.sticky_type.is_some() {
            sql.push_str(", type = ?");
        }
        if patch.title.is_some() {
            sql.push_str(", title = ?");
        }
        if patch.color.is_some() {
            sql.push_str(", color = ?");
        }
        if patch.x.is_some() {
            sql.push_str(", x = ?");
        }
        if patch.y.is_some() {
            sql.push_str(", y = ?");
        }
        if patch.width.is_some() {
            sql.push_str(", width = ?");
        }
        if patch.height.is_some() {
            sql.push_str(", height = ?");
        }
        if patch.font_size.is_some() {
            sql.push_str(", font_size = ?");
        }
        if patch.pinned.is_some() {
            sql.push_str(", pinned = ?");
        }
        if patch.z_order.is_some() {
            sql.push_str(", z_order = ?");
        }
        if patch.content.is_some() {
            sql.push_str(", content = ?");
        }
        if patch.image_path.is_some() {
            sql.push_str(", image_path = ?");
        }
        if patch.link_url.is_some() {
            sql.push_str(", link_url = ?");
        }
        sql.push_str(" WHERE id = ?");

        let mut q = sqlx::query(&sql).bind(now);
        if let Some(v) = &patch.sticky_type {
            q = q.bind(v);
        }
        if let Some(v) = &patch.title {
            q = q.bind(v);
        }
        if let Some(v) = &patch.color {
            q = q.bind(v);
        }
        if let Some(v) = patch.x {
            q = q.bind(v);
        }
        if let Some(v) = patch.y {
            q = q.bind(v);
        }
        if let Some(v) = patch.width {
            q = q.bind(v);
        }
        if let Some(v) = patch.height {
            q = q.bind(v);
        }
        if let Some(v) = patch.font_size {
            q = q.bind(v);
        }
        if let Some(v) = patch.pinned {
            q = q.bind(v);
        }
        if let Some(v) = patch.z_order {
            q = q.bind(v);
        }
        if let Some(v) = &patch.content {
            q = q.bind(v);
        }
        if let Some(v) = &patch.image_path {
            q = q.bind(v);
        }
        if let Some(v) = &patch.link_url {
            q = q.bind(v);
        }
        q = q.bind(id);
        q.execute(&self.storage.pool).await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM stickies WHERE id = ?")
            .bind(id)
            .execute(&self.storage.pool)
            .await?;
        Ok(())
    }

    // ---- Todos ----

    pub async fn list_todos(&self, sticky_id: i64) -> Result<Vec<Todo>> {
        let rows = sqlx::query_as::<_, TodoRow>(
            "SELECT id, sticky_id, text, done, sort_order, created_at \
             FROM todos WHERE sticky_id = ? ORDER BY sort_order ASC, id ASC",
        )
        .bind(sticky_id)
        .fetch_all(&self.storage.pool)
        .await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn add_todo(&self, sticky_id: i64, text: String) -> Result<Todo> {
        let now = Utc::now().to_rfc3339();
        let max_order: Option<i32> = sqlx::query_scalar(
            "SELECT MAX(sort_order) FROM todos WHERE sticky_id = ?",
        )
        .bind(sticky_id)
        .fetch_one(&self.storage.pool)
        .await?;
        let next_order = max_order.unwrap_or(0) + 1;

        let result = sqlx::query(
            "INSERT INTO todos (sticky_id, text, sort_order, created_at) \
             VALUES (?, ?, ?, ?)",
        )
        .bind(sticky_id)
        .bind(&text)
        .bind(next_order)
        .bind(&now)
        .execute(&self.storage.pool)
        .await?;
        let id = result.last_insert_rowid();
        let row = sqlx::query_as::<_, TodoRow>("SELECT * FROM todos WHERE id = ?")
            .bind(id)
            .fetch_one(&self.storage.pool)
            .await?;
        Ok(row.into())
    }

    pub async fn update_todo(&self, id: i64, patch: TodoPatch) -> Result<()> {
        let mut sql = String::from("UPDATE todos SET");
        let mut first = true;
        if patch.text.is_some() {
            sql.push_str(" text = ?");
            first = false;
        }
        if patch.done.is_some() {
            if !first {
                sql.push_str(",");
            }
            sql.push_str(" done = ?");
            first = false;
        }
        if patch.sort_order.is_some() {
            if !first {
                sql.push_str(",");
            }
            sql.push_str(" sort_order = ?");
        }
        sql.push_str(" WHERE id = ?");

        let mut q = sqlx::query(&sql);
        if let Some(v) = &patch.text {
            q = q.bind(v);
        }
        if let Some(v) = patch.done {
            q = q.bind(v);
        }
        if let Some(v) = patch.sort_order {
            q = q.bind(v);
        }
        q = q.bind(id);
        q.execute(&self.storage.pool).await?;
        Ok(())
    }

    pub async fn delete_todo(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(&self.storage.pool)
            .await?;
        Ok(())
    }
}

#[derive(FromRow)]
struct StickyRow {
    id: i64,
    sticky_type: String,
    title: String,
    color: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    font_size: i32,
    pinned: i32,
    z_order: i32,
    content: String,
    image_path: Option<String>,
    link_url: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<StickyRow> for Sticky {
    fn from(r: StickyRow) -> Self {
        Sticky {
            id: r.id,
            sticky_type: r.sticky_type,
            title: r.title,
            color: r.color,
            x: r.x,
            y: r.y,
            width: r.width,
            height: r.height,
            font_size: r.font_size,
            pinned: r.pinned,
            z_order: r.z_order,
            content: r.content,
            image_path: r.image_path,
            link_url: r.link_url,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(FromRow)]
struct TodoRow {
    id: i64,
    sticky_id: i64,
    text: String,
    done: i32,
    sort_order: i32,
    created_at: String,
}

impl From<TodoRow> for Todo {
    fn from(r: TodoRow) -> Self {
        Todo {
            id: r.id,
            sticky_id: r.sticky_id,
            text: r.text,
            done: r.done,
            sort_order: r.sort_order,
            created_at: r.created_at,
        }
    }
}
