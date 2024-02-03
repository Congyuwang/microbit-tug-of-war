//! The state machine of the main game.

use cortex_m::interrupt::CriticalSection;
use microbit::hal::{rtc::RtcInterrupt, Rng};

use self::s2_game::Players;
use crate::{
    sound::{Sound, DI_HI, PEPPA},
    DotState, DEVICE, RTC,
};

mod s0_idle;
mod s1_ready;
mod s2_game;
mod s3_result;

/// The state machine of the game.
pub enum Game {
    IdleAnimation { dot: DotState, cnt: i8 },
    ReadyAnimation { cnt: u8, count_down: u8 },
    InGame { dot: DotState, cnt: i8 },
    Result { cnt: u8, winner: Players },
}

impl Game {
    /// initial state of the game.
    pub const fn new() -> Self {
        Game::IdleAnimation {
            cnt: s0_idle::INIT_CN,
            dot: DotState::new(),
        }
    }

    pub fn poll(&mut self, cs: &CriticalSection) {
        Self::reset_rtc(cs);
        let mut device_borrow = DEVICE.borrow(cs).borrow_mut();
        let device = match device_borrow.as_mut() {
            Some(device) => device,
            None => return,
        };
        match self {
            Game::IdleAnimation { cnt, dot } => {
                if s0_idle::idle_animation(cnt, dot, &device.buttons, &mut device.display) {
                    *self = Self::ready_animation();
                }
            }
            Game::ReadyAnimation { cnt, count_down } => {
                if s1_ready::ready_animation(
                    cnt,
                    count_down,
                    &mut device.display,
                    &mut device.sound,
                ) {
                    *self = Self::start_game(&mut device.rng, &mut device.sound);
                }
            }
            Game::InGame { dot, cnt } => {
                if let Some(winner) = s2_game::game(cnt, dot, &device.buttons, &mut device.display)
                {
                    *self = Self::result(winner, &mut device.sound);
                }
            }
            Game::Result { cnt, winner } => {
                s3_result::result_animation(cnt, &winner, &mut device.display)
            }
        }
    }

    fn ready_animation() -> Self {
        const COUNTDOWN: u8 = 3;
        Game::ReadyAnimation {
            cnt: 0,
            count_down: COUNTDOWN,
        }
    }

    fn start_game(rng: &mut Rng, sound: &mut Sound) -> Self {
        let mut dot = DotState::new();
        if let 0..=127 = rng.random_u8() {
            dot.toggle_clockwise();
        }
        sound.play_track(&DI_HI);
        Game::InGame {
            dot,
            cnt: s2_game::INIT_CNT,
        }
    }

    fn result(winner: Players, sound: &mut Sound) -> Self {
        sound.play_track(&PEPPA);
        Game::Result { cnt: 0, winner }
    }

    fn reset_rtc(cs: &CriticalSection) {
        RTC.borrow(cs)
            .borrow_mut()
            .as_mut()
            .map(|rtc| rtc.reset_event(RtcInterrupt::Tick));
    }
}
