use failure::{self, ResultExt};
use input_desc;
use output_desc;

#[derive(Fail, Debug)]
pub enum BuildError {
    #[fail(display = "Room '{}' should have more than 2 walls but got {}", _0, _1)]
    RoomWithoutWalls(String, usize),
    #[fail(display = "Walls cannot go in opposite direction: {:?} - {:?}", _0, _1)]
    OppositeDirectionWalls(input_desc::Direction, input_desc::Direction),
    #[fail(display = "Room '{}' is not closed by the walls", _0)]
    RoomNotClosed(String),
}

pub fn build(home: input_desc::Home) -> Result<output_desc::Home, failure::Error> {
    let mut result = output_desc::Home { rooms: vec![] };

    for input_room in home.rooms {
        let walls_count = input_room.walls.len();
        if walls_count < 3 {
            return Err(BuildError::RoomWithoutWalls(input_room.name, walls_count).into());
        }

        let mut output_room = output_desc::Room {
            name: input_room.name.clone(),
            walls: vec![],
        };

        let origin = output_desc::Point { x: 0.0, y: 0.0 };
        let mut cur_point = output_desc::Point { x: 0.0, y: 0.0 };
        let mut prev_wall = &input_room.walls[walls_count - 1];
        for i in 0..walls_count {
            let cur_wall = &input_room.walls[i];
            let next_wall = &input_room.walls[(i + 1) % walls_count];

            let start_point =
                calc_wall_point(&cur_point, prev_wall, cur_wall, input_room.orientation).context(
                    format!(
                        "Room {:?}, wall {} and previous one. Error: ",
                        input_room.name, i
                    ),
                )?;

            match cur_wall.direction {
                input_desc::Direction::Left => cur_point.x -= cur_wall.length,
                input_desc::Direction::Right => cur_point.x += cur_wall.length,
                input_desc::Direction::Up => cur_point.y -= cur_wall.length,
                input_desc::Direction::Down => cur_point.y += cur_wall.length,
            }

            let end_point =
                calc_wall_point(&cur_point, cur_wall, next_wall, input_room.orientation).context(
                    format!(
                        "Room {:?}, wall {} and next one. Error: ",
                        input_room.name, i
                    ),
                )?;

            output_room.walls.push(output_desc::Wall {
                start: start_point,
                end: end_point,
                thickness: cur_wall.thickness,
            });

            prev_wall = cur_wall;
        }

        result.rooms.push(output_room);

        if origin != cur_point {
            return Err(BuildError::RoomNotClosed(input_room.name).into());
        }
    }

    Ok(result)
}

fn calc_wall_point(
    point: &output_desc::Point,
    wall1: &input_desc::Wall,
    wall2: &input_desc::Wall,
    orientation: input_desc::WallOrientation,
) -> Result<output_desc::Point, BuildError> {
    if wall1.direction == wall2.direction {
        return Ok(output_desc::Point {
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
        input_desc::Direction::Left => {
            if wall2.direction == input_desc::Direction::Right {
                return Err(BuildError::OppositeDirectionWalls(
                    wall1.direction,
                    wall2.direction,
                ));
            } else if wall2.direction == input_desc::Direction::Up {
                (-half_thick2, half_thick1)
            } else {
                // wall2.direction == input_desc::Direction::Down
                (half_thick2, half_thick1)
            }
        }
        input_desc::Direction::Right => {
            if wall2.direction == input_desc::Direction::Left {
                return Err(BuildError::OppositeDirectionWalls(
                    wall1.direction,
                    wall2.direction,
                ));
            } else if wall2.direction == input_desc::Direction::Up {
                (-half_thick2, -half_thick1)
            } else {
                // wall2.direction == input_desc::Direction::Down
                (half_thick2, -half_thick1)
            }
        }
        input_desc::Direction::Up => {
            if wall2.direction == input_desc::Direction::Down {
                return Err(BuildError::OppositeDirectionWalls(
                    wall1.direction,
                    wall2.direction,
                ));
            } else if wall2.direction == input_desc::Direction::Left {
                (half_thick1, -half_thick2)
            } else {
                // wall2.direction == input_desc::Direction::Right
                (-half_thick1, -half_thick2)
            }
        }
        input_desc::Direction::Down => {
            if wall2.direction == input_desc::Direction::Up {
                return Err(BuildError::OppositeDirectionWalls(
                    wall1.direction,
                    wall2.direction,
                ));
            } else if wall2.direction == input_desc::Direction::Left {
                (half_thick1, half_thick2)
            } else {
                // wall2.direction == input_desc::Direction::Right
                (half_thick1, -half_thick2)
            }
        }
    };

    Ok(output_desc::Point {
        x: point.x + off_x * dir,
        y: point.y + off_y * dir,
    })
}
