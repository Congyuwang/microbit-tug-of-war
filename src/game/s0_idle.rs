//! Idle animation, before game starts.
use crate::{display_px, undisplay_px, ButtonState, DisplayPinsArray, DotState, CENTER};

pub const INIT_CN: i8 = -1;

/// The animation when idle.
///
/// returns true if started.
pub fn idle_animation(
    cnt: &mut i8,
    dot: &mut DotState,
    buttons: &ButtonState,
    display_pins: &mut DisplayPinsArray,
) -> bool {
    const COUNTER_MASK: i8 = 0b1111;

    // display the running dot
    match *cnt {
        INIT_CN => display_px(&dot.px(), display_pins),
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
            dot_idle_spiral(dot);
            display_px(&dot.px(), display_pins);
        }
        _ => (),
    }

    // increment counter
    *cnt = (*cnt + 1) & COUNTER_MASK;

    if game_started(buttons) {
        clear_idle_animation(dot, display_pins);
        true
    } else {
        false
    }
}

/// movement of dot in idle state.
fn dot_idle_spiral(dot: &mut DotState) {
    dot.spiral(|dot| dot.toggle_clockwise());
}

/// Check start.
#[inline]
fn game_started(buttons: &ButtonState) -> bool {
    buttons.both_pressed()
}

#[inline]
fn clear_idle_animation(dot: &DotState, display_pins: &mut DisplayPinsArray) {
    undisplay_px(&dot.px(), display_pins);
    undisplay_px(&CENTER, display_pins);
}
