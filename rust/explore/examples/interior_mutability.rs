use std::cell::RefCell;

fn main() {
    let x = RefCell::new(4);

    {
        let mut m1 = x.borrow_mut();
        *m1 += 1;
    }
    {
        let mut m2 = x.borrow_mut();
        *m2 += 1;
    }

    println!("{:?}", x.borrow());
}
