use microbit::hal::gpio::{Floating, Input, Pin};

const LAST_BUTTON_MASK: u8 = 0b0000_0001;
const BOTH_AB_MASK: u8 = 0b0000_0010;

/// Buttons
pub struct ButtonState {
    pub button_a: Pin<Input<Floating>>,
    pub button_b: Pin<Input<Floating>>,
    state: u8,
}

impl ButtonState {
    pub fn new(button_a: Pin<Input<Floating>>, button_b: Pin<Input<Floating>>) -> Self {
        Self {
            button_a,
            button_b,
            state: 0,
        }
    }

    #[inline]
    pub fn set_both_pressed(&mut self) {
        self.state |= BOTH_AB_MASK;
    }

    #[inline]
    pub fn both_pressed(&self) -> bool {
        self.state & BOTH_AB_MASK != 0
    }

    #[inline]
    pub fn set_last_a(&mut self) {
        self.state &= !LAST_BUTTON_MASK;
    }

    #[inline]
    pub fn set_last_b(&mut self) {
        self.state |= LAST_BUTTON_MASK;
    }

    #[inline]
    pub fn last_a(&self) -> bool {
        self.state & LAST_BUTTON_MASK == 0
    }
}
