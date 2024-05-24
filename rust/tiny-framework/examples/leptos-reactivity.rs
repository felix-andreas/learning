use leptos_reactive::*;

fn main() {
    let _ = create_runtime();
    run();
}

fn run() {
    let (value, set_value) = create_signal(1);
    let derived = || {
        println!("compute derived");
        value.get() * 2
    };
    let memo = create_memo(move |_| {
        println!("compute memo");
        value.get() * 3
    });
    let resource = create_resource(
        move || (value),
        |number| async move {
            println!("compute resource");
            4 * number.get()
        },
    );
    create_effect(move |_| println!("value changed: {}", value.get()));
    println!(
        "value: {}, derived: {}, memo {}, resource {:?}",
        value.get(),
        derived(),
        memo.get(),
        resource.get(),
    );
    set_value.set(2);
    println!(
        "value: {}, derived: {}, memo {}, resource {:?}",
        value.get(),
        derived(),
        memo.get(),
        resource.get(),
    );
    println!(
        "value: {}, derived: {}, memo {}, resource {:?}",
        value.get(),
        derived(),
        memo.get(),
        resource.get(),
    );
}
