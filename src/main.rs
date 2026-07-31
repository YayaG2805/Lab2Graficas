use std::time::{Duration, Instant};

use lab2_graficas::framebuffer::Framebuffer;
use lab2_graficas::game_of_life::GameOfLife;
use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const INITIAL_STEP_MS: u64 = 110;
const MIN_STEP_MS: u64 = 20;
const MAX_STEP_MS: u64 = 500;
const SPEED_STEP_MS: u64 = 20;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mut game = GameOfLife::new(WIDTH, HEIGHT);
    let mut paused = false;
    let mut step_duration = Duration::from_millis(INITIAL_STEP_MS);
    let mut last_step = Instant::now();

    let mut window = Window::new(
        "Laboratorio 2 - Conway's Game of Life",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X8,
            resize: false,
            ..WindowOptions::default()
        },
    )?;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        handle_input(
            &window,
            &mut game,
            &mut paused,
            &mut step_duration,
            &mut last_step,
        );

        if !paused && last_step.elapsed() >= step_duration {
            game.step();
            last_step = Instant::now();
        }

        game.render(&mut framebuffer);
        window.update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)?;
    }

    Ok(())
}

fn handle_input(
    window: &Window,
    game: &mut GameOfLife,
    paused: &mut bool,
    step_duration: &mut Duration,
    last_step: &mut Instant,
) {
    if window.is_key_pressed(Key::Space, KeyRepeat::No) {
        *paused = !*paused;
    }

    if window.is_key_pressed(Key::R, KeyRepeat::No) {
        game.reset();
        *last_step = Instant::now();
    }

    if *paused && window.is_key_pressed(Key::N, KeyRepeat::No) {
        game.step();
        *last_step = Instant::now();
    }

    if window.is_key_pressed(Key::Up, KeyRepeat::No) {
        let millis = step_duration.as_millis() as u64;
        *step_duration =
            Duration::from_millis(millis.saturating_sub(SPEED_STEP_MS).max(MIN_STEP_MS));
    }

    if window.is_key_pressed(Key::Down, KeyRepeat::No) {
        let millis = step_duration.as_millis() as u64;
        *step_duration = Duration::from_millis((millis + SPEED_STEP_MS).min(MAX_STEP_MS));
    }
}
