use crate::components::icon::Icon;
use crate::models::user::UserDisplay;
use crate::screens::authenticate::Logout;
use leptos::html::Dialog;
use leptos::prelude::*;

#[island]
pub fn Menu() -> impl IntoView {
    // let _user = expect_context::<ReadSignal<Option<UserDisplay>>>();
    let log_out = ServerAction::<Logout>::new();
    let dialog_ref = NodeRef::<Dialog>::new();
    let open_dialog = move |_| {
        let Some(dialog) = dialog_ref.get() else {
            // !TODO: log error
            return;
        };
        dialog.show_modal().expect("Dialog should open");
    };

    // Handler to close the dialog.
    let close_dialog = move |_| {
        let Some(dialog) = dialog_ref.get() else {
            // !TODO: log error
            return;
        };
        dialog.close();
    };
    view! {
        <dialog node_ref=dialog_ref>
            <nav aria-label="Main Menu">
                <button class="close-button" on:click=close_dialog>
                    <Icon name="close".into()/>
                    <span class="sr-only">"Close Navigation Menu"</span>
                </button>
                <menu>
                    <li>
                        <a href="/app">
                            "dashboard"
                        </a>
                    </li>
                    <li>
                        <a href="/app/check_in">
                            "check in/out"
                        </a>
                    </li>
                    <li>
                        <a href="/app/timesheet">
                            "timesheet"
                        </a>
                    </li>
                    <li>
                        <a href="/app/users"  >
                            "users"
                        </a>
                    </li>
                    // <Show when=move || user().state == 1>
                        <h2>"Admin"</h2>
                        <li>
                            <a href="/admin/timesheets" >
                                "timesheets"
                            </a>
                        </li>
                    // </Show>
                </menu>

                <ActionForm action={log_out}>
                    <button class="logout-button" type="submit">
                        <span>"Logout"</span>
                        <Icon name="logout".into()/>
                    </button>
                </ActionForm>
            </nav>
        </dialog>
        <button id="nav" on:click=open_dialog>
            <Icon name="horizontal-menu".into()/>
        </button>
    }
}
