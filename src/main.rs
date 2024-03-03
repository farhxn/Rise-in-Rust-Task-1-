fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}

fn main() {
    let string1 = String::from("Farhan "); 
    let string2 = String::from("Atif!");
    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("{}", concatenated_string);
}