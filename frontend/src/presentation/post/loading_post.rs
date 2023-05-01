use yew::prelude::*;

#[function_component(LoadingPost)]
pub fn loading_post() -> Html {
    html! {
        <div class="relative w-full cursor-pointer">
            <div class="py-3 bg-white rounded-md max-w-full bg-opacity-60 font-semibold text-transparent animate-pulse shadow-sm duration-500 px-3 sm:px-6 text-sm sm:text-base">
                <div class="bg-gray-300 h-6 w-1/2 mb-2 rounded" />
                <div class="flex text-transparent font-thin text-sm">
                    <div class="bg-gray-300 h-3 w-1/4 rounded" />
                    <div class="ml-2 my-auto">
                        <div class="bg-gray-300 h-3 w-1/4 rounded" />
                    </div>
                </div>
            </div>
        </div>
    }
}
