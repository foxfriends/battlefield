#![allow(dead_code)]
use rhai::plugin::*;

lazy_static::lazy_static! {
    pub(crate) static ref PLAYER_MODULE: rhai::Shared<rhai::Module> = rhai::Shared::new(rhai::exported_module!(plugin_player));
}

#[allow(clippy::mut_mutex_lock)]
#[export_module]
mod plugin_player {
    pub type Player = crate::data::Player;

    #[rhai_fn(get = "id", pure)]
    pub fn get_id(player: &mut Player) -> usize {
        player.id
    }

    #[rhai_fn(get = "name", pure)]
    pub fn get_name(player: &mut Player) -> String {
        player.name.clone()
    }
}
