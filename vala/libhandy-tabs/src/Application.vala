public class LibHandyTabs.Application : Gtk.Application {
    public Application () {
        Object (
            application_id: "com.github.andreasfelix.libhandytabs",
            flags: ApplicationFlags.FLAGS_NONE
        );
    }

    protected override void activate () {
        var main_window = new MainWindow () {
            application = this,
            default_width = 800,
            default_height = 300,
            title = "Hello World"
        };
        main_window.show_all ();
    }

    public static int main (string[] args) {
        return new LibHandyTabs.Application ().run (args);
    }
}
