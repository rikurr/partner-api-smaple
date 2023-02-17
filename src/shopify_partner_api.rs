use std::{collections::HashMap, error::Error};

use gql_client::Client;
use url::Url;

use crate::api_query::{
    Edges, RelationshipInstalledData, RelationshipInstalledQuery, RelationshipInstalledVariables,
};
use async_recursion::async_recursion;

#[async_recursion]
pub async fn get_app_installed_events(
    url: &Url,
    access_token: &str,
    app_id: &str,
    cursor: Option<String>,
) -> Result<Vec<Edges>, Box<dyn Error>> {
    let mut headers = HashMap::new();
    headers.insert("X-Shopify-Access-Token", access_token);

    let client = Client::new_with_headers(url.as_str(), headers);
    let body = RelationshipInstalledQuery::new(cursor, app_id);

    let data = client
        .query_with_vars_unwrap::<RelationshipInstalledData, RelationshipInstalledVariables>(
            body.query,
            body.variables,
        )
        .await
        .unwrap();

    // 最後のデータの位置
    let end_cursor = data.app.events.edges.last().unwrap().cursor.clone();

    let mut app_events = Vec::new();
    app_events.extend(data.app.events.edges);

    // 取得できるデータがあれば再帰呼び出し
    if data.app.events.page_info.has_next_page {
        let new_app_events = get_app_installed_events(url, access_token, app_id, Some(end_cursor))
            .await
            .unwrap();
        app_events.extend(new_app_events);
        return Ok(app_events);
    }

    Ok(app_events)
}
