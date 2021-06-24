use gtk::prelude::*;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.andreasfelix.playground"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("First GTK Program"));
    window.set_default_size(350, 70);

    let button = gtk::Button::with_label("click me!");
    window.set_child(Some(&button));
    window.show();
}
