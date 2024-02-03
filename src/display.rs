use crate::DisplayPinsArray;
use microbit::hal::prelude::OutputPin as _;

pub fn display_px((x, y): &(u8, u8), (cols, rows): &mut DisplayPinsArray) {
    rows[*x as usize].set_high().unwrap();
    cols[*y as usize].set_low().unwrap();
}

#[inline]
pub fn undisplay_px((x, y): &(u8, u8), (cols, rows): &mut DisplayPinsArray) {
    rows[*x as usize].set_low().unwrap();
    cols[*y as usize].set_high().unwrap();
}

pub fn display_col(col: u8, col_code: u8, (cols, rows): &mut DisplayPinsArray) {
    cols[col as usize].set_low().unwrap();
    IntoIterator::into_iter([0b00001, 0b00010, 0b00100, 0b01000, 0b10000])
        .zip(rows)
        .for_each(|(mask, row)| {
            if mask & col_code as u32 != 0 {
                row.set_high().unwrap();
            }
        })
}

pub fn undisplay_col(col: u8, (cols, rows): &mut DisplayPinsArray) {
    cols[col as usize].set_high().unwrap();
    rows.iter_mut().for_each(|row| row.set_low().unwrap());
}
