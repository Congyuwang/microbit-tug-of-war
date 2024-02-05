use microbit::hal::{
    gpio::{Floating, Input, Pin},
    gpiote::Gpiote,
    prelude::InputPin as _,
};

const LAST_BUTTON_MASK: u8 = 0b0000_0001;
const BOTH_AB_MASK: u8 = 0b0000_0010;

/// Buttons
pub struct ButtonState {
    state: u8,
    pub button_a: Pin<Input<Floating>>,
    pub button_b: Pin<Input<Floating>>,
    gpiote: Gpiote,
}

impl ButtonState {
    pub fn new(
        button_a: Pin<Input<Floating>>,
        button_b: Pin<Input<Floating>>,
        gpiote: Gpiote,
    ) -> Self {
        gpiote
            .channel0()
            .input_pin(&button_a)
            .hi_to_lo()
            .enable_interrupt();
        gpiote
            .channel1()
            .input_pin(&button_b)
            .hi_to_lo()
            .enable_interrupt();
        Self {
            state: 0,
            button_a,
            button_b,
            gpiote,
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
    fn set_last_a(state: &mut u8) {
        *state &= !LAST_BUTTON_MASK;
    }

    #[inline]
    fn set_last_b(state: &mut u8) {
        *state |= LAST_BUTTON_MASK;
    }

    #[inline]
    fn set_both_pressed(state: &mut u8) {
        *state |= BOTH_AB_MASK;
    }

    pub fn handle_interrupt(&mut self) {
        let button_a = self.gpiote.channel0();
        let button_b = self.gpiote.channel1();
        if button_a.is_event_triggered() {
            crate::debug::info!("button A");
            Self::set_last_a(&mut self.state);
            if self.button_b.is_low().unwrap() {
                crate::debug::info!("button A + B");
                Self::set_both_pressed(&mut self.state);
            }
            button_a.reset_events();
        }
        if button_b.is_event_triggered() {
            crate::debug::info!("button B");
            Self::set_last_b(&mut self.state);
            if self.button_a.is_low().unwrap() {
                crate::debug::info!("button A + B");
                Self::set_both_pressed(&mut self.state);
            }
            button_b.reset_events();
        }
    }
}
