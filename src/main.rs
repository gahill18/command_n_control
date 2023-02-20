use crate::map::Map;
use crate::units::*;

pub mod map;
pub mod units;

type MyResult<T> = Result<T, String>;

// initialize the game
fn init_game(size_x: Distance, size_y: Distance) -> MyResult<Map> {
    let mut grid = Map::new(size_x, size_y);
    let start_coords = (1, 2);
    let snd_coords = (3, 4);
    grid.init_player(Team::Alliance, start_coords)?;
    grid.init_player(Team::Axis, snd_coords)?;
    Ok(grid)
}

fn main() -> MyResult<()> {
    let board = init_game(5, 5)?;
    println!("{}", board);
    Ok(())
}
