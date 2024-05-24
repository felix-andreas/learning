use {
    leptos_reactive::*,
    tiny_framework::{self as tf, El},
};

fn main() {
    tf::mount(app);
}

fn app() -> El {
    let (count, set_count) = create_signal(0);
    El::new("div")
        .child(
            El::new("button")
                .on("click", move |_| set_count.update(|n| *n -= 1))
                .attr("id", "decrement")
                .text("-1"),
        )
        .text_dyn(move || count.get().to_string())
        .child(
            El::new("button")
                .on("click", move |_| set_count.update(|n| *n += 1))
                .attr("id", "increment")
                .text("+1"),
        )
}
