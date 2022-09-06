use crate::components::game_state_provider::GameState;
use web_sys::CanvasRenderingContext2d;

mod entity;
mod map;

pub fn render(ctx: &CanvasRenderingContext2d, game_state: Option<&GameState>) {
    ctx.clear_rect(
        0.0,
        0.0,
        ctx.canvas().unwrap().width() as f64,
        ctx.canvas().unwrap().height() as f64,
    );
    if let Some(game_state) = game_state {
        map::render(ctx, &game_state.state.map);
        for entity in &game_state.state.entities {
            entity::render(ctx, entity);
        }
    }
}
