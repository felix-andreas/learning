use {
    std::ops::Deref,
    wasm_bindgen::{JsCast, JsValue},
    web_sys::{console, Document, Element, Event},
};

pub fn mount(f: impl FnOnce() -> El) {
    let _ = leptos_reactive::create_runtime();
    document().body().unwrap().append_child(&f()).unwrap();
}

/*
 * UTILS
 */

pub fn document() -> Document {
    web_sys::window().unwrap().document().unwrap()
}

pub fn log_str(data: &str) {
    console::log_1(&JsValue::from_str(data))
}

/*
 * El
 */

#[derive(Debug)]
pub struct El(Element);

impl Deref for El {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl El {
    pub fn new(tag: &str) -> El {
        El(document().create_element(tag).unwrap())
    }
    pub fn attr(self, name: &str, value: &str) -> Self {
        self.0.set_attribute(name, value).unwrap();
        self
    }
    pub fn child(self, child: El) -> Self {
        self.0.append_child(&child.0).unwrap();
        self
    }
    pub fn dyn_text(self, f: impl Fn() -> String + 'static) -> Self {
        let node = document().create_text_node("");
        self.0.append_child(&node).unwrap();
        leptos_reactive::create_effect(move |_| node.set_data(&f()));
        self
    }
    pub fn on(self, event: &str, cb: impl FnMut(Event) + 'static) -> Self {
        use wasm_bindgen::prelude::Closure;
        let listener = Closure::new(cb);
        self.0
            .add_event_listener_with_callback(event, listener.as_ref().unchecked_ref())
            .unwrap();
        listener.forget();
        self
    }
    pub fn text(self, text: &str) -> Self {
        self.0
            .append_child(&document().create_text_node(text))
            .unwrap();
        self
    }
}
