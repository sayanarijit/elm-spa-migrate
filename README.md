# elm-spa-migrate

Helper utility to auto migrate [elm-spa](https://www.elm-spa.dev/) generated pages.

Usage:

Assuming we have page `src/Pages/Home_.elm`

```elm
module Pages.Home_ exposing (view)

import Html
import View exposing (View)


view : View msg
view =
    { title = "Homepage"
    , body = [ Html.text "Hello, world!" ]
    }
```

Run

```
elm-spa-migrate --dry-run src/Pages/Home_.elm static
```

Expected result

```elm
module Pages.Home_ exposing (page)
import Shared
import Request exposing (Request)
import Page exposing (Page)
import Gen.Params.Home_ exposing (Params)

import Html
import View exposing (View)



view :   View msg
view   =
    View.placeholder "Hello World"

-- view : View msg
-- view =
--     { title = "Homepage"
--     , body = [ Html.text "Hello, world!" ]
--     }

page : Shared.Model -> Request.With Params -> Page
page shared req =
    Page.static
        { view = view
        }
```

Continuing with this file, run

```
elm-spa-migrate --dry-run src/Pages/Home_.elm element
```

Expected result (after some elm-format)

```elm
module Pages.Home_ exposing (Model, Msg, page)

import Gen.Params.Home_ exposing (Params)
import Html
import Page exposing (Page)
import Request exposing (Request)
import Shared
import View exposing (View)


view : Model -> View Msg
view model =
    View.placeholder "Hello World"



-- view : View msg
-- view =
--     View.placeholder "Hello World"
--
--
--
-- view : View msg
-- view =
--     { title = "Homepage"
--     , body = [ Html.text "Hello, world!" ]
--     }


page : Shared.Model -> Request.With Params -> Page.With Model Msg
page shared req =
    Page.element
        { init = init
        , update = update
        , view = view
        , subscriptions = subscriptions
        }



-- page : Shared.Model -> Request.With Params -> Page
-- page shared req =
--     Page.static
--         { view = view
--         }


type alias Model =
    {}


type Msg
    = ReplaceMe


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


init : ( Model, Cmd Msg )
init =
    ( {}, Cmd.none )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        _ ->
            ( model, Cmd.none )
```

Continuing with this file, run

```
elm-spa-migrate --dry-run --request --shared src/Pages/Home_.elm element
```

Expected result (after some elm-format)

```elm
module Pages.Home_ exposing (Model, Msg, page)

import Gen.Params.Home_ exposing (Params)
import Html
import Page exposing (Page)
import Request exposing (Request)
import Shared
import View exposing (View)


view : Shared.Model -> Request.With Params -> Model -> View Msg
view shared req model =
    View.placeholder "Hello World"



-- view : Model -> View Msg
-- view model =
--     View.placeholder "Hello World"
--
--
--
-- view : View msg
-- view =
--     View.placeholder "Hello World"
--
--
--
-- view : View msg
-- view =
--     { title = "Homepage"
--     , body = [ Html.text "Hello, world!" ]
--     }


page : Shared.Model -> Request.With Params -> Page.With Model Msg
page shared req =
    Page.element
        { init = init shared req
        , update = update shared req
        , view = view shared req
        , subscriptions = subscriptions shared req
        }



-- page : Shared.Model -> Request.With Params -> Page.With Model Msg
-- page shared req =
--     Page.element
--         { init = init
--         , update = update
--         , view = view
--         , subscriptions = subscriptions
--         }
--
--
--
-- page : Shared.Model -> Request.With Params -> Page
-- page shared req =
--     Page.static
--         { view = view
--         }


type alias Model =
    {}


type Msg
    = ReplaceMe


subscriptions : Shared.Model -> Request.With Params -> Model -> Sub Msg
subscriptions shared req model =
    Sub.none



-- subscriptions : Model -> Sub Msg
-- subscriptions model =
--     Sub.none
--
--


init : Shared.Model -> Request.With Params -> ( Model, Cmd Msg )
init shared req =
    ( {}, Cmd.none )



-- init : ( Model, Cmd Msg )
-- init =
--     ( {}, Cmd.none )
--
--


update : Shared.Model -> Request.With Params -> Msg -> Model -> ( Model, Cmd Msg )
update shared req msg model =
    case msg of
        _ ->
            ( model, Cmd.none )



-- update : Msg -> Model -> ( Model, Cmd Msg )
-- update msg model =
--     case msg of
--         _ ->
--             ( model, Cmd.none )

```

Continuing with this file, run

```
elm-spa-migrate --dry-run --shared src/Pages/Home_.elm advanced
```

Expected result (after some elm-format)


```elm
module Pages.Home_ exposing (Model, Msg, page)

import Effect exposing (Effect)
import Gen.Params.Home_ exposing (Params)
import Html
import Page exposing (Page)
import Request exposing (Request)
import Shared
import View exposing (View)


view : Shared.Model -> Model -> View Msg
view shared model =
    View.placeholder "Hello World"



-- view : Shared.Model -> Request.With Params -> Model -> View Msg
-- view shared req model =
--     View.placeholder "Hello World"
--
-- view : Model -> View Msg
-- view model =
--     View.placeholder "Hello World"
--
--
--
-- view : View msg
-- view =
--     View.placeholder "Hello World"
--
--
--
-- view : View msg
-- view =
--     { title = "Homepage"
--     , body = [ Html.text "Hello, world!" ]
--     }


page : Shared.Model -> Request.With Params -> Page.With Model Msg
page shared req =
    Page.advanced
        { init = init shared
        , update = update shared
        , view = view shared
        , subscriptions = subscriptions shared
        }



-- page : Shared.Model -> Request.With Params -> Page.With Model Msg
-- page shared req =
--     Page.element
--         { init = init shared req
--         , update = update shared req
--         , view = view shared req
--         , subscriptions = subscriptions shared req
--         }
--
-- page : Shared.Model -> Request.With Params -> Page.With Model Msg
-- page shared req =
--     Page.element
--         { init = init
--         , update = update
--         , view = view
--         , subscriptions = subscriptions
--         }
--
--
--
-- page : Shared.Model -> Request.With Params -> Page
-- page shared req =
--     Page.static
--         { view = view
--         }


type alias Model =
    {}


type Msg
    = ReplaceMe


subscriptions : Shared.Model -> Model -> Sub Msg
subscriptions shared model =
    Sub.none



-- subscriptions : Shared.Model -> Request.With Params -> Model -> Sub Msg
-- subscriptions shared req model =
--     Sub.none
--
-- subscriptions : Model -> Sub Msg
-- subscriptions model =
--     Sub.none
--
--


init : Shared.Model -> ( Model, Effect Msg )
init shared =
    ( {}, Effect.none )



-- init : Shared.Model -> Request.With Params -> (Model, Cmd Msg)
-- init shared req =
--     ({}, Cmd.none)
--
-- init : ( Model, Cmd Msg )
-- init =
--     ( {}, Cmd.none )
--
--


update : Shared.Model -> Msg -> Model -> ( Model, Effect Msg )
update shared msg model =
    case msg of
        _ ->
            ( model, Effect.none )



-- update : Shared.Model -> Request.With Params -> Msg -> Model -> ( Model, Cmd Msg )
-- update shared req msg model =
--     case msg of
--         _ ->
--             ( model, Cmd.none )
--
-- update : Msg -> Model -> ( Model, Cmd Msg )
-- update msg model =
--     case msg of
--         _ ->
--             ( model, Cmd.none )
```

Now try reverting back.
