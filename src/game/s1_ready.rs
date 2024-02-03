use crate::{
    display_col,
    sound::{Sound, DI_LO},
    undisplay_col, DisplayPinsArray,
};

/// by columns
const THREE: [u8; 4] = [0b01001, 0b10001, 0b10101, 0b01011];
const TWO: [u8; 4] = [0b11001, 0b10101, 0b10101, 0b10010];
const ONE: [u8; 4] = [0b10010, 0b11111, 0b10000, 0b00000];
const COUNTDOWN: [[u8; 4]; 3] = [ONE, TWO, THREE];

/// The animation when when ready.
///
/// returns true if countdown finishes.
pub fn ready_animation(
    cnt: &mut u8,
    count_down: &mut u8,
    display_pins: &mut DisplayPinsArray,
    sound: &mut Sound,
) -> bool {
    const ROW_MASK: u8 = 0b11;

    // compute col to display
    let col = *cnt & ROW_MASK;
    let cnt_down = *count_down - 1;

    // update screen
    display_countdown_col(
        col + 1,
        COUNTDOWN[cnt_down as usize][col as usize],
        display_pins,
    );

    if *cnt == 0 {
        // play countdown sound
        sound.play_track(&DI_LO);
    }

    // update states
    // 256HZ * 256 = 1s
    if *cnt == u8::MAX {
        *count_down -= 1;
        *cnt = 0;
    } else {
        *cnt += 1;
    }

    // count down finished
    if *count_down == 0 {
        // clear the last col
        clear_countdown_display(display_pins);
        true
    } else {
        false
    }
}

#[inline]
fn clear_countdown_display(display_pins: &mut DisplayPinsArray) {
    undisplay_col(4, display_pins);
}

fn display_countdown_col(col: u8, col_code: u8, display_pins: &mut DisplayPinsArray) {
    undisplay_col(col_to_undisplay(col), display_pins);
    display_col(col, col_code, display_pins);
}

#[inline]
fn col_to_undisplay(col: u8) -> u8 {
    match col {
        1 => 4,
        2 => 1,
        3 => 2,
        4 => 3,
        _ => panic!(),
    }
}
