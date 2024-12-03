use regex::Regex;

fn process_mul_expressions(input: &str) -> (Vec<(String, i32, bool)>, i32) {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let mut matches = Vec::new();
    let mut total_sum = 0;
    let mut is_summing = true;
    let mut last_signal_start = 0;

    for cap in mul_re.captures_iter(input) {
        let start = cap.get(0).unwrap().start();
        let preceding_text = &input[last_signal_start..start];
        
        is_summing = if dont_re.is_match(preceding_text) {
            last_signal_start = start;
            false
        } else if do_re.is_match(preceding_text) {
            last_signal_start = start;
            true
        } else {
            is_summing
        };

        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        let full_match = cap[0].to_string();
        let product = x * y;

        if is_summing {
            total_sum += product;
        }

        matches.push((full_match, product, is_summing));
    }

    (matches, total_sum)
}

fn main() {
    let test_string = r"don't()do()mul(3,3)";
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
}