use std::{io::Read, cmp};
use crossterm::{Screen, cursor};

fn gen() -> [[bool; 10]; 10] {
	let mut population: [[bool; 10]; 10] = [[false; 10]; 10];;
	for y in 0..10 {
		for x in 0..10 {
			if rand::random::<f32>() < 0.3 {
				population[x][y] = true;
			}
		}
	}
	population
}

fn num_alive_neighbours(old: [[bool; 10]; 10], x: usize, y: usize) -> u8 {
    let mut n = 0;
    for yp in (cmp::max(y, 1)-1)..=cmp::min(y+1,10-1) {
        for xp in (cmp::max(x, 1)-1)..=cmp::min(x+1,10-1) {
            if old[xp][yp] {
                n+=1;
            }
        }
    }
    n
}

fn next(old: [[bool; 10]; 10]) -> [[bool; 10]; 10] {
	let mut population: [[bool; 10]; 10] = [[false; 10]; 10];;
	for y in 0..10 {
		for x in 0..10 {
			let num_alive = num_alive_neighbours(old, x, y);
			if (old[x][y] && num_alive == 2) || num_alive == 3 {
                            population[x][y] = true;
                        }
		}
	}
	population
}

fn print(population: [[bool; 10]; 10]) {
	for (_y, row) in population.iter().enumerate() {
		for (_x, cell) in row.iter().enumerate() {
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

    let mut population = gen();
    loop {
        print(population);
        std::io::stdin().bytes().next();
        population = next(population);

        cursor.move_up(11);
    };
}
