mod dev_db;

use tokio::sync::OnceCell;
use tracing::info;

use crate::model::ModelManager;

/// Initialize environment for local development
/// For early development, it will be called in main()
pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:12} - {}", "DEV ONLY", "Initializing dev environment");

        dev_db::init_dev_db().await.unwrap();
    }).await;
}

pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();

    let mm = INIT.get_or_init(|| async {
        init_dev().await;
        ModelManager::new().await.unwrap()
    }).await;

    mm.clone()
}
