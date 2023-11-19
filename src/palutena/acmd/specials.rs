use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*},
        lib::lua_const::*
    },
    smash_script::*
};

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        macros::FT_MOTION_RATE(agent, 1.2);
        frame(agent.lua_state_agent, 22.0);
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_EXPLOSIVEFLAME, false, -1);
        }
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("bust"), 120.0, 0.0, 0.0, 0.0, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specialn", palutena_specialn);
}