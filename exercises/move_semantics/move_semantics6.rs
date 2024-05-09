// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

fn get_char(data: &String) -> Option<char> 
{
    data.chars().last()
}

fn string_uppercase(mut data: String) 
{
    data = data.to_uppercase();

    println!("{}", data);
}

