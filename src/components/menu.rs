use crate::components::icon::Icon;
use crate::screens::authenticate::Logout;
use leptos::prelude::*;
use leptos_router::components::A;

/// Generates a menu component for a web application.
///
/// # Arguments
///
/// * `status` - A function that returns a boolean value indicating the current status.
/// * `log_out` - An action that triggers the logout functionality.
/// * `show_menu` - A signal variable that indicates whether the menu should be shown or hidden.
/// * `set_show_menu` - A signal variable that allows updating the value of `show_menu`.
#[component]
pub fn Menu(
    log_out: ServerAction<Logout>,
    show_menu: ReadSignal<bool>,
    set_show_menu: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <nav aria-label="Main Menu" id="nav" data-visible=move || show_menu().to_string()>
            <span>
                <button class="close nav-button" on:click=move |_| set_show_menu(false)>
                    <Icon name="close".into()/>
                </button>
            </span>
            <menu>
                <li>
                    <A href="/app" exact=true>
                        "dashboard"
                    </A>
                </li>
                <li>
                    <A href="/app/check_in">
                        "check in/out"
                    </A>
                </li>
                <li>
                    <A href="/app/timesheet">
                        "timesheet"
                    </A>
                </li>
                <li>
                    <A href="/app/vacations" >
                        "vacations"
                    </A>
                </li>
                <li>
                    <A href="/app/users"  >
                        "users"
                    </A>
                </li>
                // <Show when=move || user().state == 1>
                    <h2>"Admin"</h2>
                    <li>
                        <A href="/admin/timesheets" >
                            "timesheets"
                        </A>
                    </li>
                    <li>
                        <A href="/admin/vacations">
                            "vacations"
                        </A>
                    </li>
                // </Show>
            </menu>

            <ActionForm action=log_out>
                <button type="submit">
                    <span>"Logout"</span>
                    <Icon name="logout".into()/>
                </button>
            </ActionForm>
        </nav>
        <button class="nav-button" >
            <Icon name="horizontal-menu".into()/>
        </button>
    }
}
