// Welcome to
// __________         __    __  .__                               __
// \______   \_____ _/  |__/  |_|  |   ____   ______ ____ _____  |  | __ ____
//  |    |  _/\__  \\   __\   __\  | _/ __ \ /  ___//    \\__  \ |  |/ // __ \
//  |    |   \ / __ \|  |  |  | |  |_\  ___/ \___ \|   |  \/ __ \|    <\  ___/
//  |________/(______/__|  |__| |____/\_____>______>___|__(______/__|__\\_____>
//
// This file can be a nice home for your Battlesnake logic and helper functions.
//
// To get you started we've included code to prevent your Battlesnake from moving backwards.
// For more info see docs.battlesnake.com

use log::info;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::HashMap;
use pathfinding::prelude::astar;

use crate::{Battlesnake, Board, Game};
use crate::Coord;

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "", // TODO: Your Battlesnake Username
        "color": "#ffd700", // TODO: Choose color
        "head": "all-seeing", // TODO: Choose head
        "tail": "cosmic-horror", // TODO: Choose tail
    });
}

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &i32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &i32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

/**
 * checks a snake for collisions with my head
 */
pub fn check_snake(you: &Battlesnake, opp: &Battlesnake, map: &mut HashMap<&str, bool>) {

    let my_head = &you.body[0];
    let snake = &opp.body;

    for cell in snake {
        if cell.x == my_head.x && cell.y == my_head.y {
            // this is my head, ignore it
            continue;
        }
        if cell.x == my_head.x - 1  && cell.y == my_head.y {
            //body is directly to left of head
            map.insert("left", false);
            print!("snake body at left!");
        } else if cell.x == my_head.x + 1 && cell.y == my_head.y {
            map.insert("right", false); //body directly to right of head
            print!("snake body at right!");
        }
        if cell.y == my_head.y - 1 && cell.x == my_head.x {
            map.insert("down", false);
            print!("snake body down!");
        } else if cell.y == my_head.y + 1 && cell.x == my_head.x {
            map.insert("up", false);
            print!("snake body up!");
        }
    }
}

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(game: &Game, turn: &i32, board: &Board, you: &Battlesnake) -> Value {
    
    let mut is_move_safe: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]
    .into_iter()
    .collect();

    // We've included code to prevent your Battlesnake from moving backwards
    let my_head = &you.body[0]; // Coordinates of your head
    let my_neck = &you.body[1]; // Coordinates of your "neck"
    
    if my_neck.x < my_head.x { // Neck is left of head, don't move left
        is_move_safe.insert("left", false);
        print!("neck left!");

    } else if my_neck.x > my_head.x { // Neck is right of head, don't move right
        is_move_safe.insert("right", false);
        print!("neck right!");

    } else if my_neck.y < my_head.y { // Neck is below head, don't move down
        is_move_safe.insert("down", false);
        print!("neck down!");
    
    } else if my_neck.y > my_head.y { // Neck is above head, don't move up
        is_move_safe.insert("up", false);
        print!("neck up!");
    }

    // TODO: Step 1 - Prevent your Battlesnake from moving out of bounds
    // let board_width = &board.width;
    // let board_height = &board.height;
    let board_width = board.width;
    let board_height = board.height;
    if my_head.x == 0 {
        is_move_safe.insert("left", false);
        print!("wall left!");
    }

    if my_head.x == board_height - 1 {
        is_move_safe.insert("right", false);
        print!("wall right!");
    }

    if my_head.y == 0 {
        is_move_safe.insert("down", false);
        print!("wall down!");
    }

    if my_head.y == board_height - 1 {
        is_move_safe.insert("up", false);
        print!("wall up!");
    }


    // TODO: Step 2 - Prevent your Battlesnake from colliding with itself
    // let my_body = &you.body;

        let my_body = &you.body;
        
        for cell in my_body {
            if cell.x == my_head.x && cell.y == my_head.y {
                // this is my head, ignore it
                continue;
            }
            if cell.x == my_head.x - 1 && cell.y == my_head.y {
                //body is directly to left of head
                is_move_safe.insert("left", false);
            } else if cell.x == my_head.x + 1 && cell.y == my_head.y {
                is_move_safe.insert("right", false); //body directly to right of head
            } else if cell.y == my_head.y - 1 && cell.x == my_head.x {
                is_move_safe.insert("down", false);
            } else if cell.y == my_head.y + 1 && cell.x == my_head.x {
                is_move_safe.insert("up", false);
            }
        }

    // TODO: Step 3 - Prevent your Battlesnake from colliding with other Battlesnakes
    // let opponents = &board.snakes;
    let opponents = &board.snakes;
    for snake in opponents {
        check_snake(you, snake, &mut is_move_safe);
    }

    // Are there any safe moves left?
    let safe_moves = is_move_safe
        .into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();

    impl Coord {
        fn distance(&self, other: &Coord) -> u32 {
            (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32
        }

        fn successors(&self) -> Vec<(Coord, u32)> {
            let &Coord{x, y} = self;
            vec![Coord{x: x+1, y}, Coord{x: x-1, y}, Coord{x, y: y+1}, Coord{x, y: y-1}].into_iter().map(|p| (p, 1)).collect()
        }
    }

    let possible_foods = board.food.clone();

    let mut solns = Vec::<u32>::new();

    let mut tuvec = Vec::<(Vec<Coord>, u32)>::new();

    for food in possible_foods {
        tuvec.push(astar(&Coord{x: my_head.x, y: my_head.y}, |p| p.successors(), |p| p.distance(&food), |p| *p == food).expect("no path found!"));
        solns.push(tuvec.last().expect("never get here").1);
        print!("astar coords: {}, {}\n", tuvec.last().expect("no path!").0[0].x, tuvec.last().expect("no path!").0[0].y);
    }
    let mut lowestval = u32::MAX;
    let mut iter = 0;
    let mut path = 0;
    for res in solns {
        let val = res;
        if (val < lowestval) {
            lowestval = val;
            path = iter;
        }
        iter = iter + 1;
    }

    let mut best_move: &str = "";

    // extract good moves
    if tuvec[path].0[1].x == my_head.x - 1 {
        best_move = "left";
    } else if tuvec[path].0[1].x == my_head.x + 1 {
        best_move = "right";
    } else  if tuvec[path].0[1].y == my_head.y - 1 {
        best_move = "down";
    } else if tuvec[path].0[1].y == my_head.y + 1 {
        best_move = "up";
    }

    for &mov in &safe_moves {
        if mov == best_move {
            info!("MOVE {}: {}", turn, best_move);
            return json!({ "move": best_move });
        }
    }

    // Choose a random move from the safe ones
    let chosen = safe_moves.choose(&mut rand::thread_rng()).unwrap_or_else(|| {panic!("width: {}, height: {}, head_x: {}, head_y: {}",
     board_width, board_height, my_head.x, my_head.y)});

    // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
    // let food = &board.food;

    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen });
}
