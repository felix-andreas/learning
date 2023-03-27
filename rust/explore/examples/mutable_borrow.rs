fn main() {
    let mut v = vec![1, 2, 3];
    let mut v2 = Vec::new();

    for i in &mut v {
        v2.push(i);
    }

    v.pop();
    println!("{}", v2[2]);
}
