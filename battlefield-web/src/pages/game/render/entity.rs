use super::map::{tile_offset, TILE_SIZE};
use crate::engine::components::*;
use crate::engine::*;
use battlefield_api::Entity;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

const ENTITY_PADDING: f64 = 6.0;

pub fn render(ctx: &CanvasRenderingContext2d, entity: &Entity) {
    if let Some((x, y)) = core::Position::extract(entity) {
        ctx.set_fill_style(&JsValue::from("blue"));
        ctx.fill_rect(
            tile_offset(x) + ENTITY_PADDING,
            tile_offset(y) + ENTITY_PADDING,
            TILE_SIZE - ENTITY_PADDING * 2.0,
            TILE_SIZE - ENTITY_PADDING * 2.0,
        );
        ctx.set_font("12pt");
        ctx.set_text_baseline("middle");
        ctx.set_text_align("center");
        ctx.set_fill_style(&JsValue::from("white"));
        ctx.fill_text(
            &entity.id.0.to_string(),
            tile_offset(x) + TILE_SIZE / 2.0,
            tile_offset(y) + TILE_SIZE / 2.0,
        )
        .ok();
    }
}
