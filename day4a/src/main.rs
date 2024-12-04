use std::vec;

fn find_word_matches(grid_str: &str, word: &str) -> (Vec<String>, usize) {
    // Convert grid string to 2D vector of characters
    let grid: Vec<Vec<char>> = grid_str
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut marked_grid = grid.clone();
    let mut match_count = 0;

    // Directions: right, left, down, up, down-right, up-left, down-left, up-right
    let directions = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down-right
        (-1, -1), // up-left
        (1, -1),  // down-left
        (-1, 1)   // up-right
    ];

    // Recursive search function
    fn recursive_search(
        grid: &mut Vec<Vec<char>>, 
        word: &str, 
        r: i32, 
        c: i32, 
        word_index: usize, 
        direction: (i32, i32)
    ) -> bool {
        // Get grid dimensions
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;

        // Check bounds and character match
        if r < 0 || r >= rows || c < 0 || c >= cols || 
           grid[r as usize][c as usize] != word.chars().nth(word_index).unwrap() {
            return false;
        }
        
        // Base case: Successfully matched entire word
        if word_index == word.len() - 1 {
            // Mark the last cell
            grid[r as usize][c as usize] = word.chars().nth(word_index).unwrap();
            return true;
        }
        
        // Move to next position in current direction
        let (dr, dc) = direction;
        recursive_search(grid, word, r + dr, c + dc, word_index + 1, direction)
    }

    // Try starting word search from every cell
    for r in 0..rows {
        for c in 0..cols {
            for &direction in &directions {
                // Create a copy of the grid for each search
                let mut temp_grid = marked_grid.clone();
                
                // If this starting cell matches first letter
                if grid[r][c] == word.chars().next().unwrap() {
                    // If full word found, increment match count
                    if recursive_search(&mut temp_grid, word, r as i32, c as i32, 0, direction) {
                        marked_grid = temp_grid;
                        match_count += 1;
                    }
                }
            }
        }
    }

    // Format the grid, replacing all non-matched characters with '.'
    let formatted_grid: Vec<String> = marked_grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|&ch| if word.contains(ch) { ch } else { '.' })
                .collect()
        })
        .collect();

    (formatted_grid, match_count)
}

fn main() {
    let grid_str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let word = "XMAS";
    let (result_grid, matches) = find_word_matches(grid_str, word);

    println!("Match Count: {}", matches);
    println!("\nResult Grid:");
    for row in result_grid {
        println!("{}", row);
    }
}