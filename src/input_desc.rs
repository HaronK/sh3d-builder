#[derive(Serialize, Deserialize, Debug)]
pub struct Home {
    pub rooms: Vec<Room>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    pub name: String,
    pub orientation: WallOrientation,
    pub walls: Vec<Wall>,
}

/// Specify in what order room's walls are described relatively to the room's center.
#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum WallOrientation {
    CCW,
    CW,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wall {
    pub direction: Direction,
    pub length: f32,
    pub thickness: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WallType {
    Line { length: f32 },
    Circle { radius: f32 },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
