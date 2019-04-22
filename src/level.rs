#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Wall,
    Player,
    Box,
    Goal,
    BoxInGoal,
}

pub struct Level {
    tiles: [[Tile; 20]; 16],
}

impl Level {
    pub fn parse(source: &str) -> Level {
        let mut tiles = [[Tile::Empty; 20]; 16];

        let width = source.lines().map(|x| x.len()).max().unwrap();
        let height = source.trim_end().lines().count();
        println!("{}x{}", width, height);
        let offset = ((20 - width) / 2, (16 - height) / 2);

        let mut x = offset.0;
        let mut y = offset.1;

        for c in source.chars() {
            match c {
                ' ' => tiles[16 - y][x] = Tile::Empty,
                '#' => tiles[16 - y][x] = Tile::Wall,
                '.' => tiles[16 - y][x] = Tile::Goal,
                '$' => tiles[16 - y][x] = Tile::Box,
                '@' => tiles[16 - y][x] = Tile::Player,
                '+' => tiles[16 - y][x] = Tile::BoxInGoal,
                _ => {}
            };

            if c == '\n' {
                x = offset.0;
                y += 1;
            } else {
                x += 1;
            }
        }

        Level { tiles }
    }

    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        self.tiles[y][x] == Tile::Wall
    }

    pub fn player_pos(&self) -> (usize, usize) {
        for (y, line) in self.tiles.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                if *tile == Tile::Player {
                    return (x, y);
                }
            }
        }

        panic!("No player on map");
    }

    pub fn boxes_pos(&self) -> Vec<(usize, usize)> {
        self.tiles
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter().enumerate().filter_map(move |(x, &tile)| {
                    if tile == Tile::Box || tile == Tile::BoxInGoal {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    pub fn goals_pos(&self) -> Vec<(usize, usize)> {
        self.tiles
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter().enumerate().filter_map(move |(x, &tile)| {
                    if tile == Tile::Goal || tile == Tile::BoxInGoal {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    pub fn ground(&self) -> [[usize; 20]; 16] {
        let mut ground = [[false; 20]; 16];
        let mut stack = vec![self.player_pos()];

        while let Some(pos) = stack.pop() {
            if self.tiles[pos.1][pos.0] == Tile::Wall {
                continue;
            }

            if ground[pos.1][pos.0] {
                continue;
            }

            ground[pos.1][pos.0] = true;

            if pos.0 > 0 {
                stack.push((pos.0 - 1, pos.1))
            }
            if pos.0 < 20 {
                stack.push((pos.0 + 1, pos.1))
            }
            if pos.1 > 0 {
                stack.push((pos.0, pos.1 - 1))
            }
            if pos.1 < 16 {
                stack.push((pos.0, pos.1 + 1))
            }
        }

        let mut result = [[0usize; 20]; 16];

        for y in 1..15 {
            for x in 1..19 {
                result[y][x] = match (
                    ground[y - 1][x - 1],
                    ground[y - 1][x],
                    ground[y - 1][x + 1],
                    ground[y][x - 1],
                    ground[y][x],
                    ground[y][x + 1],
                    ground[y + 1][x - 1],
                    ground[y + 1][x],
                    ground[y + 1][x + 1],
                ) {
                    // Corners
                    (_, true, true, false, true, true, _, false, _) => 1, // Large ┏
                    (_, true, false, false, true, true, _, false, _) => 16, // Tight ┏
                    (true, true, _, true, true, false, _, false, _) => 2, // Large ┓
                    (false, true, _, true, true, false, _, false, _) => 17, // Tight ┓
                    (_, false, _, false, true, true, _, true, true) => 3, // Large ┗
                    (_, false, _, false, true, true, _, true, false) => 18, // Tight ┗
                    (_, false, _, true, true, false, true, true, _) => 4, // Large ┛
                    (_, false, _, true, true, false, false, true, _) => 19, // Tight ┛
                    // Large borders
                    (_, true, true, false, true, true, _, true, true) => 5,
                    (true, true, _, true, true, false, true, true, _) => 6,
                    (true, true, true, true, true, true, _, false, _) => 7,
                    (_, false, _, true, true, true, true, true, true) => 8,
                    // Small corridors
                    (_, false, _, true, true, true, _, false, _) => 14,
                    (_, true, _, false, true, false, _, true, _) => 15,
                    // Dead ends
                    (_, true, _, false, true, false, _, false, _) => 20,
                    (_, false, _, true, true, false, _, false, _) => 21,
                    (_, false, _, false, true, false, _, true, _) => 22,
                    (_, false, _, false, true, true, _, false, _) => 23,
                    // Three-ways
                    (false, true, false, true, true, true, _, false, _) => 24,
                    (false, true, _, true, true, false, false, true, _) => 25,
                    (_, true, false, false, true, true, _, true, false) => 26,
                    (_, false, _, true, true, true, false, true, false) => 27,
                    // Tight cross
                    (false, true, false, true, true, true, false, true, false) => 28,
                    // Foobar
                    (false, true, true, true, true, true, _, false, false) => 29,
                    (true, true, false, true, true, true, false, false, _) => 30,
                    // // Foobar
                    // (_, true, true, false, true, true, _, true, true) => 5,
                    // (true, true, _, true, true, false, true, true, _) => 6,
                    // (true, true, true, true, true, true, _, false, _) => 7,
                    // (_, false, _, true, true, true, true, true, true) => 8,
                    // Floaters
                    (_, _, _, _, false, _, false, true, true) => 10,
                    (_, _, _, _, false, _, true, true, false) => 11,
                    (_, _, _, _, false, _, true, true, true) => 12,
                    (_, _, _, _, false, _, false, true, false) => 13,
                    // Catch-all
                    (_, _, _, _, true, _, _, _, _) => 9,
                    (_, _, _, _, _, _, _, _, _) => 0,
                }
            }
        }

        result
    }
}
