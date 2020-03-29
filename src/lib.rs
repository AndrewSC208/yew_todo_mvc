#![recursion_limit = "512"]

use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use yew::events::KeyboardEvent;
use yew::format::Json;
use yew::services::storage::{Ares, StorageService};
use yew::{html, Component, ComponentLink, Href, Html, InputData, ShouldRender};

const KEY: &'static str = "yew.todomvc.self";


