use microbit::hal::{
    gpio::{Floating, Input, Pin},
    gpiote::Gpiote,
    prelude::InputPin as _,
};

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
    pub fn both_pressed(&self) -> bool {
        self.state & BOTH_AB_MASK != 0
    }

    #[inline]
    pub fn last_a(&self) -> bool {
        self.state & LAST_BUTTON_MASK == 0
    }

    #[inline]
    pub fn reset(&mut self) {
        self.state = 0
    }

    #[inline]
    fn set_last_a(&mut self) {
        self.state &= !LAST_BUTTON_MASK;
    }

    #[inline]
    fn set_last_b(&mut self) {
        self.state |= LAST_BUTTON_MASK;
    }

    #[inline]
    fn set_both_pressed(&mut self) {
        self.state |= BOTH_AB_MASK;
    }

    pub fn handle_interrupt(&mut self, gpiote: &Gpiote) {
        let button_a = gpiote.channel0();
        let button_b = gpiote.channel1();
        if button_a.is_event_triggered() {
            self.set_last_a();
            if self.button_b.is_low().unwrap() {
                self.set_both_pressed();
            }
            button_a.reset_events();
        }
        if button_b.is_event_triggered() {
            self.set_last_b();
            if self.button_a.is_low().unwrap() {
                self.set_both_pressed();
            }
            button_b.reset_events();
        }
    }
}
