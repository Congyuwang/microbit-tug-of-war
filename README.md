# Microbit V2 Tug of War Game with Rust

This is a simple game for the microbit V2. The game is a tug of war game where the player has to press the A or B button as fast as possible to win the game. The game is written in Rust and uses the microbit V2 hardware.

https://github.com/Congyuwang/microbit-tug-of-war/assets/52687642/4b6673a0-a52e-4610-b32a-0f3c768878db

## Flash
Connect microbit v2 to your computer and run `just flash` or `cargo embed --target thumbv7em-none-eabihf --release`.

## How to play
- Two players, one uses button A and the other uses button B.
- Press both A and B buttons at the same time to get ready.
- After the countdown's over, players have to press button A or B as fast as possible.
- The dot will run clockwise if B is pressed faster, and counter-clockwise if A is pressed faster.
- When the dot reaches the center, one of the player wins.
- Press both A and B buttons at the same time to get ready for another round.

## Technical details

- RTC interrupt for the game loop and LED updates
- GPIOTE interrupt for the button presses
- PWM + DMA interrupt for sound playback
