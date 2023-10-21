mod dev_db;

use tokio::sync::OnceCell;
use tracing::info;

/// Initialize environment for local development
/// For early development, it will be called in main()
pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:12} - {}", "DEV ONLY", "Initializing dev environment");

        dev_db::init_dev_db().await.unwrap();
    }).await;
}
