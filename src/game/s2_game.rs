//! Idle animation, before game starts.
use core::sync::atomic::compiler_fence;

use crate::{display_px, undisplay_px, ButtonState, DisplayPinsArray, DotState, CENTER};

pub const INIT_CNT: i8 = -1;

#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
pub enum Players {
    A = 0,
    B = 1,
}

/// The animation when idle.
pub fn game(
    cnt: &mut i8,
    dot: &mut DotState,
    buttons: &ButtonState,
    display_pins: &mut DisplayPinsArray,
) -> Option<Players> {
    const COUNTER_MASK: i8 = 0b1111;

    // display the running dot and goal position
    match *cnt {
        INIT_CNT => display_px(&dot.px(), display_pins),
        6 | 14 => {
            undisplay_px(&dot.px(), display_pins);
            display_px(&CENTER, display_pins);
        }
        7 => {
            undisplay_px(&CENTER, display_pins);
            display_px(&dot.px(), display_pins);
        }
        15 => {
            undisplay_px(&CENTER, display_pins);
            if let Some(winner) = dot_game_spiral(dot, buttons) {
                return Some(winner);
            }
            display_px(&dot.px(), display_pins);
        }
        _ => (),
    }

    // increment counter
    *cnt = (*cnt + 1) & COUNTER_MASK;
    None
}

/// compute the next dot position for game.
fn dot_game_spiral(dot: &mut DotState, buttons: &ButtonState) -> Option<Players> {
    let mut result = None;

    update_dot_motion(dot, buttons);
    compiler_fence(core::sync::atomic::Ordering::SeqCst);
    dot.spiral(|dot| {
        if dot.is_left() {
            result = Some(Players::A)
        } else {
            result = Some(Players::B)
        }
    });

    result
}

/// update dot motion
#[inline]
fn update_dot_motion(dot: &mut DotState, buttons: &ButtonState) {
    if let (true, true) | (false, false) = (dot.is_clockwise(), buttons.last_a()) {
        dot.toggle_clockwise();
    }
}
