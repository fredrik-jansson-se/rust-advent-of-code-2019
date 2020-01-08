use crate::intcode::{State, CPU};
use minifb::{Key, Window, WindowOptions};
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day13.txt").unwrap();
    println!("13:1 {}", run_1(&input));
    println!("13:2 {}", run_2(&input));
}

fn run_1(input: &str) -> usize {
    let mut cpu = CPU::new(input);

    let mut input = Vec::new();

    let mut blocks = 0;
    while cpu.run(&mut input) != State::Exited {}
    for c in cpu.output.chunks(3) {
        if c[2] == 2 {
            blocks += 1;
        }
    }

    blocks
}

const MULT: usize = 8;
const WIDTH: usize = 36 * MULT;
const HEIGHT: usize = 28 * MULT;

fn draw(video: &mut [u32], x: i64, y: i64, color: u32) {
    let dx = MULT * (x as usize);
    let dy = MULT * (y as usize);
    for _y in dy..(dy + MULT) {
        for _x in dx..(dx + MULT) {
            video[_y * WIDTH + _x] = color;
        }
    }
}

fn run_2(input: &str) -> i64 {
    let mut cpu = CPU::new(input);

    // play for free
    cpu.memory[0] = 2;

    let mut input = Vec::new();
    let mut score = 0;

    let mut video = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("Aoc13-2", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let mut ball_position = 10;
    let mut paddle_position = 0;
    while window.is_open() {
        let res = cpu.run(&mut input);

        for c in cpu.output.chunks(3) {
            let x = c[0];
            let y = c[1];

            match c[2] {
                s if x == -1 && y == 0 => {
                    score = s;
                    // println!("Score: {}", c[2]);
                }
                0 => draw(&mut video, x, y, 0x01010101),
                1 => draw(&mut video, x, y, 0xff000000),
                2 => draw(&mut video, x, y, 0x00ff0000),
                3 => {
                    draw(&mut video, x, y, 0xff00ffff);
                    paddle_position = x;
                }
                4 => {
                    draw(&mut video, x, y, 0xffffffff);
                    ball_position = x;
                }
                _ => (),
            }
        }

        if res == State::Exited {
            break;
        }

        if ball_position > paddle_position {
            // move right
            input.push(1);
        } else if ball_position < paddle_position {
            // move left
            input.push(-1);
        } else {
            input.push(0);
        }

        window
            .update_with_buffer_size(&video, WIDTH, HEIGHT)
            .unwrap();
        cpu.output.clear();
    }

    score
}

#[cfg(test)]
mod tests {

    #[test]
    fn aoc13_1() {
        //
    }

    #[test]
    fn aoc13_2() {
        //
    }
}
