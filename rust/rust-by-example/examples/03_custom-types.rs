fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
        height: u8,
    }
    let leo = Person {
        name: String::from("Leo"),
        age: 27,
        height: 191,
    };
    println!("print leo: {:?}", leo);
    let Person { age, name, .. } = leo;
    println!("{} is {} years old", name, age);

    enum Direction {
        North,
        West,
    }

    let direction = if true {
        Direction::North
    } else {
        Direction::West
    };
    match direction {
        Direction::North => println!("the direction is North"),
        Direction::West => println!("the direction is West"),
    }

    const SPEED_OF_LIGHT: u32 = 299_792_458;
    println!("the speed of light is {}", SPEED_OF_LIGHT);
}
