module Main exposing (main)

import Browser
import Html exposing (..)
import Html.Attributes exposing (class)
import Html.Events exposing (onClick)
import Http
import Json.Decode exposing (Decoder, Error(..), field, int, map4, string)
import Random
import Task
import Time



-- MAIN


main : Program () Model Msg
main =
    Browser.element
        { init = init
        , update = update
        , view = view
        , subscriptions = subscriptions
        }



-- MODEL


type QuoteResult
    = Success Quote
    | Empty
    | Loading
    | Failure


type alias Quote =
    { quote : String
    , source : String
    , author : String
    , year : Int
    }


type alias Model =
    { quote : QuoteResult
    , dieFace : Int
    , zone : Time.Zone
    , time : Time.Posix
    }


init : () -> ( Model, Cmd Msg )
init _ =
    ( { quote = Empty
      , dieFace = 1
      , zone = Time.utc
      , time = Time.millisToPosix 0
      }
    , Task.perform AdjustTimeZone Time.here
    )



-- UPDATE


type Msg
    = GetQuote
    | GotQuote (Result Http.Error Quote)
    | Roll
    | NewFace Int
    | Tick Time.Posix
    | AdjustTimeZone Time.Zone


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GetQuote ->
            ( { model | quote = Loading }
            , getQuote
            )

        GotQuote result ->
            ( { model
                | quote =
                    case result of
                        Ok quote ->
                            Success quote

                        Err _ ->
                            Failure
              }
            , Cmd.none
            )

        Roll ->
            ( model
            , Random.generate NewFace (Random.int 1 6)
            )

        NewFace dieFace ->
            ( { model | dieFace = dieFace }
            , Cmd.none
            )

        Tick time ->
            ( { model | time = time }
            , Cmd.none
            )

        AdjustTimeZone zone ->
            ( { model | zone = zone }
            , Cmd.none
            )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions _ =
    Time.every 1000 Tick



-- VIEW


view : Model -> Html Msg
view model =
    div [ class "p-6 min-h-screen bg-gray-50 grid gap-12" ]
        [ h1 [ class "font-medium text-xl" ] [ text "Commands and Subscriptions" ]

        -- HTTP
        , div [ class "grid gap-6" ]
            [ h2 [ class "font-medium text-lg" ] [ text "HTTP" ]
            , div [ class "grid gap-6" ]
                [ viewButton "Get Quote" GetQuote
                , div
                    [ class "p-4 min-h-32 bg-white border border-gray-100 rounded-md shadow" ]
                    [ case model.quote of
                        Loading ->
                            div [ class "h-full grid place-items-center" ]
                                [ text "Loading ..."
                                ]

                        Empty ->
                            div [ class "h-full grid place-items-center" ]
                                [ text "Please load a quote :)"
                                ]

                        Failure ->
                            div [ class "h-full grid place-items-center font-medium text-red-600" ]
                                [ text "Unable to load a quote :/"
                                ]

                        Success quote ->
                            div [ class "grid gap-2" ]
                                [ h3
                                    [ class "font-medium" ]
                                    [ text quote.source ]
                                , p [ class "px-4 flex gap-4 text-sm text-center text-gray-700 italic" ]
                                    [ span [ class "text-2xl" ] [ text "»" ]
                                    , span [ class "block py-8" ] [ text quote.quote ]
                                    , span [ class "self-end text-2xl" ] [ text "«" ]
                                    ]
                                , div
                                    [ class "text-center" ]
                                    [ text <| String.join ", " [ quote.author, String.fromInt quote.year ] ]
                                ]
                    ]
                ]
            ]

        -- Random
        , div [ class "grid gap-6" ]
            [ h2 [ class "font-medium text-lg" ] [ text "Random" ]
            , div
                [ class "bg-white p-16 text-center text-gray-800 border border-gray-100 rounded-lg shadow" ]
                [ text <| String.fromInt model.dieFace ]
            , viewButton "Roll" Roll
            ]

        -- Time
        , div [ class "grid gap-6" ]
            [ h2 [ class "font-medium text-lg" ] [ text "Time" ]
            , div
                [ class "bg-white p-4 text-lg text-center text-gray-800 border border-gray-100 rounded-md shadow tabular-nums" ]
                [ text <| formatTime model.zone model.time ]
            ]
        ]


viewButton : String -> Msg -> Html Msg
viewButton content msg =
    button
        [ onClick msg
        , class "px-3 py-2 bg-green-500 font-medium text-white border border-green-600 rounded-md shadow-sm"
        , class "focus:outline-none focus:ring-3 focus:ring-green-300 focus:border-green-700"
        ]
        [ text content ]


formatTime : Time.Zone -> Time.Posix -> String
formatTime zone time =
    [ Time.toHour, Time.toMinute, Time.toSecond ]
        |> List.map (\func -> func zone time |> String.fromInt |> String.padLeft 2 '0')
        |> String.join ":"



-- HTTP


getQuote : Cmd Msg
getQuote =
    Http.get
        { url = "https://elm-lang.org/api/random-quotes"
        , expect = Http.expectJson GotQuote quoteDecoder
        }


quoteDecoder : Decoder Quote
quoteDecoder =
    map4 Quote
        (field "quote" string)
        (field "source" string)
        (field "author" string)
        (field "year" int)
