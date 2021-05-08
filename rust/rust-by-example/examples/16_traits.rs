use rand;

struct Sheep {
    name: &'static str,
    naked: bool,
}

struct Cow {
    name: &'static str,
}

trait Animal {
    fn name(&self) -> &'static str {
        let name: &'static str = "hello world";
        name
    }
    fn noise(&self) -> &'static str;
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Animal for Sheep {
    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.naked {
            "bääääääh"
        } else {
            "määääääh"
        }
    }
}

impl Animal for Cow {
    fn name(&self) -> &'static str {
        self.name
    }
    fn noise(&self) -> &'static str {
        let noise: &'static str = "muuuuuh";
        noise
    }
}

fn random_animal(name: &'static str, random_number: f64) -> Box<dyn Animal> {
    if random_number > 0.5 {
        Box::new(Cow { name })
    } else {
        Box::new(Sheep { name, naked: false })
    }
}

fn main() {
    let mut dolly: Sheep = Sheep {
        name: "Dolly",
        naked: false,
    };
    dolly.talk();
    dolly.naked = true;
    dolly.talk();

    let animal = random_animal("Leo", rand::random::<f64>());
    println!(
        "The random animal {} says {}",
        animal.name(),
        animal.noise()
    )
}
