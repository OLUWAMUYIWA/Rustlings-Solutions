// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age)/* your pattern here */ = cat;

    println!("{} is {} years old.", name, age);
    destructure();
}

struct Person<'a> {
    name: &'a str,
    age: u8,
    likes: Vec<String>,
}

fn destructure() {
    let me = Person {
        name: "oluwamuyiwa Elijah",
        age: 25u8,
        likes: vec![
            "fiction".to_string(),
            "places".to_string(),
            "beauty".to_string(),
        ],
    };
    let Person { name, age, likes } = me;
    println!("name: {:?}, age: {:?}, likes: {:?}", name, age, likes);
}
