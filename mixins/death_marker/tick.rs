use crate::runtime::check_and_save_death;

pub fn invoke(game: &mut crate::game::Game<'_>) {
    if let Some(player) = game.player() {
        let health = player.health();
        let pos = player.position();

        unsafe {
            static mut LAST_HEALTH: f32 = 20.0;

            if LAST_HEALTH > 0.0 && health <= 0.0 {
                check_and_save_death(pos[0], pos[1], pos[2]);
            }

            LAST_HEALTH = health;
        }
    }
}