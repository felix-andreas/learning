import gi
gi.require_version('Gtk', '3.0')
from gi.repository import Gtk
print(Gtk.get_major_version(), Gtk.get_minor_version(),Gtk.get_micro_version())
win = Gtk.Window()
win.connect("destroy", Gtk.main_quit)
win.show_all()
Gtk.main()