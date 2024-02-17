//! The state machine of the main game.
use microbit::{
    hal::{rtc::RtcInterrupt, Rng, Rtc},
    pac::RTC0,
};

use self::s2_game::Players;
use crate::{sound::Sound, ButtonState, Device, DotState, DI_HI, PEPPA};

mod s0_idle;
mod s1_ready;
mod s2_game;
mod s3_result;

/// The state machine of the game.
pub enum Game {
    /// Idle animation after device is started.
    IdleAnimation {
        /// position of the dot
        dot: DotState,
        /// tick count (256HZ)
        cnt: i8,
    },
    /// Count down animation after both players are ready.
    ReadyAnimation {
        /// count down (initialized as 3).
        count_down: u8,
        /// tick count (256HZ)
        cnt: u8,
    },
    /// On-going game.
    Playing {
        /// position of the dot
        dot: DotState,
        /// tick count (256HZ)
        cnt: i8,
    },
    /// Result animation
    Result {
        /// who wins
        winner: Players,
        /// tick count (256HZ)
        cnt: u8,
        /// flag to wait at least 1 sec before ready again.
        one_sec: bool,
    },
}

impl Game {
    /// initial state of the game.
    pub const fn new() -> Self {
        Game::IdleAnimation {
            cnt: s0_idle::INIT_CN,
            dot: DotState::new(),
        }
    }

    pub fn poll(&mut self, rtc: &mut Rtc<RTC0>, device: &mut Device) {
        rtc.reset_event(RtcInterrupt::Tick);
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
                    *self = Self::playing(&mut device.rng, &mut device.buttons, &mut device.sound);
                }
            }
            Game::Playing { dot, cnt } => {
                if let Some(winner) = s2_game::game(cnt, dot, &device.buttons, &mut device.display)
                {
                    *self = Self::result(winner, &mut device.sound);
                }
            }
            Game::Result {
                cnt,
                winner,
                one_sec,
            } => {
                if s3_result::result_animation(
                    cnt,
                    winner,
                    one_sec,
                    &mut device.buttons,
                    &mut device.display,
                ) {
                    *self = Self::ready_animation()
                }
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

    fn playing(rng: &mut Rng, buttons: &mut ButtonState, sound: &mut Sound) -> Self {
        let mut dot = DotState::new();
        buttons.reset();
        if let 0..=127 = rng.random_u8() {
            dot.toggle_clockwise();
            buttons.set_last_a();
        }
        crate::debug::info!("starting clockwise = {}", dot.is_clockwise());
        sound.play_track(&DI_HI);
        Game::Playing {
            dot,
            cnt: s2_game::INIT_CNT,
        }
    }

    fn result(winner: Players, sound: &mut Sound) -> Self {
        sound.play_track(&PEPPA);
        Game::Result {
            cnt: 0,
            winner,
            one_sec: false,
        }
    }
}
