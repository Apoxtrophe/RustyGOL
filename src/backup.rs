use std::arch::x86_64::_mm_pause;
use std::borrow::Borrow;
use std::cell::Cell;
use std::cmp::min;
use std::thread::sleep;
use std::time::Duration;
use minifb::*;
use ::grid::*;
use minifb::Key::ScrollLock;


use crate::functions::{create_world, draw_cell_black, draw_cell_white, draw_clear, sequel};
use crate::mini::minicock;

mod functions;
mod mini;


const WORLD_SIZE: usize = 500;
const CELL_SIZE: usize = 10;
const RAND_CHANCE: u8 = 2;

fn main(){
    game_of_life();
    //minicock();
}

pub fn game_of_life() {
    let mut test = 0;

    let mut tupled = create_world(WORLD_SIZE, RAND_CHANCE);
    let mut window2d = Window::new(
        "Game of Life",
        WORLD_SIZE,
        WORLD_SIZE,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Stretch,
            ..WindowOptions::default()
        },
    )
        .expect("Unable to create window");
    window2d.limit_update_rate(Some(Duration::from_millis(16)));
    while window2d.is_open() && !window2d.is_key_down(Key::Escape) {
        let mut buffer: Vec<u32> = Vec::with_capacity(WORLD_SIZE * WORLD_SIZE);
        tupled.1 = Grid::new(WORLD_SIZE, WORLD_SIZE);
        for row in 0..WORLD_SIZE {
            for col in 0..WORLD_SIZE {
                tupled = sequel(tupled.0, tupled.1, row, col, WORLD_SIZE);
                if tupled.0.get(row,col) == Some(&1){
                    buffer.push(	65280);
                }
                else{
                    buffer.push(0);
                }


            }
        }





        window2d
            .update_with_buffer(&buffer, WORLD_SIZE, WORLD_SIZE).unwrap();
        tupled.0 = tupled.1;
        test += 1;
        println!("{}", test);
        //let time = Duration::from_millis(100);
        //sleep(time);

    }
}
