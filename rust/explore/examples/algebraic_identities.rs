/// see here https://youtu.be/LkqTLJK2API?t=3460

fn main() {
    /*
     * IDENTITY: 1 * a = a
     */

    // unit is identity
    type B = ((), bool);
    let b: B = ((), true);

    // this is not allowed
    // let b: B = true

    println!("{b:?}");
}
