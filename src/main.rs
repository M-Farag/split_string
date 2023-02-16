use std::io;
fn main() {
    let mut separator:String = String::new();
    println!("Please define a separator ?!");
    io::stdin().read_line(&mut separator).expect("Err reading your input");
    let separator:char = separator.chars().take(1).last().unwrap();
    
    
    
}
