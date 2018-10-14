use std::{cmp, thread, time};
use crossterm::{Screen, cursor};

fn gen(n: usize) -> Vec<Vec<bool>> {
    (0..n).map(|_| (0..n).map(|_| rand::random::<f32>() < 0.3).collect()).collect()
}

fn num_alive_neighbours(old: &Vec<Vec<bool>>, x: usize, y: usize) -> u8 {
    let mut n = 0;
    let max : usize = old.len();
    for yp in (cmp::max(y, 1)-1)..=cmp::min(y+1,max-1) {
        for xp in (cmp::max(x, 1)-1)..=cmp::min(x+1,max-1) {
            if old[xp][yp] && (xp != x || yp != y) {
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

fn conway_string(population: &Vec<Vec<bool>>) -> String {
    population.iter().map(|row|
        row.iter().map(|cell| if *cell { "🀫" } else { "🀆" }).collect::<Vec<&str>>().join("")
    ).collect::<Vec<String>>().join("\n") +"\n"
}

fn main() {
    let screen = Screen::default();
    let mut cursor = cursor(&screen);
    let wait_duration = time::Duration::from_millis(300);

    let mut population = gen(20);
    loop {
        print!("{}", conway_string(&population));
        thread::sleep(wait_duration);
        population = next(population);

        cursor.move_up(population.len() as u16);
    };
}

#[test]
fn num_alive_when_all_are_true() {
    let pop = vec![ vec![true, true, true], vec![true, true, true], vec![true, true, true] ];
    assert_eq!(num_alive_neighbours(&pop, 0, 0), 3);
    assert_eq!(num_alive_neighbours(&pop, 2, 2), 3);
    assert_eq!(num_alive_neighbours(&pop, 1, 1), 8);
    assert_eq!(num_alive_neighbours(&pop, 1, 0), 5);
}

#[test]
fn num_alive_when_all_are_false() {
    let pop = vec![ vec![false, false, false], vec![false, false, false], vec![false, false, false] ];
    assert_eq!(num_alive_neighbours(&pop, 0, 0), 0);
    assert_eq!(num_alive_neighbours(&pop, 2, 2), 0);
    assert_eq!(num_alive_neighbours(&pop, 1, 1), 0);
    assert_eq!(num_alive_neighbours(&pop, 1, 0), 0);
}
