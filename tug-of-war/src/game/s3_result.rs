use super::s2_game::Players;
use crate::{display_col, undisplay_col, ButtonState, DisplayPinsArray};

/// by columns
const CHAR_A: [u8; 4] = [0b11110, 0b00101, 0b00101, 0b11110];
const CHAR_B: [u8; 4] = [0b11111, 0b10101, 0b10101, 0b01010];
const CHAR_AB: [[u8; 4]; 2] = [CHAR_A, CHAR_B];

const ROW_MASK: u8 = 0b11;
const LAST_COL: u8 = 3;

/// Returns whether both buttons are pressed.
pub fn result_animation(
    cnt: &mut u16,
    winner: &Players,
    buttons: &mut ButtonState,
    display_pins: &mut DisplayPinsArray,
) -> bool {
    let display_cycle = *cnt as u8;
    let player_b_wins = *winner as u8;

    // update screen
    match display_cycle {
        0..=127 => display_result_col(display_cycle, player_b_wins, display_pins),
        128 => undisplay_col(LAST_COL + player_b_wins, display_pins),
        _ => (),
    }

    // update states
    // 256HZ * 256 = 1s
    *cnt = cnt.wrapping_add(1);

    if *cnt == 255 {
        buttons.reset();
    } else if *cnt > 255 && buttons.both_pressed() {
        clear_result_col(display_cycle, player_b_wins, display_pins);
        return true;
    }

    false
}

#[inline]
fn display_result_col(display_cycle: u8, player_b_wins: u8, display_pins: &mut DisplayPinsArray) {
    let col = display_cycle & ROW_MASK;
    undisplay_col(prev_col(col) + player_b_wins, display_pins);
    display_col(
        col + player_b_wins,
        CHAR_AB[player_b_wins as usize][col as usize],
        display_pins,
    );
}

/// clear the currently displayed col if any before return.
#[inline]
fn clear_result_col(display_cycle: u8, player_b_wins: u8, display_pins: &mut DisplayPinsArray) {
    if let 0..=127 = display_cycle {
        let col = display_cycle & ROW_MASK;
        undisplay_col(col + player_b_wins, display_pins);
    }
}

#[inline]
fn prev_col(col: u8) -> u8 {
    match col {
        0 => 3,
        1 => 0,
        2 => 1,
        3 => 2,
        _ => panic!(),
    }
}
