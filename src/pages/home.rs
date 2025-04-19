// use crate::components::add_workout_button::AddWorkoutButton;
// use crate::components::workout_view::WorkoutView;
use crate::constants::APP_TITLE;
// use crate::models::workout_day::WorkoutDay;
use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    // let (workout_day, set_workout_day, _) =
    //     use_local_storage::<WorkoutDay, JsonSerdeCodec>("today");

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

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
            }
        }>
            <header>
                <h1>{APP_TITLE}</h1>
            </header>

            <div class="container">
                <main>
                    <Suspense fallback=move || view! { <p>"Loading..."</p> }>Home page</Suspense>
                </main>
            </div>
        </ErrorBoundary>
    }
}
