use gloo::utils::document;

fn main() {
    let doc = document();
    let body = doc.query_selector("body").unwrap().unwrap();
    let button = doc.create_element("button").unwrap();
    button.set_inner_html("Hello world!");
    body.append_child(&button).unwrap();
}
