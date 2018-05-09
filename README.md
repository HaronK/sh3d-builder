# sh3d-builder

This is the tool for building rooms for [Sweet Home 3D](http://www.sweethome3d.com/) application.

It is hard to build a walls in this program because it calculates length of the wall relatively to its middle and not internally to the room.
That's why when you sets wall thickness internal size fo the wall become smaller.

Using this tool one can generate rooms and walls with a proper sizes.

Features:

1. Simple JSON format to specify shape of the room and walls size and thickness.
2. Building a whole house by connecting rooms. Verify correctness of these connections: same thickness, same coordinates of the start/end points of the walls.
3. Supporting only for horizontal and vertical walls.

## Compilation

Install [rustup](https://rustup.rs/).

Clone and compile project:

```bash
git clone https://github.com/HaronK/sh3d-builder.git
cd ./sh3d-builder
cargo build --release
```

## Usage

```bash
$ ./target/release/sh3d-builder -h
Sweet Home 3D model builder 0.1.0
Khryptul Oleg <okreptul@yahoo.com>
Generating data for building Sweet Home 3D rooms from the human readable description.

USAGE:
    sh3d-builder [FLAGS] [FILE]

FLAGS:
    -f               Format the output
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               Prints version number: 0.1.0

ARGS:
    <FILE>    Sets the input file to use
```

Input data can be specified either as a file name:

```bash
./target/release/sh3d-builder ./examples/two_rooms.json
```

or can be passed via standard input:

```bash
cat ./examples/two_rooms.json | ./target/release/sh3d-builder
```

## Input file format

See for more examples in **_examples_** folder.

```json
{
    "orientation": "CW",
    "rooms": [
        {
            "name": "Room1",
            "walls": [
                {
                    "direction": "Right",
                    "length": 398,
                    "thickness": 8
                },
                {
                    "direction": "Down",
                    "length": 460,
                    "thickness": 45
                },
                {
                    "direction": "Left",
                    "length": 398,
                    "thickness": 25
                },
                {
                    "direction": "Up",
                    "length": 460,
                    "thickness": 7
                }
            ]
        },
        {
            "name": "Room2",
            "walls": [
                {
                    "direction": "Right",
                    "length": 398,
                    "thickness": 8
                },
                {
                    "direction": "Down",
                    "length": 460,
                    "thickness": 45
                },
                {
                    "direction": "Left",
                    "length": 398,
                    "thickness": 25
                },
                {
                    "direction": "Up",
                    "length": 460,
                    "thickness": 7
                }
            ]
        }
    ],
    "connections": [
        {
            "room1": {
                "name": "Room1",
                "wall_index": 3
            },
            "room2": {
                "name": "Room2",
                "wall_index": 1
            },
            "conn_type": "Coincide"
        }
    ]
}
```

Where:

* **orientation** - is the order of the walls in room description: **CW** -clockwise, **CCW** - counterclockwise.
* **rooms** - list of the rooms.
* **rooms.name** -  room name.
* **rooms.walls** - room walls.
* **rooms.walls.direction** - in what direction wall goes (**Right**, **Left**, **Up**, **Down**).
* **rooms.walls.length** - wall length.
* **rooms.walls.thickness** - wall thickness.
* **connections** - connections between rooms.
* **connections.room[1,2]** - description of the first/second room to connect.
* **connections.room[1,2].name** - room name. Should be one of the rooms from the previous section.
* **connections.room[1,2].wall_index** - index of the wall in the room description (starting from 1).
* **connections.conn_type** - specify how walls touch each other: **Coincide** - walls completely coinciding, **StartToEnd** - starting point of the first wall coincide with the end point of the second one, **EndToStart** - end point of the first wall coincide with the start point of the second one.

## Output file format

```json
{
    "rooms": [
        {
            "name": "Room1",
            "walls": [
                {
                    "start": { "x": -3.5, "y": -4.0 },
                    "end": { "x": 420.5, "y": -4.0 },
                    "thickness": 8.0
                },
                {
                    "start": { "x": 420.5, "y": -4.0 },
                    "end": { "x": 420.5, "y": 464.0 },
                    "thickness": 45.0
                },
                {
                    "start": { "x": 420.5, "y": 464.0 },
                    "end": { "x": -3.5, "y": 464.0 },
                    "thickness": 8.0
                },
                {
                    "start": { "x": -3.5, "y": 464.0 },
                    "end": { "x": -3.5, "y": -4.0 },
                    "thickness": 7.0
                }
            ]
        },
        {
            "name": "Room2",
            "walls": [
                {
                    "start": { "x": -3.5, "y": 464.0 },
                    "end": { "x": 420.5, "y": 464.0 },
                    "thickness": 8.0
                },
                {
                    "start": { "x": 420.5, "y": 464.0 },
                    "end": { "x": 420.5, "y": 940.5 },
                    "thickness": 45.0
                },
                {
                    "start": { "x": 420.5, "y": 940.5 },
                    "end": { "x": -3.5, "y": 940.5 },
                    "thickness": 25.0
                },
                {
                    "start": { "x": -3.5, "y": 940.5 },
                    "end": { "x": -3.5, "y": 464.0 },
                    "thickness": 7.0
                }
            ]
        }
    ]
}
```

It contains rooms wall coordinates that can be used to build correct rooms in Sweet Home 3D.

## Units

Use the same measurement units that you are using in Sweet Home 3D (cm, inch).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
