app "playground"
    packages { pf: "https://github.com/roc-lang/basic-cli/releases/download/0.3.1/97mY3sUwo433-pcnEQUlMhn-sWiIf_J9bPhcAFZoqY4.tar.br" }
    imports [pf.Task.{ Task }, pf.Stdout]
    provides [main] to pf

main : Task {} []
main =
    [
        stringsAndNumbers,
        ifelse,
        records,
        tags,
    ]
    |> List.map (\x -> Str.joinWith x "\n")
    |> Str.joinWith "\n\n"
    |> Stdout.line

stringsAndNumbers : List Str
stringsAndNumbers =
    name = "Max"
    stringInterpolation = "Hello \(name)!"
    arithmetic = Num.toStr (1 + 3)
    callingFunctions = Str.concat "Hi, " "there"
    [
        "Strings And Numbers",
        "  stringInterpolation: \(stringInterpolation)",
        "  arithmetic: \(arithmetic)",
        "  callingFunctions: \(callingFunctions)",
    ]

ifelse : List Str
ifelse =
    count = 4
    result = if count == 4 then "foo" else "bar"
    [
        "If .. then .. else ..",
        "  ifelse: \(result)",
    ]

records : List Str
records =
    notUsed = 10
    data = { birds: 5, iguanas: 7, notUsed }
    addAndStringify = \counts -> counts.birds + counts.iguanas |> Num.toStr
    total = addAndStringify data
    addAndStringifyDestructuring = \{ birds, iguanas } -> birds + iguanas |> Num.toStr
    totalDestructering = addAndStringifyDestructuring data
    numBirds = .birds data |> Num.toStr
    numBirdsDestructering =
        { birds } = data
        Num.toStr birds
    # original = { birds: 5, zebras: 3, iguanas: 7, goats: 1 }
    # fromOriginal = { original & birds: 4 }
    # encoded = Encode.toBytes fromOriginal Json.toUtf8 |> Str.fromUtf8 |> Result.withDefault "rip"
    [
        "Records",
        "  total: \(total)",
        "  totalDestructering: \(totalDestructering)",
        "  numBirds: \(numBirds)",
        "  numBirdsDestructering: \(numBirdsDestructering)",
        # "  fromOriginal: \(encoded)",
    ]

tags : List Str
tags =
    color = \x ->
        if x > 5 then
            Red
        else if x > 10 then
            Blue
        else
            Custom "#18fa19"
    string = when color 3 is
        Red -> "red"
        Blue -> "blue"
        Custom value -> value
    [
        "Tags",
        "  color: \(string)"
    ]
