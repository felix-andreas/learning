fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {}",
        area_variables(width1, height1)
    );

    let rect1 = (width1, height1);
    println!("The area of the rectangle is {}", area_tuple(rect1));

    let rect1 = Rect {
        width: width1,
        height: height1,
    };
    println!("The area of the rectangle is {}", area_struct(&rect1));
    println!("The area of the rectangle is {}", rect1.area_struct());
    println!("The area of the rectangle is {}", Rect::area_struct(&rect1));
}

fn area_variables(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple((width, height): (u32, u32)) -> u32 {
    width * height
}

struct Rect {
    width: u32,
    height: u32,
}

fn area_struct(Rect { width, height }: &Rect) -> u32 {
    width * height
}

impl Rect {
    fn area_struct(&self) -> u32 {
        self.width * self.height
    }
}
