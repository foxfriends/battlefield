#[cynic::schema_for_derives(
    file = "src/api/battlefield_server.graphql",
    module = "crate::api::schema"
)]
mod fragments {
    use serde::Serialize;

    #[derive(Clone, Eq, PartialEq, cynic::QueryFragment, Serialize, Debug)]
    pub struct ScenariosConnection {
        pub page_info: PageInfo,
        pub edges: Vec<ScenarioEdge>,
    }

    #[derive(Clone, Eq, PartialEq, cynic::QueryFragment, Serialize, Debug)]
    pub struct ScenarioEdge {
        pub cursor: Cursor,
        pub node: Scenario,
    }

    #[derive(Clone, Eq, PartialEq, cynic::QueryFragment, Serialize, Debug)]
    pub struct Scenario {
        pub description: Option<String>,
        pub errors: Vec<String>,
        pub is_valid: bool,
        pub path: String,
        pub name: String,
        pub modules: Option<Vec<ModuleConfigEntry>>,
    }

    #[derive(Clone, Eq, PartialEq, cynic::QueryFragment, Serialize, Debug)]
    pub struct ModuleConfigEntry {
        pub key: String,
        pub value: ModuleConfig,
    }

    #[derive(Clone, Eq, PartialEq, cynic::QueryFragment, Serialize, Debug)]
    pub struct ModuleConfig {
        pub id: String,
        pub name: String,
        pub version: String,
        pub config: Json,
    }

    #[derive(Clone, Eq, PartialEq, cynic::QueryFragment, Serialize, Debug)]
    pub struct Game {
        pub id: String,
        // pub commands: Vec<Json>,
        pub scenario: Json,
        pub state: Json,
    }

    #[derive(Clone, Eq, PartialEq, cynic::QueryFragment, Serialize, Debug)]
    pub struct PageInfo {
        pub end_cursor: Cursor,
        pub has_next_page: bool,
        pub has_previous_page: bool,
        pub start_cursor: Cursor,
    }

    #[derive(Clone, Eq, PartialEq, Hash, cynic::Scalar, Debug)]
    pub struct Cursor(pub String);

    impl From<Cursor> for yew::virtual_dom::Key {
        fn from(cursor: Cursor) -> Self {
            Self::from(cursor.0)
        }
    }

    #[derive(Eq, PartialEq, cynic::Scalar, Debug, Clone)]
    pub struct Json(pub serde_json::Value);
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

#[cynic::schema_for_derives(
    file = "src/api/battlefield_server.graphql",
    module = "crate::api::schema"
)]
mod mutations {
    use super::fragments::*;

    #[derive(cynic::FragmentArguments, Debug)]
    pub struct NewGameArguments {
        pub new_game: NewGame,
    }

    #[derive(cynic::InputObject, Debug)]
    pub struct NewGame {
        pub scenario: String,
        pub players: Vec<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(argument_struct = "NewGameArguments", graphql_type = "Mutation")]
    pub struct NewGameMutation {
        #[arguments(new_game = &args.new_game)]
        #[cynic(rename = "new_game")]
        pub game: Game,
    }
}

mod schema {
    cynic::use_schema!("src/api/battlefield_server.graphql");
}

pub use cynic::{http::SurfExt, MutationBuilder, QueryBuilder};
pub use fragments::*;
pub use mutations::*;
pub use queries::*;

#[derive(Debug)]
pub enum ApiError {
    RequestError(surf::Error),
    GraphQlErrors(Vec<cynic::GraphQlError>),
    Other(String),
}

impl ApiError {
    pub fn other(error: impl std::fmt::Display) -> Self {
        Self::Other(error.to_string())
    }
}
