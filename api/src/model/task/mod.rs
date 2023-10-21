use crate::context::Ctx;
use crate::model::ModelManager;
use crate::model::{ Result, Error };
use serde::{ Deserialize, Serialize };
use sqlx::FromRow;

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

impl TaskBmc {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, task_c: TaskForCreate) -> Result<i64> {
        let db = mm.db();
        let (id,) = sqlx
            ::query_as::<_, (i64,)>("INSERT INTO task (title) VALUES ($1) returning id")
            .bind(task_c.title)
            .fetch_one(db).await?;
        Ok(id)
    }

    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
        let db = mm.db();
        let task: Task = sqlx
            ::query_as("SELECT * FROM task WHERE id = $1")
            .bind(id)
            .fetch_optional(db).await?
            .ok_or(Error::EntityNotFound { entity: "task", id })?;
        Ok(task)
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
}

// endregion -- Unit tests
