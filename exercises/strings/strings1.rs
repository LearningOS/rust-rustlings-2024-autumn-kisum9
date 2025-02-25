// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

// Solution 1
// fn current_favorite_color() ->  &'static str {
//     "blue"
// }

// Solution 2
// fn current_favorite_color() -> String {
//     "blue".to_string()
// }

// Solution 3
fn current_favorite_color() -> String {
    String::from("blue")
}