use super::query;
use super::Context;
use juniper::FieldResult;

mod new_game;

use new_game::NewGame;

pub struct Mutation;

#[juniper::graphql_object(context = Context)]
impl Mutation {
    async fn new_game(&self, context: &Context, new_game: NewGame) -> FieldResult<query::Game> {
        use crate::game::Game;

        let scenario = context
            .engine
            .scenario(&new_game.scenario)
            .and_then(|scenario| scenario.data())
            .ok_or_else(move || anyhow::anyhow!("Scenario {} not found", new_game.scenario))?;
        let game = Game::new(
            scenario.clone(),
            context.database.clone(),
            context.engine.clone(),
        )
        .await?;
        Ok(query::Game(game.into()))
    }
}
