app "playground"
    packages { pf: "../../roc/examples/platform-switching/c-platform/main.roc" }
    imports []
    provides [main] to pf


color : [Green, Red]
color = Red

main = when color is
    Green -> Str.concat "Hi " "there!\n"
    Red -> "this is a red tag 🔴\n"

