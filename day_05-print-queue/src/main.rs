use std::collections::{HashMap, HashSet};
use std::fs;
use std::env;

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut page_ordering_rules = Vec::new();
    let mut updates = Vec::new();
    let mut is_reading_rules = true;

    for line in input.lines() {
        if line.is_empty() {
            is_reading_rules = false;
            continue;
        }

        if is_reading_rules {
            let parts: Vec<u32> = line.split('|')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            page_ordering_rules.push((parts[0], parts[1]));
        } else {
            let update: Vec<u32> = line.split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            updates.push(update);
        }
    }

    (page_ordering_rules, updates)
}

fn build_dependency_graph(rules: &[(u32, u32)]) -> HashMap<u32, HashSet<u32>> {
    let mut graph = HashMap::new();
    for &(x, y) in rules {
        graph.entry(x).or_insert_with(HashSet::new).insert(y);
    }
    graph
}

fn is_valid_update(update: &[u32], graph: &HashMap<u32, HashSet<u32>>) -> bool {
    let positions: HashMap<u32, usize> = update.iter()
        .enumerate()
        .map(|(i, &value)| (value, i))
        .collect();

    for (&x, dependencies) in graph {
        if let Some(&x_pos) = positions.get(&x) {
            for &y in dependencies {
                if let Some(&y_pos) = positions.get(&y) {
                    if x_pos >= y_pos {
                        return false; // La règle x|y est violée
                    }
                }
            }
        }
    }

    true
}

fn main() {
    // Lire le fichier d'entrée
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let input_file_path = current_dir.join("inputs/input.txt");
    let input = fs::read_to_string(input_file_path).expect("Failed to read input file");

    // Parser les données d'entrée
    let (page_ordering_rules, updates) = parse_input(&input);

    // Construire le graphe des dépendances
    let dependency_graph = build_dependency_graph(&page_ordering_rules);

    // Vérifier chaque mise à jour
    let mut total_middle_sum = 0;
    for update in updates {
        if is_valid_update(&update, &dependency_graph) {
            // Trouver la page centrale
            let middle = update[update.len() / 2];
            total_middle_sum += middle;
        }
    }

    println!("Total sum of middle page numbers: {}", total_middle_sum);
}
