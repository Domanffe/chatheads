// ChatHeads' half in the chat's own crate: the chat's lines move right by
// the head and its gap while the mod is on, so the heads drawn after the
// chat is painted (`horizon_client::game::chat::native_chatheads_glue`) land
// in the strip the lines leave. A static, because the chat has no `Game` to
// ask; `draw_chat`'s HEAD sets it each frame, before the chat is laid out.

use std::sync::atomic::{AtomicBool, Ordering};

/// What the chat's lines move right by: a head is 8 GUI pixels and keeps 2
/// of gap to its text (`runtime.rs`'s `HEAD` and the gap there).
const INDENT: i32 = 10;

pub static ON: AtomicBool = AtomicBool::new(false);

pub fn shifted(mut draws: Vec<crate::hud_layout::GuiDraw>) -> Vec<crate::hud_layout::GuiDraw> {
    if !crate::chat::native_chatheads_layout::ON.load(Ordering::Relaxed) {
        return draws;
    }
    for draw in &mut draws {
        // The chat's own lines are the text it draws at x = 0.
        if let crate::hud_layout::GuiDraw::Text { x, .. } = draw
            && *x == 0
        {
            *x = INDENT;
        }
    }
    draws
}
