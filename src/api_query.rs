use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct RelationshipInstalledQuery<'a> {
    pub query: &'a str,
    pub variables: RelationshipInstalledVariables<'a>,
}

#[derive(Serialize, Debug)]
pub struct RelationshipInstalledVariables<'a> {
    pub id: &'a str,
    pub cursor: String,
}

const RELATIONSHIP_INSTALLED_QUERY: &str = r#"
query($cursor: String, $id: ID!) {
    app(id: $id) {
        name
        events(types: [RELATIONSHIP_INSTALLED], after: $cursor, first: 50) {
            edges {
                cursor
                node {
                    occurredAt
                    type
                    shop{
                        id 
                        myshopifyDomain
                        name
                    }
                }
            }
            pageInfo {
                hasNextPage
            }
        }
    }
}
"#;

#[derive(Deserialize, Debug)]
pub struct RelationshipInstalledData {
    pub app: App,
}

#[derive(Deserialize, Debug)]
pub struct App {
    pub name: String,
    pub events: Events,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Events {
    pub edges: Vec<Edges>,
    pub page_info: PageInfo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct PageInfo {
    pub has_next_page: bool,
}

#[derive(Deserialize, Debug)]
pub struct Edges {
    pub cursor: String,
    pub node: Node,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Node {
    pub occurred_at: String,
    pub shop: Shop,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Shop {
    pub id: String,
    pub myshopify_domain: String,
    pub name: String,
}

impl<'a> RelationshipInstalledQuery<'a> {
    pub fn new(cursor: Option<String>, id: &'a str) -> Self {
        let cursor = cursor.unwrap_or(String::new());

        Self {
            query: RELATIONSHIP_INSTALLED_QUERY,
            variables: RelationshipInstalledVariables { id, cursor },
        }
    }
}
