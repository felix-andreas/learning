use {std::ops::Deref, web_sys::Element};

fn main() {
    let node = El::new("button");
    mount(node);
}

fn mount(root: El) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    body.append_child(&root).unwrap();
}

struct El(Element);

impl El {
    fn new(tag_name: &str) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let button = document.create_element(tag_name).unwrap();
        button.set_text_content(Some("hello from tiny framework"));
        El(button)
    }
}

impl Deref for El {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
