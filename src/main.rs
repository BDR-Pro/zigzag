use regex::Regex;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }
    let file = std::fs::read_to_string(args[1].clone()).unwrap();
    let rust_code = parse_custom_code_to_rust(file.as_str());
    println!("Generated Rust Code:\n{}", rust_code);
}

fn regex_remove_comments(custom_code: &str) -> String {
    let re = Regex::new(r"#.*").unwrap(); // Fixed regex pattern
    re.replace_all(custom_code, "").to_string()
}

fn regex_remove_spaces(custom_code: String) -> String {
    let re = Regex::new(r"\s+").unwrap();
    re.replace_all(&custom_code, " ").to_string()
}

fn clean_my_code(custom_code: &str) -> String {
    let custom_code_no_comments = regex_remove_comments(custom_code);
    let custom_code_no_extra_spaces = regex_remove_spaces(custom_code_no_comments);
    custom_code_no_extra_spaces.replace("\n", " ")
        .replace('\t', " ")
        .replace('\r', " ")
}

fn line_by_line(custom_code: String) -> Vec<String> {
    custom_code.split(';').map(|s| s.to_string()).collect()
}


fn process_lines(lines: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    if lines.is_empty() {
        return result; // Early return if input is empty
    }

    result.push(lines[0].to_string()); // Initialize result with the first line

    for i in 1..lines.len() {
        let current_line = lines[i].trim();
        if current_line.starts_with("}") && !result.is_empty() {
            let last_index = result.len() - 1;
            // Append the current line (trimmed and without "}") to the previous line
            result[last_index] = format!("{} {}", result[last_index], current_line.replace("}", "").trim());
        } else {
            // Push the current line as a new entry in the result
            result.push(current_line.to_string());
        }
    }

    result
}

fn my_rust_code(lines: Vec<&str>) -> String {
    // Fixed the regex patterns
    let re = Regex::new(r"^(\w+)\s*=\s*(\d+)$").unwrap(); 
    let number_re = Regex::new(r"^(\w+)::(\w+)=([\d]+)$").unwrap(); 
    let string_re = Regex::new(r#"^(\w+)::(\w+)="([^"]*)"$"#).unwrap(); // Corrected the regex for string values
    let mut decleartion_variables = HashMap::new();
    let mut assginment_variables = HashMap::new();let processed_lines = process_lines(&lines);
    for line in processed_lines.iter() {


        println!("{}", line.trim());println!("######");
        

        if let Some(caps) = re.captures(line) {
            let var_name = caps.get(1).unwrap().as_str();
            let var_value = caps.get(2).unwrap().as_str();
            assginment_variables.insert(var_name, var_value);
        } else if let Some(caps) = number_re.captures(line) {
            let var_name = caps.get(1).unwrap().as_str();
            let var_value = caps.get(3).unwrap().as_str();
            decleartion_variables.insert(var_name, var_value);
        } else if let Some(caps) = string_re.captures(line) {
            let var_name = caps.get(1).unwrap().as_str();
            let var_value = caps.get(3).unwrap().as_str();
            decleartion_variables.insert(var_name, var_value);
        }
    }
    
    let mut rust_code = String::new();  
    for (var_name, var_value) in decleartion_variables {
        if var_value.parse::<i32>().is_ok() {
            rust_code.push_str(&format!("let {} = {};\n", var_name, var_value));
        } else {
            rust_code.push_str(&format!("let {} = \"{}\";\n", var_name, var_value));
        }
    }
    for (var_name, var_value) in assginment_variables {
        rust_code.push_str(&format!("let mut {} = {};\n", var_name, var_value));
    }

    rust_code
}

fn lines_to_blocks_regex(lines: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    let re = Regex::new(r"\[\?(.*?)\!\]").unwrap(); // Matches content within [?...!]

    for line in lines.iter() {
        if let Some(captures) = re.captures(line) {
            // If the line contains the pattern, process it
            let matched = captures.get(0).unwrap().as_str();
            let processed = matched.replace('\n', " "); // Replace newlines with spaces
            result.push(processed);
        } else {
            // For lines without the pattern, add them as they are
            result.push(line.clone());
        }
    }

    result
}
fn remove_blocks(lines: Vec<String>) -> Vec<String> {
    let re = Regex::new(r"\[\?(.*?)\!\]").unwrap(); // Matches content within [?...!]
    lines.iter().map(|line| re.replace_all(line, "").to_string()).collect()
}

fn parse_custom_code_to_rust(custom_code: &str) -> String {
    let cleaned_code = clean_my_code(custom_code);
    let lines = line_by_line(cleaned_code);
    let lines_to_blocks_regex = lines_to_blocks_regex(lines);
    let clean_code = remove_blocks(lines_to_blocks_regex);
    let mut rust_code = String::new();
    rust_code.push_str("fn main() {\n");
    rust_code.push_str(&my_rust_code(clean_code.iter().map(|s| s.as_str()).collect::<Vec<&str>>()));
    rust_code.push_str("\n}");
    rust_code
}
