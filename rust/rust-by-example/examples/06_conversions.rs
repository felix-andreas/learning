fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);

    println!("{}", my_string);

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    let number = Number::from(30);
    println!("My number is {:?}.", number);

    let int = 5;
    let number: Number = int.into();
    println!("My number is {:?}.", number);

    struct Circle {
        radius: i32,
    }

    impl std::fmt::Display for Circle {
        fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            let radius = self.radius;
            for i in -radius..=radius {
                for j in -radius..=radius {
                    write!(
                        formatter,
                        "{}",
                        if i * i + j * j <= radius * radius {
                            "o"
                        } else {
                            "."
                        }
                    )?;
                }
                write!(formatter, "\n",)?;
            }
            Ok(())
        }
    }

    let circle = Circle { radius: 10 };
    println!("{}", circle);

    // called turbo fish because looks like a fish ::<> with a turbine
    let turbo_parsed = "10".parse::<i32>().unwrap();
    println!("turbo parsed! {}", turbo_parsed);
}
