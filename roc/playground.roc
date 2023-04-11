app "playground"
    packages { pf: "https://github.com/roc-lang/basic-cli/releases/download/0.3.1/97mY3sUwo433-pcnEQUlMhn-sWiIf_J9bPhcAFZoqY4.tar.br" }
    imports [pf.Task.{ Task }, pf.Stdout]
    provides [main] to pf

main : Task {} []
main = Stdout.line "foo"

