app "io"
    packages { pf: "https://github.com/roc-lang/basic-cli/releases/download/0.3.2/tE4xS_zLdmmxmHwHih9kHWQ7fsXtJr7W7h3425-eZFk.tar.br" }
    imports [pf.Task.{ Task }, pf.Stdout, pf.Stdin]
    provides [main] to pf

main : Task {} []
main = 
    Stdout.line "Guess a number"
        |> Task.await \_ -> Stdin.line
        |> Task.await \guess -> 
            if guess == "5" then
                Stdout.line "You Win"
            else
                Stdout.line "You Lose"
