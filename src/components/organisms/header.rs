use crate::components::atoms::button::Button;
use yew::prelude::*;

#[function_component]
pub fn Header() -> Html {
    html! {
        <header class="bg-gray-800 text-white p-4 w-full flex flex-row justify-between items-center">
            <h1 class="text-2xl font-semibold">{"Crispina Dive"}</h1>
            <Button />
        </header>
    }
}
