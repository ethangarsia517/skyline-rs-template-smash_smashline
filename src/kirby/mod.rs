use smashline;

mod acmd;

pub fn install() {

    let agent = &mut smashline::Agent::new("kirby");
    acmd::install(agent);
    agent.install();

    smashline::clone_weapon("palutena", "explosiveflame", "kirby", "explosiveflame",true);
    smashline::clone_weapon("mario", "fireball", "kirby", "fireball",true);
    smashline::clone_weapon("palutena", "explosiveflame_reserve", "kirby", "explosiveflame_reserve",true);
    smashline::clone_weapon("palutena", "reflectionboard", "kirby", "reflectionboard",true);
}