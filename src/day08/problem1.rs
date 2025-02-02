use std::collections::{HashMap, HashSet};

pub fn run<I>(input_data: I) -> i32
where
    I: Iterator<Item = String>,
{
    let mut antenna_coords: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in input_data.enumerate() {
        max_x = line.len() as i32 - 1;
        for (x, point) in line.chars().enumerate() {
            if point == '.' || point == '#' {
                continue;
            }
            let coords = (x as i32, y as i32);
            antenna_coords.entry(point).or_default().push(coords);
        }
        max_y = y as i32;
    }

    for (frequency, coords) in &antenna_coords {
        for i in 0..coords.len() {
            for j in 0..coords.len() {
                if i != j {
                    let (x, y) = coords[i];
                    let (xx, yy) = coords[j];

                    let distance_x = (xx - x).abs();
                    let distance_y = (yy - y).abs();

                    let pp1_x: i32;
                    let pp2_x: i32;
                    let pp1_y: i32;
                    let pp2_y: i32;

                    if xx > x {
                        pp1_x = x - distance_x;
                        pp2_x = xx + distance_x;
                    } else {
                        pp1_x = x + distance_x;
                        pp2_x = xx - distance_x;
                    }
                    if yy > y {
                        pp1_y = y - distance_y;
                        pp2_y = yy + distance_y;
                    } else {
                        pp1_y = y + distance_y;
                        pp2_y = yy - distance_y;
                    }

                    let pp1 = (pp1_x, pp1_y);
                    let pp2 = (pp2_x, pp2_y);

                    if pp1_x >= 0
                        && pp1_x <= max_x
                        && pp1_y >= 0
                        && pp1_y <= max_y
                        && antenna_coords
                            .get(frequency)
                            .map_or(true, |nodes| !nodes.contains(&pp1))
                    {
                        antinodes.insert(pp1);
                    }
                    if pp2_x >= 0
                        && pp2_x <= max_x
                        && pp2_y >= 0
                        && pp2_y <= max_y
                        && antenna_coords
                            .get(frequency)
                            .map_or(true, |nodes| !nodes.contains(&pp2))
                    {
                        antinodes.insert(pp2);
                    }
                }
            }
        }
    }
    antinodes.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_1() {
        let input_data = vec![
            ".........".to_string(),
            "...#.....".to_string(),
            ".........".to_string(),
            "....a....".to_string(),
            ".........".to_string(),
            ".....a...".to_string(),
            ".........".to_string(),
            "......#..".to_string(),
            ".........".to_string(),
            ".........".to_string(),
        ];

        let result = run(input_data.into_iter());

        assert_eq!(result, 2);
    }

    #[test]
    fn test_run_2() {
        let input_data = vec![
            "..........".to_string(),
            "...#......".to_string(),
            "#.........".to_string(),
            "....a.....".to_string(),
            "........a.".to_string(),
            ".....a....".to_string(),
            "..#.......".to_string(),
            "......#...".to_string(),
            "..........".to_string(),
            "..........".to_string(),
        ];

        let result = run(input_data.into_iter());

        assert_eq!(result, 4);
    }

    #[test]
    fn test_run_3() {
        let input_data = vec![
            "..........".to_string(),
            "...#......".to_string(),
            "#.........".to_string(),
            "....a.....".to_string(),
            "........a.".to_string(),
            ".....a....".to_string(),
            "..#.......".to_string(),
            "......A...".to_string(),
            "..........".to_string(),
            "..........".to_string(),
        ];

        let result = run(input_data.into_iter());

        assert_eq!(result, 4);
    }

    #[test]
    fn test_run_4() {
        let input_data = vec![
            "......#....#".to_string(),
            "...#....0...".to_string(),
            "....#0....#.".to_string(),
            "..#....0....".to_string(),
            "....0....#..".to_string(),
            ".#....A.....".to_string(),
            "...#........".to_string(),
            "#......#....".to_string(),
            "........A...".to_string(),
            ".........A..".to_string(),
            "..........#.".to_string(),
            "..........#.".to_string(),
        ];

        let result = run(input_data.into_iter());

        assert_eq!(result, 14);
    }
}
