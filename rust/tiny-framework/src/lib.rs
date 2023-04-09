use {
    leptos_reactive::Scope,
    std::ops::Deref,
    wasm_bindgen::{JsCast, JsValue},
    web_sys::{console, Document, Element, Event},
};

pub fn mount(f: impl FnOnce(Scope) -> El + 'static) {
    let runtime = leptos_reactive::create_runtime();
    _ = leptos_reactive::create_scope(runtime, |cx| {
        document().body().unwrap().append_child(&f(cx)).unwrap();
    })
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

pub struct El(Element);

impl El {
    pub fn new(tag_name: &str) -> Self {
        let button = document().create_element(tag_name).unwrap();
        El(button)
    }

    pub fn attr(self, attr_name: &str, value: &str) -> Self {
        self.0.set_attribute(attr_name, value).unwrap();
        self
    }

    pub fn child(self, el: El) -> Self {
        self.0.append_child(&el).unwrap();
        self
    }

    pub fn text(self, text: &str) -> Self {
        self.0
            .append_child(&document().create_text_node(text))
            .unwrap();
        self
    }

    pub fn text_dyn(self, cx: Scope, f: impl Fn() -> String + 'static) -> Self {
        let text_node = document().create_text_node("");
        self.0.append_child(&text_node).unwrap();
        leptos_reactive::create_effect(cx, move |_| text_node.set_data(&f()));
        self
    }

    pub fn on(self, event_name: &str, callback: impl FnMut(Event) + 'static) -> Self {
        use wasm_bindgen::prelude::Closure;
        let cb = Closure::wrap(Box::new(callback) as Box<dyn FnMut(Event)>);
        self.0
            .add_event_listener_with_callback(event_name, cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();
        self
    }
}

impl Deref for El {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
