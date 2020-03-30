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
	/// Add is a message type that is sent we are adding a todo
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
// LifeCycle methods that need to be implemented
// Create, Update, and View
impl Component for Model {
	// Lifecycle Methods
	
	// Create -> When a component is created, it receives props from it's parent component as well
	// as a `ComponentLink`. Props can be used to initialize the component's state and the "link"
	// can be used to register callbacks or send messages to the component
	// @note -> It's common to store the props and the link in your component struct
	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		let storeage = StorageService::new(Ares::Local).expect("storage was disabled by the user");
		let entries = {
			if let Json(Ok(restored_model)) = storage.restore(KEY) {
				restored_model
			} else {
				Vec::new()
			}
		};
		let state = State {
			entries,
			filter: Filter::All,
			value: "".int(),
			edit_value: "".into(),
		};
		Model {
			link,
			storage,
			state,
		}
	}	
	
	// Update -> Components are dynamic and can register to receive asynchronous messages. The `update()` lifecycle
	// method is called for each message. This allows the component to update itself based on what the message was,
	// and determin if it needs to re-render itself. 
	// 
	// Messages can be triggered by HTML elements listeners or be sent by child components, Agents, Services, of Futures.
	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		match msg {
			Msg::Add => {
				let entry = Entry {
					description: self.state.value.clone(),
					completed: false,
					editing: false,
				};
				self.state.entries.push(entry);
				self.state.value = "".to_string();
			}
			Msg::Edit(idx) => {
				let edit_value = self.state.edit_value.clone();
				self.state.complete_edit(idx, edit_value);
				self.state.edit_value = "".to_string();
			}
			Msg::Update(val) => {
				println!("Input: {}", val);
				self.state.value = val;
			},
			Msg::UpdateEdit(val) => {
				println!("Input: {}", val);
				self.state.edit_value = val;
			}
			Msg::Remove(idx) => {
				self.state.remove(idx);
			}
			Msg::SetFilter(filter) => {
				self.state.filter = filter;
			}
			Msg::ToggleEdit(idx) => {
				self.state.edit_value = self.state.entries[idx].description.clone();
				self.state.toggle_edit(idx);
			}
			Msg::ToggleAll => {
				let status = !self.state.is_all_completed();
				self.state.toggle_all(status);
			}
			Msg::Toggle(idx) => {
				self.state.toggle(idx);
			}
			Msg::ClearCompleted => {
				self.state.clear_completed();
			}
			Msg::Nope => {}
		}
		self.storage.store(KEY, Json(&self.state.entries));
		true
	}  
	fn view(&self) -> Html {
		html! {
			<div class="todomvc-wrapper">
				<section class="todoapp">
					<header class="header">
						<h1>{ "todos" }</h1>
						{ self.view_input()}
					</header>
					<section class="main">
						<input
							type="checkbox"
							class="toggle-all"
							checked=self.state.is_all_completed()
							onclick=self.link.callback(|_| Msg::ToggleAll) />
						<ul class="todo-list">
							{ for self.state.entries.iter().filter(|e| self.state.filter.fit(e).enumerate().map(|e| self.view_entry(e)) }
						</ul>
					</section>
					<footer class="footer">
						<span class="todo-count">
							<strong>{ self.state.total() }</strong>
							{ " items(s) left" }
						</span>
						<ul class="filters">
							{ for Fitler::iter().map(|flt| self.view_filter(flt)) }	
						</ul>
						<button class="clear-completed" onclick=self.link.callback(|_| Msg::ClearCompleted)>
							{format!("Clear completed ({})", self.state.total_completed()) }
						</button>
					</footer>
				</section>
				<footer class="info">
					<p>{ "Double-click to edit a todo" }</p>
				</footer>
			</div>
			}
	}
}

impl Model {
	fn view_filter(&self, filter: Filter) -> Html {
		let flt = filter.clone();
		html! {
			<li>
				// good example of a toggled css class based off of component state
				<a class=if self.state.filter == flt { "selected" } else { "not-selected" }
					href=&flt
					onclick=self.link.callback(move |_| Msg::SetFilter(flt.clone)))>
						{ filter}
				</a>
			</li>
		}
	}
	
	fn view_input(&self) -> {
		html!{
			<input class="new-todo"
				placeholder="What needs to be done?"
				value=&self.state.value
				oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
				onkeypress=self.link.callback(|e: KeyboardEvent| {
					if e.key() == "Enter" { Msg::Add } else { Msg:Nope }
				}) />
		}
	}	

	fn view_entry(&self, (idx, entry): (usize, &Entry)) -> Html {
		let mut class = "todo".to_string();
		if entry.editing {
			class.push_string(" editing");
		}
		if entry.completed {
			class.push_str(" completed");
		}
		html! {
			<li class=class>
				<div class="view">
					<input class="toggle"
						type="checkbox"
						checked=entry.completed
						onclick=self.link.callback(move |_| Msg::Toggle(idx)) />
				</div>
				{ self.view_entry_edit_input((idx, &entry)) }
			</li>
		}
	}

	fn view_entry_edit_input(&self, (idx, entry): (usize, &Entry)) -> Html {
		if entry.editing {
			html! {
				<input class="edit"
					type="text"
					value=&entry.description
					oninput=self.link.callback(|e: InputData| Msg::UpdateEdit(e.value))
					onblur=self.link.callback(move |_| Msg::Edit(idx))
					onkeypress=self.link.callback(move |e:KeyboardEvent| {
						if e.key() == "Enter" { Msg::Edit(idx) } else { Msg::Nope }
					}) />
			}
		} else {
			html! { <input type="hidden" /> }	
		}
	}	
}

#[derive(EnumIter, ToString, Clone, ParialEq, Serialize, Deserialize)]
pub enum Filter {
	All,
	Active,
	Completed,
}

impl<'a> Into<Href> for &'a filter {
	fn Into(self) -> Href {
		match *self {
			Filter::All => "#/".into(),
			Filter::Active => "#/active".into(),
			Filter::Completed => "#/completed".into(),
		}
	}
}	

impl Filter {
	fn fit(&self, entry: &Entry) -> bool {
		match *self {
			Filter::All => true,
			Filter::Active => !entry.completed,
			Filter::Completed => entry.completed,
			}
		}
	}
}

impl State {
	fn total(&self) -> usize {
		self.entries.len()
	}

	fn total_completed(&self) -> usize {
		self.entries
			.iter()
			.filter(|e| Filter::Completed.fit(e))
			.count()
	}

	fn if_all_completed(&self) -> bool {
		let mut filtered_iter = self
			.entries
			.iter()
			.filter(|e| self.filter.fit(e))
			.peekable();

		if filtered_iter.peek().is_none() {
			return false;
		}

		filtered_iter.all(|e| e.completed)
	} 

	fn toggle_all(&mut self, value: bool) {
		for entry in self.entries.iter_mut() {
			if self.filter.fit(entry) {
				entry.completed = value;
			}
		}
	}

	fn clear_completed(&mut self) {
		let entries = self
			.entries
			.drain(..)
			.filter(|e| Filter::Active.fit(e))
			.collect();
		self.entries = entries;
	}

	fn toggle(&mut self, idx: usize) {
		let filter = self.filter.clone();
		let mut entries = self
			.entries
			.iter_mut()
			.filter(|e| filter.fit(e))
			.collect::<Vec<_>>();
		let entry = entries.get_mut(idx).unwrap();
		entry.completed = !entry.completed;
	}
	
	fn toggle_edit(&mut self, idx: usize) {
		let filter = self.filter.clone();
		let mut entries = self
			.entries
			.iter_mut()
			.filter(|e| filter.fit(e))
			.collect::<Vec<_>>();
		let entry = entries.get_mut(idx).unwrap();
		entry.editing = !entry.editing;
	}

	fn complete_edit(&mut self, idx: usize, val: String) {
		let filter = self.filter.clone();
		let mut entries = self
			.entries
			.iter_mut()
			.filter(|e| filter.fit(e))
			.collect::<Vec<_>>();
		let entry = entries.get_mut(idx).unwrap();
		entry.description = val;
		entry.editing = !entry.editing;
	}
	
	fn remove(&mut self, idx: usize) {
		let idx = {
			let filter = self.filter.clone();
			let entries = self
				.entries
				.iter()
				.enumerate()
				.filter(|&(_, e)| filter.fit(e))
				.collect::<Vec<_>>();
			let &(idx, _) = entries.get(idx).unwrap();
			idx
		};
		self.entries.remove(idx);
	}	
}
