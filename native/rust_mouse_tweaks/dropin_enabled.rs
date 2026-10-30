fn hook(game: &crate::game::Game<'_>, enabled: &mut bool) {
    game.rust_mouse_tweaks_enabled(enabled);
}
