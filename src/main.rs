use std::error::Error;

use serde::Deserialize;
use url::Url;

mod api_query;
mod browser_screenshot;
mod shopify_partner_api;

#[derive(Deserialize, Debug)]
struct Config {
    access_token: String,
    api_url: String,
    app_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    // 環境変数取得
    let config = envy::from_env::<Config>().expect(".envファイルの設定が必要です");
    let url = Url::parse(&config.api_url)?;

    shopify_partner_api::get_app_installed_events(&url, &config.access_token, &config.app_id, None)
        .await?;

    Ok(())
}
