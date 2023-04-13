app "http"
    packages { pf: "https://github.com/roc-lang/basic-cli/releases/download/0.3.1/97mY3sUwo433-pcnEQUlMhn-sWiIf_J9bPhcAFZoqY4.tar.br" }
    imports [pf.Http, pf.Task.{ Task }, pf.Stdin, pf.Stdout]
    provides [main] to pf

main : Task {} []
main =
    _ <- Task.await (Stdout.line "Enter a URL to fetch. It must contain a scheme like \"http://\" or \"https://\".")

    url <- Task.await Stdin.line

    request = {
        method: Get,
        headers: [],
        url,
        body: Http.emptyBody,
        timeout: NoTimeout,
    }

    output <- Http.send request
        |> Task.onFail (\err -> err |> Http.errorToString |> Task.succeed)
        |> Task.await

    Stdout.line output
