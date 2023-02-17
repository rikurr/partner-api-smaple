use std::{collections::HashMap, error::Error, fs};

use serde::{Deserialize, Serialize};
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

type StoreName = String;

#[derive(Serialize, Debug)]
struct StoreMap(HashMap<StoreName, String>);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    // 環境変数取得
    let config = envy::from_env::<Config>().expect(".envファイルの設定が必要です");
    let url = Url::parse(&config.api_url)?;

    let api_data = shopify_partner_api::get_app_installed_events(
        &url,
        &config.access_token,
        &config.app_id,
        None,
    )
    .await?;

    let mut stores = StoreMap(HashMap::new());
    api_data.into_iter().for_each(|app| {
        let myshopify_domain =
            Url::parse(format!("https://{}", app.node.shop.myshopify_domain).as_str());

        let Ok(url) = myshopify_domain else { return; };
        stores.0.insert(app.node.shop.name, url.to_string());
    });
    let output_dir = "output";
    fs::create_dir_all(output_dir)?;
    let mut wtr = csv::Writer::from_path(format!("{}/name.csv", output_dir))?;
    wtr.write_record(["name", "url"])?;
    stores.0.iter().for_each(|(name, url)| {
        wtr.write_record([name, url]).unwrap();
    });

    wtr.flush()?;

    fs::create_dir_all("screenshot")?;
    stores.0.iter().for_each(|map| {
        let Ok(url) = Url::parse(map.1) else { return; };
        browser_screenshot::get(&url).unwrap();
    });

    Ok(())
}
