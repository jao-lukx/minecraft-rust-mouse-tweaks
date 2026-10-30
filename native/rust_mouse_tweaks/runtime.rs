use super::{Game, native_hooks::NativeMod};
impl Game<'_> {
    pub(super) fn rust_mouse_tweaks_list(&mut self, mods: &mut Vec<NativeMod>) {
        mods.push(NativeMod {
            id: "rust_mouse_tweaks".into(),
            title: "Mouse Tweaks Rust 0.2.0".into(),
            description: "Arrastar, coletar, Shift e roda.".into(),
            keys: vec![],
        });
    }
    pub(super) fn rust_mouse_tweaks_enabled(&self, enabled: &mut bool) {
        *enabled |= self.native_on("rust_mouse_tweaks");
    }
}
