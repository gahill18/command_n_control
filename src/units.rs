use crate::{map::Cell, Distance, MyResult};

// For moveable/controlable units
pub type Name = String;
pub type Health = i32;
pub type Strength = i32;

#[derive(Default, Debug, Clone, Copy)]
pub enum DmgType {
    #[default]
    Melee,
    Ranged(Distance),
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Team {
    #[default]
    Alliance,
    Axis,
}

#[derive(Default, Debug, Clone)]
pub struct Soldier {
    name: Name,
    hp: Health,
    atk: Strength,
    def: Strength,
    dmg_type: DmgType,
    moves: Distance,
    sight: Distance,
    team: Team,
}

impl Soldier {
    pub fn teamed_default(team: Team) -> Self {
        Soldier::new(
            String::from("Soldier"), // name
            10,                      // hp
            5,                       // atk
            3,                       // def
            DmgType::Melee,          // dmg_type
            2,                       // moves
            2,                       // sight
            team,                    // team
        )
    }
    pub fn default() -> Self {
        Soldier::new(
            String::from("Soldier"), // name
            10,                      // hp
            5,                       // atk
            3,                       // def
            DmgType::Melee,          // dmg_type
            2,                       // moves
            2,                       // sight
            Team::Alliance,          // team
        )
    }

    pub fn new(
        name: String,
        hp: Health,
        atk: Strength,
        def: Strength,
        dmg_type: DmgType,
        moves: Distance,
        sight: Distance,
        team: Team,
    ) -> Self {
        Soldier {
            name,
            hp,
            atk,
            def,
            dmg_type,
            moves,
            sight,
            team,
        }
    }

    pub fn allegience(&self) -> Team {
        self.team
    }
}

#[derive(Default, Debug, Clone)]
pub struct Civilian {
    name: Name,
    hp: Health,
    moves: Distance,
    sight: Distance,
    team: Team,
}

impl Civilian {
    pub fn teamed_default(team: Team) -> Self {
        let (name, hp, moves, sight) = (String::from("Civilian"), 1, 2, 1);
        Civilian::new(name, hp, moves, sight, team)
    }
    pub fn default() -> Self {
        let (name, hp, moves, sight, team) = (String::from("Civilian"), 1, 2, 1, Team::Alliance);
        Civilian::new(name, hp, moves, sight, team)
    }

    pub fn new(name: String, hp: Health, moves: Distance, sight: Distance, team: Team) -> Self {
        Self {
            name,
            hp,
            moves,
            sight,
            team,
        }
    }

    pub fn allegience(&self) -> Team {
        self.team
    }
}

// logic for how long the unit sticks around for
trait Alive {
    fn health(self) -> Health;
    fn take_damage(&mut self, dmg: Health) -> Health;
    fn die(&mut self) -> ();
}

impl Alive for Soldier {
    fn take_damage(&mut self, dmg: Health) -> Health {
        let dmg_dealt = dmg.clamp(0, self.hp);
        if dmg_dealt >= self.hp {
            self.die();
        }
        dmg_dealt
    }

    fn die(&mut self) -> () {
        println!("{:?} died", self.name)
    }

    fn health(self) -> Health {
        self.hp
    }
}

impl Alive for Civilian {
    fn take_damage(&mut self, dmg: Health) -> Health {
        let dmg_dealt = dmg.clamp(0, self.hp);
        if dmg_dealt >= self.hp {
            self.die();
        }
        dmg_dealt
    }

    fn die(&mut self) -> () {
        println!("{:?} died", self.name)
    }

    fn health(self) -> Health {
        self.hp
    }
}
// logic for moving between cells
trait Moveable {
    fn moves(self) -> Distance;
    fn move_to(&mut self, target: &mut Cell) -> MyResult<()>;
}

impl Moveable for Soldier {
    fn moves(self) -> Distance {
        self.moves
    }

    fn move_to(&mut self, target: &mut Cell) -> MyResult<()> {
        target.mut_add_soldier(self)
    }
}

impl Moveable for Civilian {
    fn moves(self) -> Distance {
        self.moves
    }

    fn move_to(&mut self, target: &mut Cell) -> MyResult<()> {
        target.mut_add_civilian(self)
    }
}

pub enum Command {}
