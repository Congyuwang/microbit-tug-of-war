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
const CHANNEL: pwm::Channel = pwm::Channel::C0;

struct Track {
    notes: Notes,
    position: usize,
}

impl Track {
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
    pub fn play_track(&mut self, notes: Notes) {
        // initialize to 1 since note[0] is immediately triggered.
        const INIT_PLAY_POS: usize = 1;
        // set new state to Playing
        let state = core::mem::replace(
            &mut self.state,
            AudioState::Playing {
                track: Track {
                    notes,
                    position: INIT_PLAY_POS,
                },
            },
        );
        // previous state
        match state {
            AudioState::Playing { .. } => {
                // restart pwm to completely stop previous track.
                self.pwm_mut().disable();
                self.pwm_mut().enable();
                crate::debug::info!("speaker re-enabled");
            }
            AudioState::Disconnected { speaker } => {
                // connect speaker if disconnected
                let speaker = speaker.into_push_pull_output(Level::Low);
                self.pwm_mut().set_output_pin(CHANNEL, speaker).enable();
                crate::debug::info!("speaker connected");
            }
        }
        // play the first note
        let pwm = self.pwm.take().unwrap();
        self.pwm.replace(Self::play_note(pwm, notes[0]));
    }

    /// handles LOOPS_DONE event.
    pub fn handle_interrupt(&mut self) {
        // reset event
        self.pwm_mut().reset_event(pwm::PwmEvent::LoopsDone);

        if let AudioState::Playing { track } = &mut self.state {
            if Self::play_next_note(track, &mut self.pwm) {
                self.disconnect();
            }
        }
    }

    #[inline]
    fn disconnect(&mut self) {
        self.pwm_mut().disable();
        let speaker = self
            .pwm_mut()
            .clear_output_pin(CHANNEL)
            .unwrap()
            .into_disconnected();
        self.state = AudioState::Disconnected { speaker };
        crate::debug::info!("speaker disconnected");
    }

    /// Play next note.
    ///
    /// Won't do anything if currently not 'Playing'.
    ///
    /// return done: bool
    #[inline]
    fn play_next_note(track: &mut Track, pwm: &mut Option<pwm::Pwm<PWM0>>) -> bool {
        if let Some(note) = track.next_note() {
            let pwm_inner = pwm.take().unwrap();
            pwm.replace(Self::play_note(pwm_inner, note));
            false
        } else {
            true
        }
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
    fn pwm_mut(&mut self) -> &mut pwm::Pwm<PWM0> {
        self.pwm.as_mut().unwrap()
    }

    #[inline]
    fn loops(t_ms: u16, sample_len: usize) -> u16 {
        // always >= 1.
        (t_ms as u32 * SAMPLE_FREQ as u32 / 1000_u32 / sample_len as u32) as u16
    }
}
