# get a shell with gtk4 and pkg-config

```sh
nix-shell -p gtk4 pkg-config
```


compile with (or see makefile):


```sh
gcc -o hello-gtk `pkg-config --cflags --libs gtk4`
```
