use yew::prelude::*;

#[function_component]
pub fn Button() -> Html {
    html! {
        <button class="font-semibold border border-gray-500 px-4 py-2 rounded-xl">{"Hello World!"}</button>
    }
}
