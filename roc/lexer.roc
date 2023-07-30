app "nn"
    packages { pf: "https://github.com/roc-lang/basic-cli/releases/download/0.3.2/tE4xS_zLdmmxmHwHih9kHWQ7fsXtJr7W7h3425-eZFk.tar.br" }
    imports [
        pf.Stdout,
        pf.Task.{ Task },
        pf.File,
        pf.Path,
    ]
    provides [main] to pf

main : Task {} []
main =
    result <- Task.attempt task
    when result is
        Ok {} -> Task.succeed {}
        Err _ -> Stdout.line "something went horrible wrong here ..."

task =
    content <- "input.txt" |> Path.fromStr |> File.readUtf8 |> Task.await
    tokens = tokenize content
    lines = tokens |> List.mapWithIndex
        \token, i ->
            number = Num.toStr i
            when token is
                Assign -> "\(number)| ASSIGN"
                LParen -> "\(number)| LPAREN"
                RParen -> "\(number)| RPAREN"
                Plus -> "\(number)| Plus"
                Minus -> "\(number)| Minus"
                Word word -> "\(number)| WORD \(word)"
                Number n ->
                    numberString = Num.toStr n
                    "\(number)| NUMBER \(numberString)"
    Stdout.line (Str.joinWith lines "\n")

tokenize = \content ->
    pushToken = \x, token -> { x & parsed: List.append x.parsed token}
    pushCurrent = \x ->
        { x
        & parsed: when x.current is
            WordC word -> x.parsed |> List.append (Word (word |> Str.fromUtf8 |> Result.withDefault "fuckup"))
            NumberC number -> x.parsed |> List.append (
                string = when Str.fromUtf8 number is
                    Ok asdf -> asdf
                    Err _ -> crash "fuckup"
                when Str.toNat string is
                    Ok n -> Number n
                    Err _ -> crash "fuckup"
            )
            None -> x.parsed
        , current: None
        }
    tmp = content |> Str.toUtf8 |> List.walk { parsed: [], current: None } \state, char ->
        when char is
            ' ' | '\n' -> pushCurrent state
            '(' -> pushCurrent state |> pushToken LParen
            ')' -> pushCurrent state |> pushToken RParen
            '+' -> pushCurrent state |> pushToken Plus
            '-' -> pushCurrent state |> pushToken Minus
            '=' -> pushCurrent state |> pushToken Assign
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' -> when state.current is
                NumberC number -> { state & current: NumberC (List.append number char) }
                WordC word -> { state & current: WordC (List.append word char) }
                None -> { state & current: NumberC [char] }
            _ -> when state.current is
                WordC word -> { state & current: WordC (List.append word char) }
                NumberC number -> { state & current: WordC (List.append number char) }
                None -> { state & current: WordC [char] }

    (pushCurrent tmp).parsed
