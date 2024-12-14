use std::fs;
use std::env;
use std::collections::{HashSet, HashMap};

type Position = (i64, i64);
type Distance = (i64, i64);
type Segment = (Position, Position);
type SegmentDistance = (Segment, Distance);
type Antinode = Position;

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let grid = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        Grid { grid }
    }

    fn _get(&self, x: i32, y: i32) -> Option<char> {
        if y >= 0 && (y as usize) < self.grid.len() {
            let row = &self.grid[y as usize];
            if x >= 0 && (x as usize) < row.len() {
                return Some(row[x as usize]);
            }
        }
        None
    }

    fn find_by_char(&self, c: char) -> HashSet<Position> {
        let mut positions: HashSet<Position> = HashSet::new();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == c {
                    positions.insert((x as i64, y as i64));
                }
            }
        }
        positions
    }
}

fn get_all_chars(grid: &Grid) -> HashSet<char> {
    let mut all_chars = HashSet::new();
    for row in &grid.grid {
        for &c in row {
            all_chars.insert(c);
        }
    }
    all_chars.remove(&'.');
    all_chars
}

fn get_combination_without_replacement(positions: HashSet<Position>) -> HashSet<Segment> {
    let mut segments: HashSet<Segment> = HashSet::new();
    let positions_vec: Vec<Position> = positions.into_iter().collect();

    for i in 0..positions_vec.len() {
        for j in i + 1..positions_vec.len() {
            // Créer une paire triée pour éviter (A, B) et (B, A) comme des paires distinctes
            let mut pair = vec![positions_vec[i], positions_vec[j]];
            pair.sort(); // Trie les deux éléments pour garantir l'unicité
            segments.insert((pair[0], pair[1])); // Insère la paire triée
        }
    }
    segments
}

fn compute_distance(segment: Segment) -> SegmentDistance {
    let (p1, p2) = segment;
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let dx = x2 - x1;
    let dy = y2 - y1;
    let distance = (dx, dy);
    (segment, distance)
}

fn compute_antinode(segment_distances: SegmentDistance, max_iter: Option<i32>) -> HashSet<Antinode> {
    let segment = segment_distances.0;
    let distance = segment_distances.1;
    let mut point_1 = segment.0;
    let mut point_2 = segment.1;
    let mut antinodes = HashSet::new();
    let max_i;
    match max_iter {
        Some(max_iter) => {
            max_i = max_iter;
            antinodes.insert(point_1);
            antinodes.insert(point_2);
        }
        None => {
            max_i = 1;
        }
    }
    for _ in 0..max_i {
        let antinode1 = (point_1.0 - distance.0, point_1.1 - distance.1);
        let antinode2 = (point_2.0 + distance.0, point_2.1 + distance.1);
        antinodes.insert(antinode1);
        antinodes.insert(antinode2);
        point_1 = antinode1;
        point_2 = antinode2;
    }
    antinodes
}

fn is_antinode_in_square(antinode: Antinode, square: Segment) -> bool {
    let (x, y) = antinode;
    let (p1, p2) = square;
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    x >= x1 && x <= x2-1 && y >= y1 && y <= y2-1
}
fn process_grid(input: &str, square: Segment, max_iter: Option<i32>) -> i32 {
    let grid = Grid::new(input);
    let all_chars = get_all_chars(&grid);
    let mut char_positions: HashMap<char, HashSet<Position>> = HashMap::new();
    for c in all_chars {
        char_positions.insert(c, grid.find_by_char(c));
    }

    let mut all_segments: HashSet<Segment> = HashSet::new();
    for (_, positions) in &char_positions {
        let segments = get_combination_without_replacement(positions.clone());
        all_segments.extend(segments);
    }

    let mut all_segment_distances: HashSet<SegmentDistance> = HashSet::new();
    for segment in all_segments {
        all_segment_distances.insert(compute_distance(segment));
    }

    let mut all_antinodes: HashSet<Antinode> = HashSet::new();
    for segment_distance in all_segment_distances {
        let mut tmp: HashSet<Antinode> = HashSet::new();
        for anti in compute_antinode(segment_distance, max_iter) {
            tmp.insert(anti);
        }
        all_antinodes.extend(tmp);
    }

    let mut count = 0;
    for antinode in all_antinodes {
        if is_antinode_in_square(antinode, square) {
            count += 1;
        }
    }
    count
}

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let input_file_path = current_dir.join("inputs/input.txt");
    let input = fs::read_to_string(input_file_path).expect("Failed to read input file");

    let square = ((0, 0), (50, 50));
    let count = process_grid(&input, square, None);

    println!("The number of antinodes in the square is: {}", count);

    let square = ((0, 0), (50, 50));
    let count = process_grid(&input, square, Some(50));

    println!("The number of antinodes in the square with 50 iterations is: {}", count);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_find_by_char() {
        let input = "\
....\n\
.#..\n\
..#.\n\
....";
        let grid = Grid::new(input);
        let positions = grid.find_by_char('#');
        assert_eq!(positions.len(), 2);
        assert!(positions.contains(&(1, 1)));
        assert!(positions.contains(&(2, 2)));
    }

    #[test]
    fn test_get_all_chars() {
        let input = "\
....\n\
.#..\n\
..#.\n\
....";
        let grid = Grid::new(input);
        let all_chars = get_all_chars(&grid);
        assert_eq!(all_chars.len(), 1);
        assert!(all_chars.contains(&'#'));
    }

    #[test]
    fn test_get_combination_without_replacement() {
        let positions: HashSet<Position> = [(0, 0), (1, 1), (2, 2)].iter().cloned().collect();
        let segments = get_combination_without_replacement(positions);
        assert_eq!(segments.len(), 3);
        assert!(segments.contains(&((0, 0), (1, 1))));
        assert!(segments.contains(&((0, 0), (2, 2))));
        assert!(segments.contains(&((1, 1), (2, 2))));
        assert!(!segments.contains(&((1, 1), (0, 0))));
        assert!(!segments.contains(&((2, 2), (1, 1))));
        assert!(!segments.contains(&((2, 2), (0, 0))));
        assert!(!segments.contains(&((0, 0), (0, 0))));
        assert!(!segments.contains(&((1, 1), (1, 1))));
        assert!(!segments.contains(&((2, 2), (2, 2))));
    }

    #[test]
    fn test_compute_distance() {
        let segment = ((0, 0), (3, 4));
        let segment_distance = compute_distance(segment);
        assert_eq!(segment_distance, (segment, (3, 4)));
    }

    #[test]
    fn test_compute_distance_2() {
        let segment = ((3, 4), (0, 0));
        let segment_distance = compute_distance(segment);
        assert_eq!(segment_distance, (segment, (-3, -4)));
    }

    #[test]
    fn test_compute_antinode() {
        let segment_distance = (((4, 3), (5, 5)), (1, 2));
        let antinodes = compute_antinode(segment_distance, None);
        assert!(antinodes.contains(&(3, 1)));
        assert!(antinodes.contains(&(6, 7)));
        assert_eq!(antinodes.len(), 2);
    }

    #[test]
    fn test_is_antinode_in_square() {
        let antinode = (5, 5);
        let square = ((0, 0), (10, 10));
        assert!(is_antinode_in_square(antinode, square));
        let antinode_outside = (15, 15);
        assert!(!is_antinode_in_square(antinode_outside, square));
    }

    #[test]
    fn test_process_grid() {
        let input = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let square = ((0, 0), (12, 12));
        let result = process_grid(input, square, None);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_process_grid_2() {
        let input = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let square = ((0, 0), (12, 12));
        let result = process_grid(input, square, Some(12));
        assert_eq!(result, 34);
    }
}