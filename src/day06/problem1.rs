use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, Default)]
enum Direction {
    North,
    East,
    South,
    West,
    #[default]
    None, // this was less of a pain than handling a proper option...
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Direction::North),
            '>' => Some(Direction::East),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            _ => None,
        }
    }

    fn rotate_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::None => Direction::None,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
struct PatrolMap {
    obstacles_x: HashMap<i32, HashSet<i32>>,
    obstacles_y: HashMap<i32, HashSet<i32>>,
    max_x: i32,
    max_y: i32,
    start_node: (i32, i32, Direction),
    visited_nodes: HashSet<(i32, i32)>,
}

impl PatrolMap {
    fn mark_obstacle(&mut self, x: i32, y: i32) {
        self.obstacles_x.entry(x).or_default().insert(y);
        self.obstacles_y.entry(y).or_default().insert(x);
    }

    fn mark_boundries(&mut self, x: i32, y: i32) {
        self.max_x = x;
        self.max_y = y;
    }

    fn is_at_the_boundry(&self, x: i32, y: i32) -> bool {
        x == 0 || x == self.max_x || y == 0 || y == self.max_y
    }

    fn calculate_next_position(&self, current_position: (i32, i32, Direction)) -> (i32, i32) {
        let (x, y, current_direction) = current_position;
        match current_direction {
            Direction::North => {
                if let Some(ys) = self.obstacles_x.get(&x) {
                    if let Some(&obstacle_y) = ys.iter().filter(|&&obstacle_y| obstacle_y < y).max()
                    {
                        return (x, obstacle_y + 1);
                    }
                }
                (x, 0)
            }
            Direction::East => {
                if let Some(ys) = self.obstacles_y.get(&y) {
                    if let Some(&obstacle_x) = ys.iter().filter(|&&obstacle_x| obstacle_x > x).min()
                    {
                        return (obstacle_x - 1, y);
                    }
                }
                (self.max_x, y)
            }
            Direction::South => {
                if let Some(ys) = self.obstacles_x.get(&x) {
                    if let Some(&obstacle_y) = ys.iter().filter(|&&obstacle_y| obstacle_y > y).min()
                    {
                        return (x, obstacle_y - 1);
                    }
                }
                (x, self.max_y)
            }
            Direction::West => {
                if let Some(ys) = self.obstacles_y.get(&y) {
                    if let Some(&obstacle_x) = ys.iter().filter(|&&obstacle_x| obstacle_x < x).max()
                    {
                        return (obstacle_x + 1, y);
                    }
                }
                (0, y)
            }
            Direction::None => todo!(),
        }
    }
}

pub fn run<I>(input_data: I) -> i32
where
    I: Iterator<Item = String>,
{
    let mut patrol_map = PatrolMap::default();

    for (y, line) in input_data.enumerate() {
        patrol_map.mark_boundries(line.len() as i32, y as i32);

        for (x, district) in line.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;

            if district == '.' {
                continue;
            } else if district == '#' {
                patrol_map.mark_obstacle(x, y);
            } else if let Some(direction) = Direction::from_char(district) {
                patrol_map.start_node = (x, y, direction);
            }
        }
    }

    if patrol_map.start_node.2 == Direction::None {
        panic!("No current direction set after parsing the map");
    }

    simulate_patrol(patrol_map.clone()).visited_nodes.len() as i32
}

fn simulate_patrol(mut patrol_map: PatrolMap) -> PatrolMap {
    let mut current_node = patrol_map.start_node;

    loop {
        let (x, y, direction) = current_node;

        let (next_x, next_y) = patrol_map.calculate_next_position(current_node);
        if direction == Direction::East || direction == Direction::West {
            for intermediate_x in x.min(next_x)..=x.max(next_x) {
                patrol_map
                    .visited_nodes
                    .insert((intermediate_x, y));
            }
        } else {
            for intermediate_y in y.min(next_y)..=y.max(next_y) {
                patrol_map
                    .visited_nodes
                    .insert((x, intermediate_y));
            }
        }

        if patrol_map.is_at_the_boundry(next_x, next_y) {
            return patrol_map;
        }

        current_node = (next_x, next_y, current_node.2.rotate_right());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        // this example is NOT covering all cases, unfortunately...
        let input_data = vec![
            "....#.....".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            "..#.......".to_string(),
            ".......#..".to_string(),
            "..........".to_string(),
            ".#..^.....".to_string(),
            "........#.".to_string(),
            "#.........".to_string(),
            "......#...".to_string(),
        ];

        let result = run(input_data.into_iter());

        assert_eq!(result, 41);
    }
}
