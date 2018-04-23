#![recursion_limit="500"]

#[macro_use]
extern crate stdweb;

use stdweb::web::{
    Element,
    document
};
use stdweb::web::event::ClickEvent;
use stdweb::traits::*;


macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

type Reducer = fn(State) -> State;
type Handler = fn() -> Reducer;

struct State {
    count: i32,
}

struct Store {
    state: State,
    subscribers: Vec<Reducer>
}

impl Store {
    fn new(state: State) -> Store {
        Store {
            state: state,
            subscribers: Vec::new()
        }
    }

    fn subscribe(&mut self, reducer: Reducer) -> () {
        self.subscribers.push(reducer);
    }
}

struct Event {
    event: String,
    selector: String,
    handler: Handler
}

struct View {
    html: String,
    events: Vec<Event>
}

fn increment_reducer(state:State) -> State {
    State { count: state.count + 1 }
}

fn increment_handler() -> Reducer {
    increment_reducer
}

fn decrement_reducer(state:State) -> State {
    State { count: state.count - 1 }
}

fn decrement_handler() -> Reducer {
    decrement_reducer
}

fn view(state: State) -> View {
    let html = format!("
        <h1>Click the buttons below!</h1>
        <button id=\"increment\" type=\"button\">+</button>
        <button id=\"decrement\" type=\"button\">-</button>
        <p>Count: {count}</p>
    ", count = state.count);

    let increment = Event {
        event: String::from("click"),
        selector: String::from("#increment"),
        handler: increment_handler
    };

    let decrement = Event {
        event: String::from("click"),
        selector: String::from("#decrement"),
        handler: decrement_handler
    };

    View {
        html,
        events: vec![ increment, decrement ]
    }
}

fn add_listeners(events: Vec<Event>){
    for event in events {
        document()
            .query_selector(&event.selector)
            .unwrap()
            .unwrap()
            .add_event_listener(|_event: ClickEvent| {
                console!(log, "Something was Clicked");


            });
    }
}


fn main() {
    let state = State { count: 0 };
    let store = Store::new(state);
    let initial_view = view(store.state);
    let root = document()
        .get_element_by_id("root")
        .unwrap();

    js! {
        @(no_return)
        @{ root }.innerHTML = @{ initial_view.html };
    };


    add_listeners(initial_view.events);
}
