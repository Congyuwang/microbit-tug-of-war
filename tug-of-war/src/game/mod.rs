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
    IdleAnimation { dot: DotState, cnt: i8 },
    ReadyAnimation { count_down: u8, cnt: u8 },
    Playing { dot: DotState, cnt: i8 },
    Result { winner: Players, cnt: u16 },
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
                    // IdleAnimation -> ReadyAnimation
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
                    device.buttons.reset();
                    // Ready -> Playing (reset buttons)
                    *self = Self::playing(&mut device.rng, &mut device.buttons, &mut device.sound);
                }
            }
            Game::Playing { dot, cnt } => {
                if let Some(winner) = s2_game::game(cnt, dot, &device.buttons, &mut device.display)
                {
                    // Playing -> Result
                    // (buttons reset 1 sec afterwards within result_animation).
                    *self = Self::result(winner, &mut device.sound);
                }
            }
            Game::Result { cnt, winner } => {
                if s3_result::result_animation(
                    cnt,
                    winner,
                    &mut device.buttons,
                    &mut device.display,
                ) {
                    // Result -> ReadyAnimation
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
        let rand_num = rng.random_u8();
        crate::debug::info!("rand seed = {}", rand_num);
        if let 0..=127 = rand_num {
            dot.toggle_clockwise();
            buttons.set_last_a();
            crate::debug::info!("starting clockwise toggled");
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
        Game::Result { cnt: 0, winner }
    }
}
