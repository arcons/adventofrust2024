use regex::Regex;

fn process_mul_expressions(input: &str) -> (Vec<(String, i32, bool)>, i32) {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    
    let mut matches = Vec::new();
    let mut total_sum = 0;
    let mut is_summing = true;
    
    for cap in mul_re.captures_iter(input) {
        // Check for don't() before this match
        if dont_re.is_match(&input[..cap.get(0).unwrap().start()]) {
            is_summing = false;
        }
        
        // Check for do() before this match
        if do_re.is_match(&input[..cap.get(0).unwrap().start()]) {
            is_summing = true;
        }
        
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        
        let full_match = cap[0].to_string();
        let product = x * y;
        
        let was_summed = is_summing;
        if is_summing {
            total_sum += product;
        }
        
        matches.push((full_match, product, was_summed));
    }
    
    (matches, total_sum)
}

fn main() {
    let test_string = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))don't()_mul(5,5)do()?mul(8,5)";
    let (results, sum) = process_mul_expressions(test_string);
    
    println!("Multiplication Tracking:");
    for (expression, product, was_summed) in results {
        println!("{} = {} [{}]", 
            expression, 
            product, 
            if was_summed { "ADDED" } else { "SKIPPED" }
        );
    }
    
    println!("\nTotal Sum of Products: {}", sum);
} //162716714 too high