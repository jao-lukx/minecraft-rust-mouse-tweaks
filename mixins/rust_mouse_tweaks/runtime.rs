use super::{Game, native_hooks::NativeMod};
impl Game<'_> {
    pub(super) fn rust_mouse_tweaks_list(&mut self, mods: &mut Vec<NativeMod>) {
        mods.push(NativeMod {
            id: "rust_mouse_tweaks".into(),
            title: "Mouse Tweaks Rust 0.1.0".into(),
            description: "Arrastar, coletar, Shift e roda; usa o motor de cliques do Horizon."
                .into(),
            keys: vec![],
        });
    }
}
