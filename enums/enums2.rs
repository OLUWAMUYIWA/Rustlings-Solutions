// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }

    fn print_out(&self) {
        let formatted = format!("This is how we do things here{:?}", &self);
        println!("{:?}", formatted);
    }
}
fn array_of_messages() {
    let my_messages = [
        Message::Move { x: 34, y: 32 },
        Message::Echo(String::from("hi there people")),
        Message::ChangeColor(50, 34, 5),
        Message::Quit,
    ];

    for m in my_messages.iter() {
        m.print_out();
    }
}
fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }

    println!();

    array_of_messages();
}
