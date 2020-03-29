#![recursion_limit = "512"]

use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use yew::events::KeyboardEvent;
use yew::format::Json;
use yew::services::storage::{Ares, StorageService};
use yew::{html, Component, ComponentLink, Href, Html, InputData, ShouldRender};

const KEY: &'static str = "yew.todomvc.self";

pub struct Model {
	link: ComponentLink<Self>,
	storage: StorageService,
	state: State,
}

// Create the state object for the application
// Seems like this might need to be nested at some point
// depending on how the views are setup.
// @todo -> Figure out routing, and complex state management
#[derive(Serialize, Deserialize)]
pub struct State {
	entries: Vec<Entry>,
	filter: Filter,
	value: String,
	edit_value: String,
}

#[derive(Serialize, Deserialize)]
struct Entry {
	description: String,
	completed: bool,
	editing: bool,
}

// Msg type that could be emitted by the application?
pub enum Msg {
	Add,
	Edit(usize),
	Update(String),
	UpdateEdit(String),
	Remove(usize),
	SetFilter(Filter),
	ToggleAll,
	ToggleEdit(usize),
	Toggle(usize),
	ClearCompleted,
	Nope,
}

// Implement the Component trait for the Model struct
// This seems like how we are telling rust that Model
// implements all of the necissary functionality 
// of a component.
impl Component for Model {
	
}
