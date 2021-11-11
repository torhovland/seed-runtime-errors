// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

use leaflet::{LatLng, Map, TileLayer};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    let (app, msg_mapper) = (orders.clone_app(), orders.msg_mapper());

    // Cannot initialize Leaflet until the map element has rendered.
    orders.after_next_render(|_| {
        let map = Map::new("map", &JsValue::NULL);
        map.setView(&LatLng::new(60.0, 11.0), 14.0);

        TileLayer::new(
            "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
            &JsValue::NULL,
        )
        .addTo(&map);

        let on_move = Closure::wrap(
            Box::new(move || app.update(msg_mapper(Msg::Increment))) as Box<dyn FnMut()>
        );

        map.on("movestart", on_move.as_ref());
        on_move.forget();

        Msg::SetMap(map)
    });

    Model {
        counter: 0,
        map: None,
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    counter: i32,
    map: Option<Map>,
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment,
    MoveHome,
    SetMap(Map),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
        Msg::MoveHome => {
            if let Some(map) = &model.map {
                map.panTo(&LatLng::new(65.0, 12.0));
            }
        }
        Msg::SetMap(map) => model.map = Some(map),
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        C!["counter"],
        button![model.counter, ev(Ev::Click, |_| Msg::Increment),],
        button!["Move home", ev(Ev::Click, |_| Msg::MoveHome),],
        div![id!["map"],],
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
