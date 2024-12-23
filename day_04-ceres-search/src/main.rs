use std::fs;
use std::io::Result;
use std::path::Path;
use std::env;

struct Matrix {
    data: Vec<Vec<char>>,
    size: (usize, usize),
}

// TODO: Définir un itérateur pour parcourir les indices et les valeurs d'une matrice : (rows, cols), char

impl Matrix {
    fn new(data: Vec<Vec<char>>) -> Self {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        Matrix { data, size: (rows, cols) }
    }

    /// Vérifie si un motif correspond à une position donnée
    fn matches_pattern(&self, pattern: &Matrix, center_row: usize, center_col: usize) -> bool {
        let pattern_center_row = pattern.size.0 as isize / 2;
        let pattern_center_col = pattern.size.1 as isize / 2;

        for (i, row) in pattern.data.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                if ch != ' ' { // Seuls les caractères significatifs sont vérifiés
                    let target_row = center_row as isize + i as isize - pattern_center_row;
                    let target_col = center_col as isize + j as isize - pattern_center_col;

                    // Vérification des limites
                    if target_row < 0 || target_col < 0 || 
                        target_row >= self.size.0 as isize || target_col >= self.size.1 as isize {
                        return false;
                    }

                    // Vérification du caractère
                    if self.data[target_row as usize][target_col as usize] != ch {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Compte le nombre de correspondances pour un ensemble de motifs
    fn count_pattern_matches(&self, patterns: &[Matrix]) -> usize {
        let mut count = 0;

        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
                // Filtrer sur le caractère central du premier motif (optimisation)
                let central_char = patterns[0].data[patterns[0].size.0 / 2][patterns[0].size.1 / 2];
                if self.data[row][col] == central_char {
                    // Tester chaque motif à la position donnée
                    for pattern in patterns {
                        if self.matches_pattern(pattern, row, col) {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
}

fn file_reader(file_path: &Path) -> Result<String> {
    fs::read_to_string(file_path)
}

fn get_aoc_input(aoc_input_file: &str) -> Result<String> {
    let current_dir = env::current_dir()
        .expect("Failed to get current directory");
    let file_input_path = current_dir.join("inputs/").join(aoc_input_file);
    file_reader(file_input_path.as_path()) 
}

fn string_to_matrix(input_file: String) -> Matrix {
    let mut data: Vec<Vec<char>> = Vec::new();
    for line in input_file.lines() {
        let data_line: Vec<char> = line.chars().collect();
        data.push(data_line);
    }
    Matrix::new(data)
}

fn main() {
    let contents = match get_aoc_input("input.txt") {  // 'match' handles the result of file_parser
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading the file: {}", e);
            return;
        },
    };

    let word_search: Matrix = string_to_matrix(contents);

    let mut paterns_1 = vec![];
    paterns_1.push(Matrix::new(vec![
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', 'X', 'M', 'A', 'S'],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ]));
    paterns_1.push(Matrix::new(vec![
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec!['S', 'A', 'M', 'X', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ]));
    paterns_1.push(Matrix::new(vec![
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', 'X', ' ', ' ', ' '],
        vec![' ', ' ', ' ', 'M', ' ', ' ', ' '],
        vec![' ', ' ', ' ', 'A', ' ', ' ', ' '],
        vec![' ', ' ', ' ', 'S', ' ', ' ', ' '],
    ]));
    paterns_1.push(Matrix::new(vec![
        vec![' ', ' ', ' ', 'S', ' ', ' ', ' '],
        vec![' ', ' ', ' ', 'A', ' ', ' ', ' '],
        vec![' ', ' ', ' ', 'M', ' ', ' ', ' '],
        vec![' ', ' ', ' ', 'X', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ]));
    paterns_1.push(Matrix::new(vec![
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', 'X', ' ', ' ', ' '],
        vec![' ', ' ', 'M', ' ', ' ', ' ', ' '],
        vec![' ', 'A', ' ', ' ', ' ', ' ', ' '],
        vec!['S', ' ', ' ', ' ', ' ', ' ', ' '],
    ]));
    paterns_1.push(Matrix::new(vec![
        vec![' ', ' ', ' ', ' ', ' ', ' ', 'S'],
        vec![' ', ' ', ' ', ' ', ' ', 'A', ' '],
        vec![' ', ' ', ' ', ' ', 'M', ' ', ' '],
        vec![' ', ' ', ' ', 'X', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ]));
    paterns_1.push(Matrix::new(vec![
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', 'X', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', 'M', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', 'A', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', 'S'],
    ]));
    paterns_1.push(Matrix::new(vec![
        vec!['S', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', 'A', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', 'M', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', 'X', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ]));

    let mut paterns_2 = vec![];
    paterns_2.push(Matrix::new(vec![
        vec!['M', ' ', 'M'],
        vec![' ', 'A', ' '],
        vec!['S', ' ', 'S'],
    ]));
    paterns_2.push(Matrix::new(vec![
        vec!['S', ' ', 'S'],
        vec![' ', 'A', ' '],
        vec!['M', ' ', 'M'],
    ]));
    paterns_2.push(Matrix::new(vec![
        vec!['M', ' ', 'S'],
        vec![' ', 'A', ' '],
        vec!['M', ' ', 'S'],
    ]));
    paterns_2.push(Matrix::new(vec![
        vec!['S', ' ', 'M'],
        vec![' ', 'A', ' '],
        vec!['S', ' ', 'M'],
    ]));

    let match_count_1 = word_search.count_pattern_matches(&paterns_1);
    let match_count_2 = word_search.count_pattern_matches(&paterns_2);
    println!("Nombre de correspondances trouvées: {}", match_count_1);
    println!("Nombre de correspondances trouvées: {}", match_count_2);
}
