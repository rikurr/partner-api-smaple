use std::{collections::HashMap, error::Error};

use gql_client::Client;
use url::Url;

use crate::api_query::{
    RelationshipInstalledData, RelationshipInstalledQuery, RelationshipInstalledVariables,
};

pub async fn get_app_installed_events(
    url: &Url,
    access_token: &str,
    app_id: &str,
    cursor: Option<&str>,
) -> Result<(), Box<dyn Error>> {
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

    println!("Id: {:?}", data);

    Ok(())
}
