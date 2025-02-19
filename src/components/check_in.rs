use crate::functions::CheckIn;
use leptos::form::ActionForm;
use leptos::prelude::*;

/// This function returns a view component based on whether it is being rendered on the server side or client side.
///
/// # Arguments
///
/// * `check_in` - An `Action` object representing a check-in action.
#[component]
pub fn CheckInView() -> impl IntoView {
    let check_in = ServerAction::<CheckIn>::new();

    view! {
        <div class="center-center stack">
            <ActionForm action=check_in>"Click"</ActionForm>
        </div>
    }
}
