use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*},
        lib::lua_const::*
    },
    smash_script::*
};

use crate::vars::FIGHTER_KIRBY_GENERATE_ARTICLE_FIREBALL;
use crate::vars::FIGHTER_KIRBY_GENERATE_ARTICLE_EXPLOSIVEFLAME;
use crate::vars::FIGHTER_KIRBY_GENERATE_ARTICLE_EXPLOSIVEFLAME_RESERVE;
use crate::vars::FIGHTER_KIRBY_GENERATE_ARTICLE_REFLECTIONBOARD;


unsafe extern "C" fn kirby_attacks3(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 1.2);
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_KIRBY_GENERATE_ARTICLE_FIREBALL, false, -1);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_attacks3", kirby_attacks3);
}