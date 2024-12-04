use std::vec;

const PATTERNS: &[&[&str]] = &[
    &[
        "M.M",
        ".A.",
        "S.S"
    ],
    &[
        "M.S",
        ".A.",
        "M.S"
    ],
    &[
        "S.S",
        ".A.",
        "M.M"
    ],
    &[
        "S.M",
        ".A.",
        "S.M"
    ]
];

fn matches_pattern(grid: &Vec<Vec<char>>, row: usize, col: usize, pattern: &[&str]) -> bool {
    if row + 3 > grid.len() || col + 3 > grid[0].len() {
        return false;
    }

    for (pattern_row, &pattern_line) in pattern.iter().enumerate() {
        for (pattern_col, pattern_char) in pattern_line.chars().enumerate() {
            let grid_char = grid[row + pattern_row][col + pattern_col];
            
            if pattern_char != '.' && pattern_char != grid_char {
                return false;
            }
        }
    }
    true
}

fn find_matches(input: &str) -> (usize, Vec<Vec<char>>) {
    // Convert input string to 2D grid
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut result_grid = grid.clone();
    let mut match_count = 0;

    for row in 0..rows - 2 {
        for col in 0..cols - 2 {
            // Check against all patterns
            for pattern in PATTERNS {
                if matches_pattern(&grid, row, col, pattern) {
                    // Mark the match in the result grid
                    for (pattern_row, &pattern_line) in pattern.iter().enumerate() {
                        for (pattern_col, pattern_char) in pattern_line.chars().enumerate() {
                            if pattern_char != '.' {
                                result_grid[row + pattern_row][col + pattern_col] = pattern_char;
                            }
                        }
                    }
                    match_count += 1;
                }
            }
        }
    }

    (match_count, result_grid)
}

fn main() {
    let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let (matches, result) = find_matches(input);

    println!("Matches found: {}", matches);
    for row in result {
        println!("{}", row.iter().collect::<String>());
    }
}