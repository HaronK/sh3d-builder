#[derive(Serialize, Deserialize, Debug)]
pub struct Home {
    pub rooms: Vec<Room>,
    #[serde(default)]
    pub connections: Vec<RoomConnection>,
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
    CW,
    CCW,
}

impl Default for WallOrientation {
    fn default() -> Self {
        WallOrientation::CW
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wall {
    pub direction: Direction,
    pub length: f32,
    pub thickness: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Left
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomConnection {
    pub room1: ConnectionInfo,
    pub room2: ConnectionInfo,
    pub conn_type: ConnectionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionInfo {
    pub name: String,
    /// Index of the coinciding wall (starts from 1)
    pub wall_index: usize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum ConnectionType {
    /// Walls coincide
    Coincide,
    /// Start of the first wall coincide with the end of the second one
    StartToEnd,
    /// End of the first wall coincide with the start of the second one
    EndToStart,
}
