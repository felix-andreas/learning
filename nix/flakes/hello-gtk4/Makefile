hello-gtk4: hello-gtk4.c
	gcc -o hello-gtk4 hello-gtk4.c `pkg-config --cflags --libs gtk4`

install: hello-gtk4
	mkdir -p $(out)/bin
	cp hello-gtk4 $(out)/bin/
