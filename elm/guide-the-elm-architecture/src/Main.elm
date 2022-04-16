module Main exposing (main)

import Browser
import Char exposing (isDigit)
import Html exposing (Html, a, button, div, h1, h2, input, text)
import Html.Attributes exposing (class, type_, value)
import Html.Events exposing (onClick, onInput)


type alias Model =
    { count : Int
    , text : String
    , name : String
    , password : String
    , passwordConfirmation : String
    }


type Msg
    = Increment
    | Decrement
    | Reset
    | Change String
    | Name String
    | Password String
    | PasswordConfirmation String


main : Program () Model Msg
main =
    Browser.sandbox
        { init = init
        , update = update
        , view = view
        }


init : Model
init =
    { count = 0
    , text = "Hello World!"
    , name = "Foo Bar"
    , password = ""
    , passwordConfirmation = ""
    }


update : Msg -> Model -> Model
update msg model =
    case msg of
        Increment ->
            { model | count = model.count + 1 }

        Decrement ->
            { model | count = model.count - 1 }

        Reset ->
            { model | count = 0 }

        Change content ->
            { model | text = content }

        Name name ->
            { model | name = name }

        Password password ->
            { model | password = password }

        PasswordConfirmation content ->
            { model | passwordConfirmation = content }


view : Model -> Html Msg
view model =
    div [ class "max-w-screen-sm mx-auto p-4 grid gap-4" ]
        [ h1 [ class "font-medium text-2xl text-center" ] [ text "The Elm Architecture" ]
        , div [ class "grid gap-4" ]
            [ h2 [ class "font-medium text-lg" ] [ text "Form" ]
            , viewForm model.name model.password model.passwordConfirmation
            ]
        , div [ class "grid gap-4" ]
            [ h2 [ class "font-medium text-lg" ] [ text "Text Field" ]
            , viewTextField model.text
            ]
        , div [ class "grid gap-4" ]
            [ h2 [ class "font-medium text-lg" ] [ text "Button" ]
            , viewButton model.count
            ]
        ]


viewForm : String -> String -> String -> Html Msg
viewForm name password passwordConfirmation =
    div [ class "grid gap-4" ]
        [ viewInput name "text" Name
        , viewInput password "password" Password
        , viewInput passwordConfirmation "password" PasswordConfirmation
        , viewValidation password passwordConfirmation
        ]


viewInput : String -> String -> (String -> Msg) -> Html Msg
viewInput value_ t msg =
    input
        [ value value_
        , type_ t
        , onInput msg
        , class "p-2 border border-gray-200 rounded shadow-sm focus:outline-none focus:ring-3 focus:border-blue-400"
        ]
        []


viewValidation : String -> String -> Html Msg
viewValidation password passwordConfirmation =
    if String.isEmpty passwordConfirmation then
        div [ class "font-medium text-gray-600" ] [ text "Confirm password" ]

    else if password == passwordConfirmation then
        if String.length password < 8 then
            div [ class "font-medium text-red-600" ]
                [ text "Your password must be at least 8 characters" ]

        else if
            [ Char.isUpper, Char.isLower, Char.isDigit ]
                |> List.all (\func -> String.any func password)
                |> not
        then
            div [ class "font-medium text-red-600" ]
                [ text "Password must contain at least one digit, one lowercase, and one uppercase character." ]

        else
            div [ class "font-medium text-green-600" ] [ text "Save password 👍️" ]

    else
        div [ class "font-medium text-red-600" ] [ text "Passwords don't match!" ]


viewTextField : String -> Html Msg
viewTextField content =
    div [ class "grid gap-4 text-center" ]
        [ input
            [ onInput Change
            , value content
            , class "p-2 border border-gray-200 rounded shadow-sm focus:outline-none focus:ring-3 focus:border-blue-400"
            ]
            []
        , div [ class "flex gap-4 justify-between" ]
            [ div [ class "flex gap-2 items-center" ]
                [ div [ class "text-sm opacity-40" ] [ text "Reverse" ]
                , div [ class "font-medium" ] [ String.reverse content |> text ]
                ]
            , div [ class "flex gap-2 items-center" ]
                [ div [ class "text-sm opacity-40" ] [ text "Length" ]
                , div [ class "font-medium tabular-nums" ] [ String.length content |> String.fromInt |> text ]
                ]
            ]
        ]


viewButton : Int -> Html Msg
viewButton count =
    div [ class "grid grid-cols-3 border border-gray-200 rounded shadow-sm" ]
        [ button
            [ class "p-4 bg-white"
            , onClick Decrement
            ]
            [ text "Minus" ]
        , div
            [ class "grid place-items-center border-x border-gray-100" ]
            [ text (String.fromInt count) ]
        , button
            [ class "p-4 bg-white"
            , onClick Increment
            ]
            [ text "Plus" ]
        , button
            [ class "col-span-full p-4 bg-gray-50 text-red-600 border-t border-gray-200"
            , onClick Reset
            ]
            [ text "Reset" ]
        ]
