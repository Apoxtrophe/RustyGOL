use rand::*;
use ::grid::*;
use piston_window::*;

pub fn create_world(world_size: usize, random_chance: u32) -> (Grid<u32>, Grid<u32>) {
    let mut world = Grid::new(world_size, world_size);
    let world_new = Grid::new(world_size, world_size);
    for row in 0..world_size {
        for col in 0..world_size {
            let mut rng = thread_rng();
            let rand: u32= rng.gen_range(0..=random_chance);
            *world.get_mut(row, col).unwrap() = rand;
        }
    }
    return (world, world_new);
}

pub fn sequel(world: Grid<u32>, mut world_new: Grid<u32>, row: usize, col: usize, world_size: usize) -> (Grid<u32>, Grid<u32>) {
    let mut neighbors = 0;
    if row == 0 || col == 0 {
        return (world, world_new);
    }
    if row == world_size - 1 || col == world_size - 1 {
        return (world, world_new);
    }
    //world.flatten();
    neighbors += world.get(row + 1, col + 1).unwrap();
    neighbors += world.get(row, col + 1).unwrap();
    neighbors += world.get(row - 1, col + 1).unwrap();
    neighbors += world.get(row + 1, col).unwrap();
    neighbors += world.get(row - 1, col).unwrap();
    neighbors += world.get(row + 1, col - 1).unwrap();
    neighbors += world.get(row, col - 1).unwrap();
    neighbors += world.get(row - 1, col - 1).unwrap();
    if (neighbors == 2 || neighbors == 3) && world.get(row, col) == Some(&1) {
        *world_new.get_mut(row, col).unwrap() = 1;
    }
    if neighbors == 3 && world.get(row, col) == Some(&0) {
        *world_new.get_mut(row, col).unwrap() = 1;
    }
    return (world, world_new);
}