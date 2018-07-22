#[macro_use]
extern crate stdweb;

use stdweb::web::{document};
use stdweb::web::event::ClickEvent;
use stdweb::traits::*;

type Listener = fn(Store) -> ();
type Reducer = fn(State) -> State;

#[derive(Debug, Copy, Clone)]
struct State {
    count: i32,
}

#[derive(Debug,Clone)]
struct Store {
    state: State,
    subscribers: Vec<Box<Listener>>
}

impl Store {
    fn new(state: State) -> Store {
        Store {
            state: state,
            subscribers: Vec::new()
        }
    }

    fn get_state(&self) -> State {
        self.state.clone()
    }

    fn dispatch(&mut self, reducer: &Reducer) -> () {
        self.state = reducer(self.get_state());

        for sub in &self.subscribers {
            let new_state = self.clone();

            sub(new_state);
        }
    }

    fn subscribe(&mut self, listener: Listener) -> () {
        self.subscribers.push(Box::new(listener));
    }
}

struct Event {
    selector: String,
    handler: Reducer
}

struct View {
    html: String,
    events: Vec<Event>
}

fn increment_reducer(state:State) -> State {
    State { count: state.count + 1 }
}

fn decrement_reducer(state:State) -> State {
    State { count: state.count - 1 }
}

fn view(store: &Store) -> View {
    let state = store.get_state();
    let html = format!("
        <h1>Click the buttons below!</h1>
        <button id=\"increment\" type=\"button\">+</button>
        <button id=\"decrement\" type=\"button\">-</button>
        <p>Count: {count}</p>
    ", count = state.count);

    let increment = Event {
        selector: String::from("#increment"),
        handler: increment_reducer
    };

    let decrement = Event {
        selector: String::from("#decrement"),
        handler: decrement_reducer
    };

    View {
        html,
        events: vec![ increment, decrement ]
    }
}

fn add_listeners(events: Vec<Event>, store: Store) -> () {
    for event in events {
        let mut new_store = store.clone();
        let element = document()
            .query_selector(&event.selector)
            .unwrap()
            .unwrap();

        let listener = move |_event: ClickEvent| {
            new_store.dispatch(&event.handler);
        };

        element.add_event_listener(listener);
    }
}

fn render(store: Store) {
    let initial_view = view(&store);
    let root = document()
        .get_element_by_id("root")
        .unwrap();

    js! {
        @(no_return)
        @{ root }.innerHTML = @{ initial_view.html };
    };

    add_listeners(initial_view.events, store);
}

fn main() {
    let state = State { count: 0 };
    let mut store = Store::new(state);

    store.subscribe(render);

    render(store);
}
