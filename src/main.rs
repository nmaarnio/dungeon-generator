#[derive(Copy, Clone, PartialEq)]
enum Cavestate {
    Ground = 0,
    Wall = 1
}

use std::{thread, time, io};
use rand::{prelude::*};
use Cavestate::*;

const NEIGHBORHOOD: [(i64, i64); 9] = [
    (-1,  1),
    ( 0,  1),
    ( 1,  1),
    (-1,  0),
    ( 0,  0),
    ( 1,  0),
    (-1, -1),
    ( 0, -1),
    ( 1, -1)
];

struct Grids {
    grid: Vec<Vec<Cavestate>>,
    new_grid: Vec<Vec<Cavestate>>,
    height: usize,
    width: usize
}

struct UpdateReturn {
    grids: Grids,
    updated: bool
}


fn main() {
    println!("Enter dungeon width: ");
    let dungeon_width = get_numeric_input();
    println!("Enter dungeon height: ");
    let dungeon_height = get_numeric_input();
    println!("Enter max generations: ");
    let max_generations = get_numeric_input();

    let grids = initialize_cave(dungeon_width, dungeon_height);
    cave_loop(grids, max_generations);
    println!("\nFinished, world!");
}


// fn ask_dungeon_size() -> DungeonSize {
//     loop {
//         let mut input_line: String;
//         println!("Enter dungeon size (x, y): ");
//         io::stdin().read_line(&mut input_line).unwrap();
//         let contents: Vec<&str> = input_line.split(&[',', ' ', ';']).collect();
//         for content in contents {
//             if !content.trim().parse::<usize>().is_ok() {
//                 println!("Incorrect input, try again");
//                 continue
//             }
//         }
//         return DungeonSize {
//             width: contents[0].trim().parse::<usize>().unwrap(),
//             height: contents[1].trim().parse::<usize>().unwrap()
//         }
//     }
// }


fn get_numeric_input()-> usize {
    loop {
        let mut input = String::new();

        // Reads the input from STDIN and places it in the String named input.
        io::stdin().read_line(&mut input)
            .expect("Failed to read input");

        // Convert to another type.
        // If successful, bind to a new variable named input.
        // If failed, restart the loop.
        let input = match input.trim().parse::<usize>() {
            Ok(parsed_input) => parsed_input,
            Err(_) => {
                println!("Could not parse input, try again:");
                continue
            },
        };
        return input;
    }
}


fn initialize_cave(width: usize, height: usize) -> Grids {
    let grid = vec![vec![Cavestate::Ground; width]; height];
    let new_grid = vec![vec![Cavestate::Ground; width]; height];
    let mut grids = Grids {
        grid,
        new_grid,
        height,
        width
    };

    grids = randomize_states(grids);
    print_cave(&grids, 0);
    grids
}


fn randomize_states(mut grids: Grids) -> Grids {
    let mut rng = rand::thread_rng();
    for i in 0..grids.height  {
        for j in 0..grids.width {
            let random_number: i16 = rng.gen_range(0..2);
            match random_number {
                0 => {
                    grids.grid[i][j] = Ground;
                    grids.new_grid[i][j] = Ground;
                },
                1 => {
                    grids.grid[i][j] = Wall;
                    grids.new_grid[i][j] = Wall;
                }
                _ => panic!("What is going on!")
            }
        }
    }
    grids
}


fn evolve(mut grids: Grids) -> Grids {

    for i in 0..grids.height  {
        for j in 0..grids.width {
            let mut walls = 0;
            for (dx, dy) in NEIGHBORHOOD {
                let x_coord = i as i64 + dx;
                let y_coord = j as i64 + dy;
                if x_coord < 0 || y_coord < 0 || 
                x_coord >= grids.height as i64 || y_coord >= grids.width as i64 {  // out of bounds
                    continue
                }
                match grids.grid[x_coord as usize][y_coord as usize] {
                    Wall => walls += 1,
                    _ => ()
                }
            }
            if walls > 4 {
                grids.new_grid[i][j] = Wall;
            }
            else {
                grids.new_grid[i][j] = Ground;
            }


            // if NEIGHBORHOOD.
            //     iter().
            //     map(|(dx, dy)| grid[x as i64 + dx][y as i64 + dy]).
            //     filter( |x| match x { Wall => true, _ => false }).
            //     count() as i32 >= 5 {
            //         new_grid[x][y] = Wall;
            // }
        }
    }
    grids
}


fn update(mut grids: Grids) -> UpdateReturn {
    let mut updated = false;
    for x in 0..grids.height  {
        for y in 0..grids.width {
            if !updated && grids.grid[x][y] != grids.new_grid[x][y] {
                updated = true
            }
            grids.grid[x][y] = grids.new_grid[x][y]
        }
    }
    UpdateReturn { grids, updated }
}


fn cave_loop(mut grids: Grids, max_generations: usize) {
    let mut finished = false;
    let mut generation = 0;

    while !finished && generation < max_generations {
        thread::sleep(time::Duration::from_secs(2));
        grids = evolve(grids);
        generation += 1;
        let update_return = update(grids);
        grids = update_return.grids;
        print_cave(&grids, generation);
        if !update_return.updated {
            finished = true
        }
    }
}


fn print_cave(grids: &Grids, generation: usize) {
    println!("\nGeneration {}", generation);
    for x in 0..grids.height  {
        let mut row = String::new();
        for y in 0..grids.width {
            match grids.grid[x][y] {
                Wall => row.push_str("#"),
                Ground => row.push_str("."),
            }
        }
        println!("{}", row);
    }
}

