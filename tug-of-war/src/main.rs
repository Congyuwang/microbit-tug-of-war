#![no_main]
#![no_std]
use cortex_m_rt::entry;
use game::Game;
use microbit::{
    board::Buttons,
    gpio::{DisplayPins, NUM_COLS, NUM_ROWS},
    hal::{
        gpio::{p0::P0_00, Disconnected, PushPull},
        gpiote::Gpiote,
        rtc::RtcInterrupt,
        Clocks, Rng, Rtc,
    },
    pac::{interrupt, CLOCK, GPIOTE, NVIC, PWM0, RNG, RTC0},
    Board,
};
#[cfg(not(debug_assertions))]
use panic_halt as _;
#[cfg(debug_assertions)]
use panic_rtt_target as _;
#[cfg(debug_assertions)]
use rtt_target::rtt_init_print;

mod buttons;
mod display;
mod game;
mod sound;
mod spiral;
use buttons::*;
use display::*;
use embed_mutex::*;
use sound::*;
use spiral::*;

static RTC: Mutex<Rtc<RTC0>> = Mutex::new_uinit();
static GAME: Mutex<Game> = Mutex::new(Game::new());
static DEVICE: Mutex<Device> = Mutex::new_uinit();

type DisplayPinsArray = (
    [microbit::hal::gpio::Pin<microbit::hal::gpio::Output<PushPull>>; NUM_COLS],
    [microbit::hal::gpio::Pin<microbit::hal::gpio::Output<PushPull>>; NUM_ROWS],
);

/// Devices used for the game.
struct Device {
    buttons: ButtonState,
    display: DisplayPinsArray,
    sound: Sound,
    rng: Rng,
}

#[entry]
fn main() -> ! {
    #[cfg(debug_assertions)]
    rtt_init_print!();
    let board = Board::take().unwrap();
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
    loop {
        cortex_m::asm::wfi();
    }
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
    cortex_m::interrupt::free(|cs| RTC.init(cs, rtc0));
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
    let buttons = ButtonState::new(
        buttons.button_a.degrade(),
        buttons.button_b.degrade(),
        Gpiote::new(gpiote),
    );
    let display = display.degrade();
    let sound = Sound::new(pwm, speaker.degrade());
    let rng = Rng::new(rng);
    cortex_m::interrupt::free(|cs| {
        DEVICE.init(
            cs,
            Device {
                buttons,
                display,
                sound,
                rng,
            },
        )
    });
}

/// main interrupt to drive display and game progress.
#[interrupt]
fn RTC0() {
    cortex_m::interrupt::free(|cs| {
        if let (Some(mut device), Some(mut rtc), Some(mut game)) =
            (DEVICE.try_lock(cs), RTC.try_lock(cs), GAME.try_lock(cs))
        {
            game.poll(&mut rtc, &mut device);
        }
    });
}

/// interrupt for playing sound.
#[interrupt]
fn PWM0() {
    cortex_m::interrupt::free(|cs| {
        if let Some(mut device) = DEVICE.try_lock(cs) {
            device.sound.handle_interrupt();
        }
    });
}

/// interrupt for buttons.
#[interrupt]
fn GPIOTE() {
    cortex_m::interrupt::free(|cs| {
        if let Some(mut device) = DEVICE.try_lock(cs) {
            device.buttons.handle_interrupt();
        }
    });
}
