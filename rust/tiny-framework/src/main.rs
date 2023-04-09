use {
    leptos_reactive::{create_signal, Scope, SignalUpdate},
    tiny_framework::{self as tf, El},
};

fn main() {
    tf::mount(app);
}

fn app(cx: Scope) -> El {
    let (count, set_count) = create_signal(cx, 0);
    El::new("div")
        .child(
            El::new("button")
                .on("click", move |_| set_count.update(|n| *n -= 1))
                .attr("id", "decrement")
                .text("-1"),
        )
        .text_dyn(cx, move || count().to_string())
        .child(
            El::new("button")
                .on("click", move |_| set_count.update(|n| *n += 1))
                .attr("id", "increment")
                .text("+1"),
        )
}
