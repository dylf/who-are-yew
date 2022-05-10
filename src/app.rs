use yew::prelude::*;
use yew_feather::anchor::Anchor;
use yew_feather::heart::Heart;

#[function_component(App)]
pub fn app() -> Html {
    use_effect(move || {
        log::debug!("Hello, world!");
        || ()
    });

    html! {
        <main class="bg-gray-900 h-screen overflow-hidden">
            <header class="bg-gray-600 text-white p-4">
                <Anchor size="40" class="inline-block align-top mr-2" />
                <h1 class="text-4xl text-bold inline-block">{ "Who Are Yew?" }</h1>
            </header>
            <p class="bg-gray-400 p-9 m-4">{ "from Yew with " }<Heart fill="rgb(236, 72, 153)" class="inline-block" /></p>
        </main>
    }
}
