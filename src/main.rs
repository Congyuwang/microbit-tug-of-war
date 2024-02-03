#![no_main]
#![no_std]
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use game::Game;
use microbit::{
    board::Buttons,
    gpio::{DisplayPins, NUM_COLS, NUM_ROWS},
    hal::{
        gpio::{p0::P0_00, Disconnected, PushPull},
        gpiote::Gpiote,
        prelude::InputPin,
        rtc::RtcInterrupt,
        Clocks, Rng, Rtc,
    },
    pac::{interrupt, CLOCK, GPIOTE, NVIC, PWM0, RNG, RTC0},
    Board,
};
use panic_halt as _;
// use panic_rtt_target as _;
// use rtt_target::rtt_init_print;

mod buttons;
mod display;
mod game;
mod sound;
mod spiral;
use buttons::*;
use display::*;
use sound::Sound;
use spiral::*;

static RTC: Mutex<RefCell<Option<Rtc<RTC0>>>> = Mutex::new(RefCell::new(None));
static GAME: Mutex<RefCell<Game>> = Mutex::new(RefCell::new(Game::new()));
static DEVICE: Mutex<RefCell<Option<Device>>> = Mutex::new(RefCell::new(None));

type DisplayPinsArray = (
    [microbit::hal::gpio::Pin<microbit::hal::gpio::Output<PushPull>>; NUM_COLS],
    [microbit::hal::gpio::Pin<microbit::hal::gpio::Output<PushPull>>; NUM_ROWS],
);

/// Devices used for the game.
struct Device {
    display: DisplayPinsArray,
    buttons: ButtonState,
    gpiote: Gpiote,
    rng: Rng,
    sound: Sound,
}

#[entry]
fn main() -> ! {
    // rtt_init_print!();
    let board = Board::take().unwrap();
    // board.
    init_rtc(board.CLOCK, board.RTC0);
    init_device(
        board.display_pins,
        board.buttons,
        board.GPIOTE,
        board.RNG,
        board.speaker_pin,
        board.PWM0,
        board.NVIC,
    );
    loop {}
}

/// initialize a 256HZ RTC clock.
fn init_rtc(clock: CLOCK, rtc0: RTC0) {
    // 256HZ (32768 / 256 - 1)
    const RTC_PRESCALER: u32 = 127u32;

    Clocks::new(clock).set_lfclk_src_rc().start_lfclk();
    let mut rtc0 = Rtc::new(rtc0, RTC_PRESCALER).unwrap();
    rtc0.enable_event(RtcInterrupt::Tick);
    rtc0.enable_interrupt(RtcInterrupt::Tick, None);
    rtc0.enable_counter();
    cortex_m::interrupt::free(|cs| {
        RTC.borrow(cs).borrow_mut().replace(rtc0);
    });
}

/// initialize DEVICE variable.
fn init_device(
    display: DisplayPins,
    buttons: Buttons,
    gpiote: GPIOTE,
    rng: RNG,
    speaker: P0_00<Disconnected>,
    pwm: PWM0,
    mut nvic: NVIC,
) {
    // enable interrupts
    unsafe {
        nvic.set_priority(interrupt::GPIOTE, 32);
        nvic.set_priority(interrupt::RTC0, 64);
        nvic.set_priority(interrupt::PWM0, 128);
        NVIC::unmask(interrupt::RTC0);
        NVIC::unmask(interrupt::GPIOTE);
        NVIC::unmask(interrupt::PWM0);
    }
    // enable gpiote for buttons
    let gpiote = Gpiote::new(gpiote);
    let button_a = buttons.button_a.degrade();
    let button_b = buttons.button_b.degrade();
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
    let buttons = ButtonState::new(button_a, button_b);
    // prepare sound object
    let sound = Sound::new(pwm, speaker.degrade());
    cortex_m::interrupt::free(|cs| {
        DEVICE.borrow(cs).borrow_mut().replace(Device {
            display: display.degrade(),
            buttons,
            gpiote,
            rng: Rng::new(rng),
            sound,
        });
    });
}

/// main interrupt to drive display and game progress.
#[interrupt]
fn RTC0() {
    cortex_m::interrupt::free(|cs| {
        GAME.borrow(cs).borrow_mut().poll(cs);
    });
}

/// interrupt for playing sound.
#[interrupt]
fn PWM0() {
    cortex_m::interrupt::free(|cs| {
        if let Some(device) = DEVICE.borrow(cs).borrow_mut().as_mut() {
            device.sound.handle_interrupt();
        }
    });
}

/// interrupt for buttons.
#[interrupt]
fn GPIOTE() {
    cortex_m::interrupt::free(|cs| {
        if let Some(device) = DEVICE.borrow(cs).borrow_mut().as_mut() {
            let button_a = device.gpiote.channel0().is_event_triggered();
            let button_b = device.gpiote.channel1().is_event_triggered();
            if button_a {
                device.buttons.set_last_a();
                if device.buttons.button_b.is_low().unwrap() {
                    device.buttons.set_both_pressed();
                }
            }
            if button_b {
                device.buttons.set_last_b();
                if device.buttons.button_a.is_low().unwrap() {
                    device.buttons.set_both_pressed();
                }
            }
            device.gpiote.reset_events();
        }
    });
}
