use {leptos_reactive::*, tiny_framework::El};

fn main() {
    console_error_panic_hook::set_once();
    tiny_framework::mount(app);
}
fn app() -> El {
    let count = RwSignal::new(0);
    El::new("div")
        .attr("style", "display: flex; gap: 1rem;")
        .child(
            El::new("button")
                .attr("id", "log")
                .text("log value")
                .on("click", move |_| {
                    tiny_framework::log_str(&count().to_string())
                }),
        )
        .child(
            El::new("button")
                .attr("id", "dec")
                .text("-")
                .on("click", move |_| count.update(|x| *x -= 1)),
        )
        .text(" Value: ")
        .dyn_text(move || count().to_string())
        .child(
            El::new("button")
                .attr("id", "inc")
                .text("+")
                .on("click", move |_| count.update(|x| *x += 1)),
        )
}
