use std::{io::Read, cmp};
use crossterm::{Screen, cursor};

fn gen(n: usize) -> Vec<Vec<bool>> {
    (0..n).map(|_| (0..n).map(|_| rand::random::<f32>() < 0.3).collect()).collect()
}

fn num_alive_neighbours(old: &Vec<Vec<bool>>, x: usize, y: usize) -> u8 {
    let mut n = 0;
    let max : usize = old.len();
    for yp in (cmp::max(y, 1)-1)..cmp::min(y+1,max) {
        for xp in (cmp::max(x, 1)-1)..cmp::min(x+1,max) {
            if old[xp][yp] {
                n+=1;
            }
        }
    }
    n
}

fn next(old: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let max : usize = old.len();
    (0..max).map(|y|
                (0..max).map(|x| {
                    let num_alive = num_alive_neighbours(&old, x, y);
                    (old[x][y] && num_alive == 2) || num_alive == 3
                }).collect()
    ).collect()
}

fn print(population: &Vec<Vec<bool>>) {
    for row in population.iter() {
        for cell in row.iter() {
            if *cell {
                print!("🀫")
            } else {
                print!("🀆")
            }
        }
        println!("");
    }
}

fn main() {
    let screen = Screen::default();
    let mut cursor = cursor(&screen);

    let mut population = gen(20);
    loop {
        print(&population);
        std::io::stdin().bytes().next();
        population = next(population);

        cursor.move_up(population.len() as u16 + 1);
    };
}
