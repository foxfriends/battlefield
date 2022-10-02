use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject)]
pub struct NewGame {
    pub scenario: String,
    pub players: Vec<String>,
}
