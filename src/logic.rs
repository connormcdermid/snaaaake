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

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "", // TODO: Your Battlesnake Username
        "color": "#888888", // TODO: Choose color
        "head": "default", // TODO: Choose head
        "tail": "default", // TODO: Choose tail
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
        } else if cell.x == my_head.x + 1 && cell.y == my_head.y {
            map.insert("right", false); //body directly to right of head
        }
        if cell.y == my_head.y - 1 && cell.x == my_head.x {
            map.insert("down", false);
        } else if cell.y == my_head.y + 1 && cell.x == my_head.x {
            map.insert("up", false);
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

    } else if my_neck.x > my_head.x { // Neck is right of head, don't move right
        is_move_safe.insert("right", false);

    } else if my_neck.y < my_head.y { // Neck is below head, don't move down
        is_move_safe.insert("down", false);
    
    } else if my_neck.y > my_head.y { // Neck is above head, don't move up
        is_move_safe.insert("up", false);
    }

    // TODO: Step 1 - Prevent your Battlesnake from moving out of bounds
    // let board_width = &board.width;
    // let board_height = &board.height;
    let board_width = board.width;
    let board_height = board.height;
    if my_head.x == 0 {
        is_move_safe.insert("left", false);
    }

    if my_head.x == board_height - 1 {
        is_move_safe.insert("right", false);
    }

    if my_head.y == 0 {
        is_move_safe.insert("down", false);
    }

    if my_head.y == board_height - 1 {
        is_move_safe.insert("up", false);
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

    /*
    
    let good_moves = safe_moves.clone();

    let food = (board.food[0].x, board.food[0].y);

    impl Coord {
        fn distance(&self, other: &Coord) -> u32 {
            (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32
        }

        fn successors(&self) -> Vec<(Coord, u32)> {
            let &Coord(x, y) = self;
            vec![Coord(x+1, y), Coord(x-1, y), Coord(x, y+1), Coord(x, y-1)].into_iter().map(|p|, (p, 1)).collect()
        }
    }

    possible_foods = board.food;

    */



    // Choose a random move from the safe ones
    let chosen = safe_moves.choose(&mut rand::thread_rng()).unwrap_or_else(|| {panic!("width: {}, height: {}, head_x: {}, head_y: {}",
     board_width, board_height, my_head.x, my_head.y)});

    // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
    // let food = &board.food;

    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen });
}
