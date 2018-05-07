use error::Result;
use failure::ResultExt;
use input_desc::{self, ConnectionType, Direction, RoomConnection};
use output_desc::{self, Point};

#[derive(Fail, Debug)]
pub enum BuildError {
    #[fail(display = "Room connections are not specified")]
    NoRoomConnections,
    #[fail(display = "Room '{}' should have more than 2 walls but got {}", _0, _1)]
    RoomWithoutWalls(String, usize),
    #[fail(display = "Walls cannot go in opposite direction: {:?} - {:?}", _0, _1)]
    OppositeDirectionWalls(Direction, Direction),
    #[fail(display = "Room '{}' is not closed by the walls", _0)]
    RoomNotClosed(String),
    #[fail(display = "There is no room with name '{}'", _0)]
    NoRoom(String),
    #[fail(
        display = "Wrong wall index for room '{}'. It should be between 1 an {} but was {}",
        _0,
        _1,
        _2
    )]
    WrongWallIndex(String, usize, usize),
    #[fail(display = "Cannot connect room '{}' wall {} with room '{}' wall {}", _0, _1, _2, _3)]
    WallsCannotConnect(String, usize, String, usize),
    #[fail(
        display = "Room '{}' wall {} and room '{}' wall {} have different thickness", _0, _1, _2, _3
    )]
    DifferentThickness(String, usize, String, usize),
    #[fail(display = "Rooms {:?} are not connected to other", _0)]
    RoomsNotConnected(Vec<String>),
    #[fail(
        display = "Room '{}' wall {} start {:?} doesn't coincide with room '{}' wall {} end {:?}",
        _0,
        _1,
        _2,
        _3,
        _4,
        _5
    )]
    WallsStartEndNotCoincide(String, usize, Point, String, usize, Point),
}

pub fn build(input_home: input_desc::Home) -> Result<output_desc::Home> {
    if input_home.rooms.len() > 1 && input_home.connections.len() == 0 {
        return Err(BuildError::NoRoomConnections.into());
    }

    let mut output_home = build_output_rooms(&input_home)?;

    if input_home.rooms.len() > 1 {
        connect_rooms(&mut output_home, &input_home.connections)?;
    }

    Ok(output_home)
}

fn build_output_rooms(input_home: &input_desc::Home) -> Result<output_desc::Home> {
    let mut output_home = output_desc::Home {
        orientation: input_home.orientation,
        rooms: vec![],
    };

    let input_rooms = &input_home.rooms;
    for input_room in input_rooms {
        let walls_count = input_room.walls.len();
        if walls_count < 3 {
            return Err(BuildError::RoomWithoutWalls(input_room.name.clone(), walls_count).into());
        }

        let mut output_room = output_desc::Room {
            name: input_room.name.clone(),
            walls: vec![],
        };

        let origin = Point { x: 0.0, y: 0.0 };
        let mut cur_point = Point { x: 0.0, y: 0.0 };
        let mut prev_wall = &input_room.walls[walls_count - 1];
        for i in 0..walls_count {
            let cur_wall = &input_room.walls[i];
            let next_wall = &input_room.walls[(i + 1) % walls_count];

            let start_point =
                calc_wall_point(&cur_point, prev_wall, cur_wall, input_home.orientation).context(
                    format!(
                        "Room {:?}, wall {} and previous one. Error: ",
                        input_room.name, i
                    ),
                )?;

            match cur_wall.direction {
                Direction::Left => cur_point.x -= cur_wall.length,
                Direction::Right => cur_point.x += cur_wall.length,
                Direction::Up => cur_point.y -= cur_wall.length,
                Direction::Down => cur_point.y += cur_wall.length,
            }

            let end_point =
                calc_wall_point(&cur_point, cur_wall, next_wall, input_home.orientation).context(
                    format!(
                        "Room {:?}, wall {} and next one. Error: ",
                        input_room.name, i
                    ),
                )?;

            output_room.walls.push(output_desc::Wall {
                start: start_point,
                end: end_point,
                thickness: cur_wall.thickness,
                direction: cur_wall.direction,
                length: cur_wall.length,
            });

            prev_wall = cur_wall;
        }

        output_home.rooms.push(output_room);

        if origin != cur_point {
            return Err(BuildError::RoomNotClosed(input_room.name.clone()).into());
        }
    }

    Ok(output_home)
}

fn calc_wall_point(
    point: &Point,
    wall1: &input_desc::Wall,
    wall2: &input_desc::Wall,
    orientation: input_desc::WallOrientation,
) -> Result<Point> {
    if wall1.direction == wall2.direction {
        return Ok(Point {
            x: point.x,
            y: point.y,
        });
    }

    let dir = if orientation == input_desc::WallOrientation::CCW {
        -1.0
    } else {
        1.0
    };
    let half_thick1 = wall1.thickness / 2.0;
    let half_thick2 = wall2.thickness / 2.0;

    let (off_x, off_y) = match wall1.direction {
        Direction::Left => {
            if wall2.direction == Direction::Right {
                return Err(
                    BuildError::OppositeDirectionWalls(wall1.direction, wall2.direction).into(),
                );
            } else if wall2.direction == Direction::Up {
                (-half_thick2, half_thick1)
            } else {
                // wall2.direction == Direction::Down
                (half_thick2, half_thick1)
            }
        }
        Direction::Right => {
            if wall2.direction == Direction::Left {
                return Err(
                    BuildError::OppositeDirectionWalls(wall1.direction, wall2.direction).into(),
                );
            } else if wall2.direction == Direction::Up {
                (-half_thick2, -half_thick1)
            } else {
                // wall2.direction == Direction::Down
                (half_thick2, -half_thick1)
            }
        }
        Direction::Up => {
            if wall2.direction == Direction::Down {
                return Err(
                    BuildError::OppositeDirectionWalls(wall1.direction, wall2.direction).into(),
                );
            } else if wall2.direction == Direction::Left {
                (half_thick1, -half_thick2)
            } else {
                // wall2.direction == Direction::Right
                (-half_thick1, -half_thick2)
            }
        }
        Direction::Down => {
            if wall2.direction == Direction::Up {
                return Err(
                    BuildError::OppositeDirectionWalls(wall1.direction, wall2.direction).into(),
                );
            } else if wall2.direction == Direction::Left {
                (half_thick1, half_thick2)
            } else {
                // wall2.direction == Direction::Right
                (half_thick1, -half_thick2)
            }
        }
    };

    Ok(Point {
        x: point.x + off_x * dir,
        y: point.y + off_y * dir,
    })
}

fn connect_rooms(
    output_home: &mut output_desc::Home,
    connections: &Vec<RoomConnection>,
) -> Result<()> {
    let mut unconnected_rooms: Vec<_> = output_home.rooms.iter().map(|r| r.name.clone()).collect();
    let mut connected_rooms = vec![(unconnected_rooms.remove(0), 0.0, 0.0)];
    let mut connected_index = 0;

    // search for connections between rooms from connected and unconnected lists
    while connected_index < connected_rooms.len() {
        let (room1_name, conn_off_x, conn_off_y) = connected_rooms[connected_index].clone();

        let active_connections: Vec<_> = connections
            .iter()
            .filter(|conn| {
                (conn.room1.name == *room1_name && unconnected_rooms.contains(&conn.room2.name))
                    || (conn.room2.name == *room1_name
                        && unconnected_rooms.contains(&conn.room1.name))
            })
            .collect();

        for conn in active_connections {
            let mut connected_room_info = &conn.room1;
            let mut unconnected_room_info = &conn.room2;

            if conn.room2.name == *room1_name {
                connected_room_info = &conn.room2;
                unconnected_room_info = &conn.room1;
            }

            let connected_room = output_home
                .rooms
                .iter()
                .find(|room| room.name == connected_room_info.name)
                .ok_or_else(|| BuildError::NoRoom(connected_room_info.name.clone()))?;

            let unconnected_room = output_home
                .rooms
                .iter()
                .find(|room| room.name == unconnected_room_info.name)
                .ok_or_else(|| BuildError::NoRoom(unconnected_room_info.name.clone()))?;

            let wall1 = get_wall_by_index(connected_room, connected_room_info.wall_index)?;
            let wall2 = get_wall_by_index(unconnected_room, unconnected_room_info.wall_index)?;

            if !check_walls_can_connect(wall1, wall2) {
                return Err(BuildError::WallsCannotConnect(
                    connected_room.name.clone(),
                    connected_room_info.wall_index,
                    unconnected_room.name.clone(),
                    unconnected_room_info.wall_index,
                ).into());
            }

            if wall1.thickness != wall2.thickness {
                return Err(BuildError::DifferentThickness(
                    connected_room.name.clone(),
                    connected_room_info.wall_index,
                    unconnected_room.name.clone(),
                    unconnected_room_info.wall_index,
                ).into());
            }

            let (off_x, off_y) = match conn.conn_type {
                ConnectionType::Coincide | ConnectionType::StartToEnd => {
                    (wall1.start.x - wall2.end.x, wall1.start.y - wall2.end.y)
                }
                ConnectionType::EndToStart => {
                    (wall1.end.x - wall2.start.x, wall1.end.y - wall2.start.y)
                }
            };

            connected_rooms.push((
                unconnected_room_info.name.clone(),
                off_x + conn_off_x,
                off_y + conn_off_y,
            ));
            unconnected_rooms.retain(|ref r| **r != unconnected_room_info.name);
        }

        connected_index += 1;
    }

    if !unconnected_rooms.is_empty() {
        return Err(BuildError::RoomsNotConnected(unconnected_rooms).into());
    }

    // shift rooms
    output_home.rooms.iter_mut().for_each(|room| {
        let (_, off_x, off_y) = connected_rooms.iter().find(|r| r.0 == room.name).unwrap();

        room.walls.iter_mut().for_each(|wall| {
            wall.start.x += off_x;
            wall.start.y += off_y;
            wall.end.x += off_x;
            wall.end.y += off_y;
        });
    });

    verify_room_connections(output_home, connections)
}

fn check_walls_can_connect(wall1: &output_desc::Wall, wall2: &output_desc::Wall) -> bool {
    (wall1.direction == Direction::Left && wall2.direction == Direction::Right)
        || (wall1.direction == Direction::Right && wall2.direction == Direction::Left)
        || (wall1.direction == Direction::Up && wall2.direction == Direction::Down)
        || (wall1.direction == Direction::Down && wall2.direction == Direction::Up)
}

fn get_wall_by_index(room: &output_desc::Room, index: usize) -> Result<&output_desc::Wall> {
    if index == 0 && index > room.walls.len() {
        return Err(BuildError::WrongWallIndex(room.name.clone(), room.walls.len(), index).into());
    }

    Ok(&room.walls[index - 1])
}

fn verify_room_connections(
    output_home: &output_desc::Home,
    connections: &Vec<RoomConnection>,
) -> Result<()> {
    for conn in connections {
        let room1 = &output_home
            .rooms
            .iter()
            .find(|room| room.name == conn.room1.name)
            .ok_or_else(|| BuildError::NoRoom(conn.room1.name.clone()))?;
        let wall1 = get_wall_by_index(room1, conn.room1.wall_index)?;
        let room2 = &output_home
            .rooms
            .iter()
            .find(|room| room.name == conn.room2.name)
            .ok_or_else(|| BuildError::NoRoom(conn.room2.name.clone()))?;
        let wall2 = get_wall_by_index(room2, conn.room2.wall_index)?;

        if conn.conn_type == ConnectionType::Coincide
            || conn.conn_type == ConnectionType::StartToEnd
        {
            if wall1.start != wall2.end {
                return Err(BuildError::WallsStartEndNotCoincide(
                    room1.name.clone(),
                    conn.room1.wall_index,
                    wall1.start.clone(),
                    room2.name.clone(),
                    conn.room2.wall_index,
                    wall2.end.clone(),
                ).into());
            }
        }
        if conn.conn_type == ConnectionType::Coincide
            || conn.conn_type == ConnectionType::EndToStart
        {
            if wall2.start != wall1.end {
                return Err(BuildError::WallsStartEndNotCoincide(
                    room2.name.clone(),
                    conn.room2.wall_index,
                    wall2.start.clone(),
                    room1.name.clone(),
                    conn.room1.wall_index,
                    wall1.end.clone(),
                ).into());
            }
        }

        if !check_walls_can_connect(wall1, wall2) {
            return Err(BuildError::WallsCannotConnect(
                room1.name.clone(),
                conn.room1.wall_index,
                room2.name.clone(),
                conn.room2.wall_index,
            ).into());
        }

        if wall1.thickness != wall2.thickness {
            return Err(BuildError::DifferentThickness(
                room1.name.clone(),
                conn.room1.wall_index,
                room2.name.clone(),
                conn.room2.wall_index,
            ).into());
        }
    }

    Ok(())
}
