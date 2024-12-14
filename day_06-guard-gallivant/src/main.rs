use std::fs;
use std::env;
use std::collections::HashSet;
use indicatif::ProgressBar;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Guard {
    position: (i32, i32),
    direction: Direction,
}

impl Guard {
    fn new(position: (i32, i32), direction: Direction) -> Guard {
        Guard { position, direction }
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.position.1 -= 1,
            Direction::Down => self.position.1 += 1,
            Direction::Left => self.position.0 -= 1,
            Direction::Right => self.position.0 += 1,
        }
    }

    fn look_forward(&self) -> (i32, i32) {
        match self.direction {
            Direction::Up => (self.position.0, self.position.1 - 1),
            Direction::Down => (self.position.0, self.position.1 + 1),
            Direction::Left => (self.position.0 - 1, self.position.1),
            Direction::Right => (self.position.0 + 1, self.position.1),
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

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

    fn get(&self, x: i32, y: i32) -> Option<char> {
        if y >= 0 && (y as usize) < self.grid.len() {
            let row = &self.grid[y as usize];
            if x >= 0 && (x as usize) < row.len() {
                return Some(row[x as usize]);
            }
        }
        None
    }
}

struct Game {
    grid: Grid,
    guard: Guard,
    visited: HashSet<(i32, i32)>,
    blocked: bool,
}

impl Game {
    fn new(grid: Grid, guard: Guard) -> Game {
        let mut visited = HashSet::new();
        visited.insert(guard.position);
        Game { grid, guard, visited, blocked: false }
    }

    fn play(&mut self) {
        let mut step_count = 0;
        loop {
            let next_position = self.guard.look_forward();
            let next_cell = self.grid.get(next_position.0, next_position.1);
            let this_cell = self.grid.get(self.guard.position.0, self.guard.position.1);
            match this_cell {
                Some('*') => {
                    step_count += 1;
                }
                _ => {
                    step_count = 0;
                }
            }

            if step_count > 200 {
                self.blocked = true;
                break;
            }

            match next_cell {
                Some('#') | Some('@') => {
                    self.guard.turn_right();
                }
                Some(_) => {
                    // avant de bouger, je modifie la cellule actuelle par '*'
                    self.grid.grid[self.guard.position.1 as usize][self.guard.position.0 as usize] = '*';
                    self.guard.move_forward();
                    self.visited.insert(self.guard.position);
                }
                None => {
                    // Si on sort du plateau, on arrête
                    break;
                }
            }
        }
    }

    fn count_visited_cells(&self) -> usize {
        self.visited.len()
    }

    fn is_blocked(&self) -> bool {
        self.blocked
    }
}

fn find_starting_position(grid: &Grid) -> (i32, i32) {
    for y in 0..grid.grid.len() {
        for x in 0..grid.grid[y].len() {
            if grid.grid[y][x] == '^' {
                return (x as i32, y as i32);
            }
        }
    }
    panic!("No starting position found");
}

fn block(input: String) -> u64 {
    let mut score_block: u64 = 0;
    let grid = Grid::new(&input);
    let starting_position = find_starting_position(&grid);
    let total_cells = input.lines().count() * input.lines().next().unwrap_or("").len();
    let pb = ProgressBar::new(total_cells as u64);

    for (y, line) in input.lines().enumerate() {
        for (x, _ch) in line.chars().enumerate() {
            if starting_position == (x as i32, (y as i32) + 1) {
                continue;
            }
            let mut new_grid = grid.clone();
            new_grid.grid[y][x] = '@';
            let guard = Guard::new(starting_position, Direction::Up);
            let mut game = Game::new(new_grid, guard);
            game.play();
            if game.is_blocked() {
                score_block += 1;
            }
            pb.inc(1);
        }
    }
    pb.finish_with_message("done");
    score_block
}

fn part_1() 
{
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let input_file_path = current_dir.join("inputs/input.txt");
    let input = fs::read_to_string(input_file_path).expect("Failed to read input file");

    let grid = Grid::new(&input);
    let starting_position = find_starting_position(&grid);

    let guard = Guard::new(starting_position, Direction::Up);
    let mut game = Game::new(grid, guard);
    game.play();

    let visited_cells = game.count_visited_cells();
    println!("Nombre de cellules visitées: {}", visited_cells);
}

fn part_2() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let input_file_path = current_dir.join("inputs/input.txt");
    let input = fs::read_to_string(input_file_path).expect("Failed to read input file");

    let score_block = block(input);
    println!("Nombre de cellules bloquées: {}", score_block);
}

fn main() {
    part_1();
    part_2();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let grid = Grid::new(input);
        let starting_position = find_starting_position(&grid);
        assert_eq!(starting_position, (4, 6));

        let guard = Guard::new(starting_position, Direction::Up);
        let mut game = Game::new(grid, guard);
        game.play();
        assert_eq!(game.count_visited_cells(), 41);
    }

    #[test]
    fn test_part_2() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let score_block = block(input.to_string());
        assert_eq!(score_block, 6);

    }
}