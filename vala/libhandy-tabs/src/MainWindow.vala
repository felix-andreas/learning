public class LibHandyTabs.MainWindow : Hdy.ApplicationWindow {
    private Hdy.TabView tab_view;
    private Hdy.TabBar tab_bar;

    construct {
        Hdy.init ();

        var headerbar = new Hdy.HeaderBar () {
            hexpand = true,
            show_close_button = true
        };

        unowned var header_context = headerbar.get_style_context ();
        header_context.add_class (Granite.STYLE_CLASS_DEFAULT_DECORATION);
        header_context.add_class (Gtk.STYLE_CLASS_FLAT);

        var button_add_tab = new Gtk.Button.from_icon_name ("list-add-symbolic", Gtk.IconSize.MENU) {
            tooltip_text = "New Tab",
            margin = 6
        };
        headerbar.add (button_add_tab);

        var menu = new GLib.Menu ();
        menu.append_item (new GLib.MenuItem ("hello menu!", null));

        tab_view = new Hdy.TabView () {
            vexpand = true,
            visible = true,
            menu_model = menu,
            shortcut_widget = this
        };
        tab_bar = new Hdy.TabBar () {
            view = tab_view,
            autohide = false,
            inverted = true,
            //  start_action_widget = button_add_tab,
            hexpand = true
        };

        create_tab ("Initial 1");
        create_tab ("Initial 2");

        var window_handle = new Hdy.WindowHandle ();
        window_handle.add (tab_view);
        var grid = new Gtk.Grid () {
            orientation = Gtk.Orientation.VERTICAL
        };
        grid.add (headerbar);
        grid.add (tab_bar);
        grid.add (window_handle);

        int i = 3;
        button_add_tab.clicked.connect (() => {
            create_tab ("Label %d".printf (i++));
        });

        add (grid);
    }

    private void create_tab (string title) {
        var entry = new Gtk.Entry () { 
            visible = true,
            text = title,
            halign = Gtk.Align.CENTER,
            valign = Gtk.Align.CENTER
        };
        var tab_page = tab_view.add_page (entry, null);
        entry.bind_property(
            "text",
            tab_page,
            "title", 
            GLib.BindingFlags.SYNC_CREATE | GLib.BindingFlags.BIDIRECTIONAL
        );
        tab_view.set_selected_page (tab_page);
    }
}
