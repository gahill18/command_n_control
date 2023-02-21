// creating the world and how units interact with it

use crate::{
    units::{Civilian, Soldier, Team},
    Distance, MyResult,
};

#[derive(Default, Debug, Clone)]
pub struct Cell {
    soldier: Option<Soldier>,
    civilian: Option<Civilian>,
}

impl Cell {
    pub fn default() -> Self {
        Cell::new()
    }
    pub fn new() -> Self {
        let (soldier, civilian) = (None, None);
        Cell { soldier, civilian }
    }

    pub fn soldier_full(&self) -> bool {
        self.soldier.is_some()
    }
    pub fn civilian_full(&self) -> bool {
        self.civilian.is_some()
    }

    pub fn add_soldier(&mut self, soldier: Soldier) -> MyResult<()> {
        if !self.soldier_full() {
            self.soldier = Some(soldier);
            Ok(())
        } else {
            Err(format!("cell occupied!: {:?}", self))
        }
    }
    pub fn add_civilian(&mut self, civilian: Civilian) -> MyResult<()> {
        if !self.civilian_full() {
            self.civilian = Some(civilian);
            Ok(())
        } else {
            Err(format!("cell occupied: {:?}", self))
        }
    }
    pub fn mut_add_soldier(&mut self, soldier: &mut Soldier) -> MyResult<()> {
        if !self.soldier_full() {
            self.soldier = Some(soldier.to_owned());
            Ok(())
        } else {
            Err(format!("cell occupied!: {:?}", self))
        }
    }
    pub fn mut_add_civilian(&mut self, civilian: &mut Civilian) -> MyResult<()> {
        if !self.civilian_full() {
            self.civilian = Some(civilian.to_owned());
            Ok(())
        } else {
            Err(format!("cell occupied: {:?}", self))
        }
    }

    fn allegience(&self) -> Option<Team> {
        if let Some(soldier) = self.soldier() {
            Some(soldier.allegience())
        } else if let Some(civilian) = self.civilian() {
            Some(civilian.allegience())
        } else {
            None
        }
    }

    pub fn soldier(&self) -> Option<&Soldier> {
        self.soldier.as_ref()
    }

    pub fn civilian(&self) -> Option<&Civilian> {
        self.civilian.as_ref()
    }
}

#[derive(Default, Debug, Clone)]
pub struct Map {
    size_x: Distance,
    size_y: Distance,
    grid: Vec<Vec<Cell>>,
}

impl Map {
    pub fn default() -> Self {
        Map::new(10, 10)
    }
    pub fn new(size_x: Distance, size_y: Distance) -> Self {
        let empty_cell = Cell::default();
        let grid = vec![vec![empty_cell; size_x]; size_y];
        Map {
            size_x,
            size_y,
            grid,
        }
    }
    pub fn cell_at(&self, x: Distance, y: Distance) -> Cell {
        self.grid[x][y].to_owned()
    }
    pub fn add_soldier(&mut self, soldier: Soldier, coords: (Distance, Distance)) -> MyResult<()> {
        let (x, y) = coords;
        self.grid[x][y].add_soldier(soldier)
    }

    pub fn add_civilian(
        &mut self,
        civilian: Civilian,
        coords: (Distance, Distance),
    ) -> MyResult<()> {
        let (x, y) = coords;
        self.grid[x][y].add_civilian(civilian)
    }

    pub fn init_player(&mut self, team: Team, starting_cell: (Distance, Distance)) -> MyResult<()> {
        let starting_soldier = Soldier::teamed_default(team);
        let starting_civilian = Civilian::teamed_default(team);
        self.add_soldier(starting_soldier, starting_cell)?;
        self.add_civilian(starting_civilian, starting_cell)?;
        Ok(())
    }

    pub fn live(self) -> bool {
        true
    }
}

// Rendering

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tile_char = match (self.soldier_full(), self.civilian_full()) {
            (true, true) => 'F',
            (true, false) => 'S',
            (false, true) => 'C',
            _ => '_',
        };
        let boundary_char = match self.allegience() {
            Some(Team::Alliance) => ('[', ']'),
            Some(Team::Axis) => ('<', '>'),
            None => ('_', '_'),
        };
        write!(f, " {}{}{} ", boundary_char.0, tile_char, boundary_char.1)
    }
}
impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.size_x {
            for y in (0..self.size_y).rev() {
                write!(f, "{}", self.cell_at(y, x))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
