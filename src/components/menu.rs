use crate::components::Icon;
use crate::models::UserToday;
use crate::screens::authenticate::Logout;
use leptos::html::Dialog;
use leptos::prelude::*;

#[island]
pub fn Menu(user: UserToday) -> impl IntoView {
    let log_out = ServerAction::<Logout>::new();
    let dialog_ref = NodeRef::<Dialog>::new();
    let open_dialog = move |_| {
        let Some(dialog) = dialog_ref.get() else {
            tracing::error!("Dialog reference is None");
            return;
        };
        dialog.show_modal().expect("Dialog should open");
    };

    let close_dialog = move |_| {
        let Some(dialog) = dialog_ref.get() else {
            tracing::error!("Dialog reference is None");
            return;
        };
        dialog.close();
    };
    view! {
        <dialog node_ref=dialog_ref class="menu-dialog">
            <nav aria-label="Main Menu">
                <button class="close-button" on:click=close_dialog>
                    <Icon name="close".into()/>
                    <span class="sr-only">"Close Navigation Menu"</span>
                </button>
                <menu>
                    <ul>
                        <li>
                            <a href="/app">"dashboard"</a>
                        </li>
                        <li>
                            <a href="/app/timesheet">"timesheet"</a>
                        </li>
                        <Show when=move || user.state == 1>
                            <h2>"Admin"</h2>
                            <li>
                                <a href="/admin/timesheets">"timesheets"</a>
                            </li>
                            <li>
                                <a href="/admin/users">"users"</a>
                            </li>
                        </Show>
                    </ul>
                </menu>
            </nav>
        </dialog>

        <div id="nav">
            <ActionForm action=log_out>
                <button class="logout-button" type="submit">
                    <span>"Logout"</span>
                    <Icon name="logout".into()/>
                </button>
            </ActionForm>
            <button on:click=open_dialog>
                <Icon name="horizontal-menu".into()/>
            </button>
        </div>
    }
}
