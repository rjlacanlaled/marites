use crate::context::Ctx;
use crate::model::ModelManager;
use crate::model::{ Result, Error };
use serde::{ Deserialize, Serialize };
use sqlx::FromRow;

use super::base::{ DbBmc, self };

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
}

pub struct TaskBmc;

impl DbBmc for TaskBmc {
    const TABLE: &'static str = "task";
}

impl TaskBmc {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, task_c: TaskForCreate) -> Result<i64> {
        let db = mm.db();
        let (id,) = sqlx
            ::query_as::<_, (i64,)>("INSERT INTO task (title) VALUES ($1) returning id")
            .bind(task_c.title)
            .fetch_one(db).await?;
        Ok(id)
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<u64> {
        let db = mm.db();
        let count = sqlx
            ::query("DELETE FROM task WHERE id = $1")
            .bind(id)
            .execute(db).await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound { entity: "task", id });
        }
        Ok(count)
    }

    pub async fn list(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
        let db = mm.db();
        let tasks = sqlx::query_as("SELECT * FROM task").fetch_all(db).await?;
        Ok(tasks)
    }
}

// region -- Unit tests

#[cfg(test)]
mod tests {
    use crate::_dev_utils;

    use super::*;
    use anyhow::Result;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // Setup & fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "text_create_ok title";

        // Execute
        let task_c = TaskForCreate {
            title: fx_title.to_string(),
        };
        let id = TaskBmc::create(&ctx, &mm, task_c).await?;

        // Check
        let task = TaskBmc::get(&ctx, &mm, id).await?;
        assert_eq!(task.title, fx_title);

        // -- Clean
        let count = TaskBmc::delete(&ctx, &mm, id).await?;
        assert_eq!(count, 1, "Should delete 1 row");

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100;

        // Exec
        let task = TaskBmc::get(&ctx, &mm, fx_id).await;
        assert!(matches!(task, Err(Error::EntityNotFound { entity: "task", id: 100 })));

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_ok() -> Result<()> {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_titles = &["test_list_ok 01", "test_list_ok 02", "test_list_ok 03"];
        _dev_utils::seed_tasks(&ctx, &mm, fx_titles).await?;

        // Exec
        let tasks = TaskBmc::list(&ctx, &mm).await?;

        // Check
        let tasks: Vec<Task> = tasks
            .into_iter()
            .filter(|t| t.title.starts_with("test_list_ok"))
            .collect();

        assert_eq!(tasks.len(), 3, "Should have 3 tasks");

        // Clean
        for task in tasks {
            TaskBmc::delete(&ctx, &mm, task.id).await?;
        }

        Ok(())
    }
}

// endregion -- Unit tests
