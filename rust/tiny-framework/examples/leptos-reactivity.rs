use leptos_reactive::*;

fn main() {
    _ = create_scope(create_runtime(), |cx| {
        run(cx);
    })
}

fn run(cx: Scope) {
    let (value, set_value) = create_signal(cx, 1);
    let derived = || {
        println!("compute derived");
        value() * 2
    };
    let memo = create_memo(cx, move |_| {
        println!("compute memo");
        value() * 3
    });
    let resource = create_resource(cx, value, |number| async move {
        println!("compute resource");
        4 * number
    });
    create_effect(cx, move |_| println!("value changed: {}", value()));
    println!(
        "value: {}, derived: {}, memo {}, resource {:?}",
        value(),
        derived(),
        memo(),
        resource.read(cx),
    );
    set_value(2);
    println!(
        "value: {}, derived: {}, memo {}, resource {:?}",
        value(),
        derived(),
        memo(),
        resource.read(cx),
    );
    println!(
        "value: {}, derived: {}, memo {}, resource {:?}",
        value(),
        derived(),
        memo(),
        resource.read(cx),
    );
}
