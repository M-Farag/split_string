use std::io;
fn main() {
    let mut separator:String = String::new();
    println!("Please define a separator ?!");
    io::stdin().read_line(&mut separator).expect("Err reading your input");
    let separator:char = separator.chars().take(1).last().unwrap();
    
    println!("Please input a sentence ?!");
    let mut user_input:String = String::new();
    io::stdin().read_line(&mut user_input).expect("Err reading your input");

    split(separator, &user_input);
    
}

fn split(separator:char,user_input:&String) 
{
    let text_under_processing:&str  = user_input.trim();
    let mut start_index:usize = 0;

    for (end_index, letter) in text_under_processing.chars().enumerate()
    {
        if letter == separator {
            println!("Word is: {}",&text_under_processing[start_index..end_index]);
            start_index = end_index + 1;
        }
    }
    println!("Word is: {}", &text_under_processing[start_index..]);
}