app "hello"
    packages { pf: "/home/felix/Projects/roc/examples/cli/platform" }
    imports [ pf.Stdout ]
    provides [ main ] to pf

main = Stdout.line "I'm a Roc application!"
