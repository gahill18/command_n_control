use crate::map::Map;
use crate::units::{Command, Team};

pub mod map;
pub mod units;

type MyResult<T> = Result<T, String>;
pub type Distance = usize;

// initialize the game
fn init_game(size_x: Distance, size_y: Distance) -> MyResult<Map> {
    let mut grid = Map::new(size_x, size_y);
    let start_coords = (1, 2);
    let snd_coords = (3, 4);
    grid.init_player(Team::Alliance, start_coords)?;
    grid.init_player(Team::Axis, snd_coords)?;
    Ok(grid)
}

fn get_command() -> MyResult<Command> {
    todo!()
}

fn gameloop(board: Map) -> Option<Map> {
    let cmd = get_command();
    Some(board)
}

fn main() -> MyResult<()> {
    let mut board: Map = init_game(5, 5)?;
    let mut gameover = false;

    while !gameover {
        match gameloop(board.clone()) {
            Some(new_board) => board = new_board,
            None => gameover = true,
        }
    }

    Ok(())
}
