use input_desc::{Direction, WallOrientation};

#[derive(Serialize, Deserialize, Debug)]
pub struct Home {
    #[serde(skip)]
    pub orientation: WallOrientation,
    pub rooms: Vec<Room>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    pub name: String,
    pub walls: Vec<Wall>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wall {
    pub start: Point,
    pub end: Point,
    pub thickness: f32,
    #[serde(skip)]
    pub direction: Direction,
    #[serde(skip)]
    pub length: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
