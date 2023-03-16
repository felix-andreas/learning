app "playground"
    packages { pf: "../../basic-cli/src/main.roc" }
    imports [pf.Task.{ Task }, pf.Stdout, pf.Path]
    provides [main] to pf

main : Task {} []
main = Stdout.line "foo"

