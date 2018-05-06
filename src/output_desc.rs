#[derive(Serialize, Deserialize, Debug)]
pub struct Home {
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
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
