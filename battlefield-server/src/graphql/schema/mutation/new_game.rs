use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject)]
pub struct NewGame {
    pub scenario: String,
}
