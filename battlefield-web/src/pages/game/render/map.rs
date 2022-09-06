use battlefield_api::Map;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub const TILE_SIZE: f64 = 34.0;
pub const TILE_GAP: f64 = 1.0;

pub fn tile_offset(i: usize) -> f64 {
    i as f64 * TILE_SIZE
}

pub fn render(ctx: &CanvasRenderingContext2d, map: &Map) {
    for (y, row) in map.tiles.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            ctx.set_fill_style(&JsValue::from("green"));
            ctx.fill_rect(
                tile_offset(x) + TILE_GAP,
                tile_offset(y) + TILE_GAP,
                TILE_SIZE - TILE_GAP * 2.0,
                TILE_SIZE - TILE_GAP * 2.0,
            );
        }
    }
}
