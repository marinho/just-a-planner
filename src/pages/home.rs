// use crate::components::add_workout_button::AddWorkoutButton;
// use crate::components::workout_view::WorkoutView;
use crate::constants::APP_TITLE;
use crate::models::phase_set::PhaseSet;
use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let (phases, set_phases, _) = use_local_storage::<PhaseSet, JsonSerdeCodec>("phases");

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <div class="container">
                    <header>
                        <h1 class="title">"Uh oh! Something went wrong!"</h1>
                    </header>

                    <p>"Errors: "</p>
                    // Render a list of errors as strings - good for development purposes
                    <ul>
                        {move || {
                            errors
                                .get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                .collect_view()
                        }}

                    </ul>
                </div>
            }
        }>
            <div class="container">
                <header>
                    <h1 class="title">{APP_TITLE}</h1>
                </header>

                <main>
                    <Suspense fallback=move || {
                        view! { <p>"Loading..."</p> }
                    }>Home page content here</Suspense>
                </main>
            </div>
        </ErrorBoundary>
    }
}
