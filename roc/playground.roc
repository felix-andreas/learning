app "playground"
    packages { pf: "platform/main.roc" }
    imports []
    provides [main] to pf


color : [Green, Red]
color = Red

main = when color is
    Green -> Str.concat "Hi " "there!\n"
    Red -> "this is a red tag 🔴\n"

