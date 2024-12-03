use regex::Regex;

fn process_mul_expressions(input: &str) -> (Vec<(String, i32)>, i32) {
    // Regex to match mul expressions ONLY with parentheses ()
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
    // Find all matches and process them
    let mut matches = Vec::new();
    let mut total_sum = 0;
    
    for cap in re.captures_iter(input) {
        // Extract X and Y, parse them to integers
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        
        // Create the original match string and compute the product
        let full_match = cap[0].to_string();
        let product = x * y;
        
        // Add to matches and running sum
        matches.push((full_match, product));
        total_sum += product;
    }
    
    (matches, total_sum)
}

fn main() {
    // Test string with various delimiters
    let test_string = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let (results, sum) = process_mul_expressions(test_string);
    
    println!("Multiplication Matches:");
    for (expression, product) in results {
        println!("{} = {}", expression, product);
    }
    
    println!("\nTotal Sum of Products: {}", sum);
}