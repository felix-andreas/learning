#[allow(unused_variables)]
fn main() {
    println!("hello world!");

    let stack_f64 = 1f64;
    let heap_f64 = Box::new(1f64);

    stack_procedure(&stack_f64);
    println!("stack_f64 from main {}", stack_f64);

    heap_procedure(&heap_f64);
    println!("stack_f64 from main {}", heap_f64);

    let var_a = 1;
    let mut var_b = &var_a;
    let var_tmp = var_b + 1;
    var_b = &var_tmp;
    let var_c = &var_a;
    println!("{} {} {}", var_a, var_b, var_c);
}

fn stack_procedure(param: &f64) {
    println!("stack_f64 from procedure {}", param);
}

fn heap_procedure(param: &f64) {
    // param += 9f64;
    println!("head_f64 from procedure {}", param)
}
