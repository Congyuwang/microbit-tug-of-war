# Microbit V2 Tug of War Game with Rust

This is a simple game for the microbit V2. The game is a tug of war game where the player has to press the A or B button as fast as possible to win the game. The game is written in Rust and uses the microbit V2 hardware.

https://github.com/Congyuwang/microbit-tug-of-war/assets/52687642/4b6673a0-a52e-4610-b32a-0f3c768878db

## Technical details

- RTC interrupt for the game loop and LED updates
- GPIOTE interrupt for the button presses
- PWM + DMA interrupt for sound playback
