use microbit::{
    hal::{
        gpio::{Disconnected, Level, Pin},
        pwm,
    },
    pac::PWM0,
};

use crate::{Note, Notes};

const MAX_DUTY: u16 = 256;
const SAMPLE_FREQ: u16 = 62500;
const CHANNEL: microbit::hal::pwm::Channel = microbit::hal::pwm::Channel::C0;

struct Track {
    notes: Notes,
    position: usize,
}

impl Track {
    fn new(notes: Notes) -> Self {
        Self { notes, position: 0 }
    }

    fn next_note(&mut self) -> Option<Note> {
        match self.notes.get(self.position) {
            Some(note) => {
                self.position += 1;
                Some(*note)
            }
            None => None,
        }
    }
}

pub struct Sound {
    pwm: Option<pwm::Pwm<PWM0>>,
    state: AudioState,
}

enum AudioState {
    Disconnected { speaker: Pin<Disconnected> },
    Idle,
    Playing { track: Track },
}

impl Sound {
    pub fn init(pwm: PWM0, speaker: Pin<Disconnected>) -> Self {
        let pwm = pwm::Pwm::new(pwm);
        pwm.set_counter_mode(pwm::CounterMode::Up)
            .set_seq_refresh(pwm::Seq::Seq0, 0)
            .set_seq_end_delay(pwm::Seq::Seq0, 0)
            .set_prescaler(pwm::Prescaler::Div1)
            .set_load_mode(pwm::LoadMode::Common)
            .enable_channel(CHANNEL)
            .enable_interrupt(pwm::PwmEvent::LoopsDone)
            .set_max_duty(MAX_DUTY);
        let state = AudioState::Disconnected { speaker };
        Self {
            pwm: Some(pwm),
            state,
        }
    }

    /// set track and start playing.
    /// If currently playing, stop this track.
    pub fn play_track(&mut self, track: Notes) {
        // stop the playing track.
        if let AudioState::Playing { .. } = self.state {
            self.stop();
        }

        // if disconnected, connect.
        // if idle, set track.
        // if track set, play next note.
        loop {
            match self.state {
                AudioState::Disconnected { .. } => self.connect(),
                AudioState::Idle { .. } => self.set_track(track),
                AudioState::Playing { .. } => {
                    self.play_next_note();
                    break;
                }
            };
        }
    }

    /// handles LOOPS_DONE event.
    pub fn handle_interrupt(&mut self) {
        // reset event
        self.pwm_mut().reset_event(pwm::PwmEvent::LoopsDone);

        // LOOPS_DONE

        // if track unfinished, play next note.
        // if track finished, stop playing.
        // if idle, disconnect.
        // if disconnected, do nothing.
        loop {
            match self.state {
                AudioState::Playing { .. } => {
                    if self.play_next_note() {
                        self.stop();
                        // go on to execute disconnect
                    } else {
                        break;
                    }
                }
                AudioState::Idle { .. } => {
                    self.disconnect();
                    break;
                }
                AudioState::Disconnected { .. } => break,
            };
        }
    }

    /// Set track and swtich to playing.
    ///
    /// Switch from Idle to Playing.
    #[inline]
    fn set_track(&mut self, notes: Notes) {
        if let AudioState::Idle = self.state {
            self.state = AudioState::Playing {
                track: Track::new(notes),
            };
        } else {
            unreachable!()
        }
    }

    /// Stop pwm, unset track and switch to idle.
    ///
    /// Switch from Playing to Idle.
    #[inline]
    fn stop(&mut self) {
        if let AudioState::Playing { track: _ } = self.state {
            self.pwm_mut().stop();
            self.pwm_mut().disable();
            self.pwm_mut().enable();
            self.state = AudioState::Idle;
        } else {
            unreachable!()
        }
    }

    /// Connect speaker pin to pwm generator.
    ///
    /// Switch from Disconnected to Idle.
    #[inline]
    fn connect(&mut self) {
        let state = core::mem::replace(&mut self.state, AudioState::Idle);
        if let AudioState::Disconnected { speaker } = state {
            let speaker = speaker.into_push_pull_output(Level::Low);
            self.pwm_mut()
                .set_output_pin(pwm::Channel::C0, speaker)
                .enable();
        } else {
            unreachable!()
        }
    }

    /// Disconnect speaker pin to save power.
    ///
    /// Switch from Idle to Disconnected.
    #[inline]
    fn disconnect(&mut self) {
        if let AudioState::Idle = self.state {
            self.pwm_mut().disable();
            let speaker = self
                .pwm_mut()
                .clear_output_pin(CHANNEL)
                .unwrap()
                .into_disconnected();
            self.state = AudioState::Disconnected { speaker };
        } else {
            unreachable!()
        }
    }

    /// Play next note.
    ///
    /// Won't do anything if currently not 'Playing'.
    ///
    /// return done: bool
    #[inline]
    fn play_next_note(&mut self) -> bool {
        if let AudioState::Playing { track } = &mut self.state {
            if let Some(note) = track.next_note() {
                let pwm = self.pwm.take().unwrap();
                self.pwm.replace(Self::play_note(pwm, note));
                false
            } else {
                true
            }
        } else {
            unreachable!()
        }
    }

    #[inline]
    fn pwm_mut(&mut self) -> &mut pwm::Pwm<PWM0> {
        self.pwm.as_mut().unwrap()
    }

    /// Silent note is treated differently with refresh instead of loop.
    #[inline]
    fn play_note(pwm: pwm::Pwm<PWM0>, (note, t_ms): Note) -> pwm::Pwm<PWM0> {
        pwm.repeat(Self::loops(t_ms, note.len()));
        let (s0, s1) = note.split_at(note.len() / 2);
        let (_, _, pwm) = pwm.load(Some(s0), Some(s1), true).unwrap().split();
        pwm
    }

    #[inline]
    fn loops(t_ms: u16, sample_len: usize) -> u16 {
        (t_ms as u32 * SAMPLE_FREQ as u32 / 1000_u32 / sample_len as u32).max(1) as u16
    }
}
