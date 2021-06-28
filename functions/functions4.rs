// functions4.rs
// Make me compile! Execute `rustlings hint functions4` for hints :)

// This store is having a sale where if the price is an even number, you get
// 10 Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: i32) -> i64 {
    if is_even(price) {
        (price - 10) as i64
    } else {
        (price - 3) as i64
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
