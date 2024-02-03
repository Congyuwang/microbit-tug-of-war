use super::s2_game::Players;
use crate::{display_col, undisplay_col, DisplayPinsArray};

/// by columns
const CHAR_A: [u8; 4] = [0b11110, 0b00101, 0b00101, 0b11110];
const CHAR_B: [u8; 4] = [0b11111, 0b10101, 0b10101, 0b01010];
const CHAR_AB: [[u8; 4]; 2] = [CHAR_A, CHAR_B];

pub fn result_animation(cnt: &mut u8, winner: &Players, display_pins: &mut DisplayPinsArray) {
    const ROW_MASK: u8 = 0b11;

    // compute col to display
    let col = *cnt & ROW_MASK;

    // update screen
    match *cnt {
        0..=127 => display_result_col(col, winner, display_pins),
        128 => clear_countdown_display(display_pins, winner),
        _ => (),
    }

    // update states
    // 256HZ * 256 = 1s
    *cnt += 1;
}

#[inline]
fn clear_countdown_display(display_pins: &mut DisplayPinsArray, winner: &Players) {
    let player_b_wins = (*winner == Players::B) as u8;
    undisplay_col(3 + player_b_wins, display_pins);
}

#[inline]
fn display_result_col(col: u8, winner: &Players, display_pins: &mut DisplayPinsArray) {
    let player_b_wins = (*winner == Players::B) as u8;
    undisplay_col(col_to_undisplay(col, winner), display_pins);
    display_col(
        col + player_b_wins,
        CHAR_AB[*winner as usize][col as usize],
        display_pins,
    );
}

#[inline]
fn col_to_undisplay(col: u8, winner: &Players) -> u8 {
    let player_b_wins = (*winner == Players::B) as u8;
    match col {
        0 => 3 + player_b_wins,
        1 => 0 + player_b_wins,
        2 => 1 + player_b_wins,
        3 => 2 + player_b_wins,
        _ => panic!(),
    }
}
