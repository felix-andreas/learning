// dummy for tailwind flex
fn main() {
    console_error_panic_hook::set_once();

    let document = web_sys::window()
        .and_then(|win| win.document())
        .expect("Could not access document");
    let body = document.body().expect("Could not access document.body");
    let text_node = document.create_text_node("Hello, world from Vanilla Rust!");
    let div = document.create_element("div").unwrap();
    div.set_class_name("p-4 border rounded-md");
    div.append_child(&text_node).expect("Failed to append text");
    body.append_child(&div).expect("Failed to append text");
}
