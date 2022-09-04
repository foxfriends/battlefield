#[cynic::schema_for_derives(
    file = "src/api/battlefield_server.graphql",
    module = "crate::api::schema"
)]
mod fragments {
    #[derive(cynic::QueryFragment, Debug)]
    pub struct ScenariosConnection {
        pub page_info: PageInfo,
        pub edges: Vec<ScenarioEdge>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct ScenarioEdge {
        pub cursor: Cursor,
        pub node: Scenario,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct Scenario {
        pub description: Option<String>,
        pub errors: Vec<String>,
        pub is_valid: bool,
        pub path: String,
        pub name: String,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct PageInfo {
        pub end_cursor: Cursor,
        pub has_next_page: bool,
        pub has_previous_page: bool,
        pub start_cursor: Cursor,
    }

    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct Cursor(pub String);
}

#[cynic::schema_for_derives(
    file = "src/api/battlefield_server.graphql",
    module = "crate::api::schema"
)]
mod queries {
    use super::fragments::*;

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct ListScenariosArguments {
        pub first: i32,
        pub after: Option<Cursor>,
    }

    impl Default for ListScenariosArguments {
        fn default() -> Self {
            Self {
                first: 50,
                after: None,
            }
        }
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(argument_struct = "ListScenariosArguments", graphql_type = "Query")]
    pub struct ListScenariosQuery {
        #[arguments(first = &args.first, after = &args.after)]
        pub scenarios_connection: ScenariosConnection,
    }
}

mod schema {
    cynic::use_schema!("src/api/battlefield_server.graphql");
}

pub use cynic::{http::SurfExt, QueryBuilder};
pub use fragments::*;
pub use queries::*;

#[derive(Debug)]
pub enum ApiError {
    RequestError(surf::Error),
    GraphQlErrors(Vec<cynic::GraphQlError>),
}
