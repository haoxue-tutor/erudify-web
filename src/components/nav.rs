use leptos::prelude::*;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="bg-white shadow-lg">
            <div class="max-w-7xl mx-auto px-4">
                <div class="flex justify-between h-16">
                    <div class="flex">
                        <div class="flex-shrink-0 flex items-center">
                            <a href="/" class="text-xl font-bold text-gray-800">"Erudify"</a>
                        </div>
                    </div>
                    <div class="flex items-center space-x-4">
                        <a href="/study" class="text-gray-600 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium">
                            "Study"
                        </a>
                        <a href="/signin" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                            "Login"
                        </a>
                    </div>
                </div>
            </div>
        </nav>
    }
}
