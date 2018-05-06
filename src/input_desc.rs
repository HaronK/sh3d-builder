#[derive(Serialize, Deserialize, Debug)]
pub struct Home {
    rooms: Vec<Room>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    name: String,
    walls: Vec<Wall>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wall {
    wall_type: WallType,
    direction: Direction,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WallType {
    Line { length: f32 },
    Circle { radius: f32 },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Direction {
    Left,
    Right,
    Top,
    Bottom,
    Undefined,
}
