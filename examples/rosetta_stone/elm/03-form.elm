import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput)



-- MAIN


main =
  Browser.sandbox { init = init, update = update, view = view }



-- MODEL


type alias Model =
  { name : String
  , password : String
  , passwordAgain : String
  }


init : Model
init =
  Model "" "" ""



-- UPDATE


type Msg
  = Name String
  | Password String
  | PasswordAgain String


update : Msg -> Model -> Model
update msg model =
  case msg of
    Name name ->
      { model | name = name }

    Password password ->
      { model | password = password }

    PasswordAgain password ->
      { model | passwordAgain = password }



-- VIEW


view : Model -> Html Msg
view model =
  div [ 
    style "display" "grid",
    style "grid-template-columns" "1fr 1fr 1fr",
    style "grid-template-rows" "auto auto",
    style "align-items" "center"
    ]

    [ div [style "grid-row" "1 / 2", style "grid-column" "1 / 2"] 
      [viewInput "text" "Name" model.name Name
    ]

    , div [style "grid-row" "1 / 2", style "grid-column" "2 / 3"] 
      [viewInput "password" "Password" model.password Password
      ]

    , div [style "grid-row" "1 / 2", style "grid-column" "3 / 4"] 
     [viewInput "password" "Re-enter Password" model.passwordAgain PasswordAgain
     ]

    , div [style "grid-row" "2 / 3", style "grid-column" "1 / 4"] 
      [viewValidation model
    ]
  ]


viewInput : String -> String -> String -> (String -> msg) -> Html msg
viewInput t p v toMsg =
  input [ type_ t, placeholder p, value v, onInput toMsg ] []


viewValidation : Model -> Html msg
viewValidation model =
  if validate model then
    div [ style "color" "green" ] [ text "OK" ]
  else
    div [ style "color" "red" ] [ text "Passwords do not match!" ]

validate : Model -> Bool
validate model = 
  if model.password == model.passwordAgain && String.length model.password > 8 then
    True
  else
    False
