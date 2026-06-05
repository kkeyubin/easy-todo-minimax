use anyhow::{Context, Result};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;
use tauri::{AppHandle, Manager};

pub struct Storage {
    pub pool: SqlitePool,
}

impl Storage {
    pub fn new(app: &AppHandle) -> Result<Self> {
        let app_data_dir = app
            .path()
            .app_data_dir()
            .context("failed to resolve app data dir")?;
        std::fs::create_dir_all(&app_data_dir).context("create app data dir")?;
        let db_path = app_data_dir.join("sticky.db");
        let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

        let opts = SqliteConnectOptions::from_str(&db_url)
            .context("parse sqlite url")?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal);

        let pool = tauri::async_runtime::block_on(async {
            SqlitePoolOptions::new()
                .max_connections(5)
                .connect_with(opts)
                .await
        })
        .context("connect sqlite")?;

        Ok(Self { pool })
    }

    pub fn run_migrations(&self) -> Result<()> {
        tauri::async_runtime::block_on(async {
            let sql = include_str!("../migrations/0001_initial.sql");
            for stmt in sql.split(';') {
                let stmt = stmt.trim();
                if stmt.is_empty() {
                    continue;
                }
                sqlx::query(stmt)
                    .execute(&self.pool)
                    .await
                    .with_context(|| format!("apply migration stmt: {}", stmt.lines().next().unwrap_or("")))?;
            }
            Ok::<(), anyhow::Error>(())
        })
    }
}
