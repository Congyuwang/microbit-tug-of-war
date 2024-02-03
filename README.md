# Microbit V2 Tug of War Game with Rust

This is a simple game for the microbit V2. The game is a tug of war game where the player has to press the A or B button as fast as possible to win the game. The game is written in Rust and uses the microbit V2 hardware.

## Technical details

- RTC interrupt for the game loop and LED updates
- GPIOTE interrupt for the button presses
- PWM interrupt for sound playback
