use crate::components::{Icon, Loading};
use crate::models::user::CurrentUser;
use leptos::html::Dialog;
use leptos::prelude::*;

#[component]
pub fn Menu(user: CurrentUser) -> impl IntoView {
    let dialog_ref = NodeRef::<Dialog>::new();
    let open_dialog = move |_| {
        let Some(dialog) = dialog_ref.get() else {
            tracing::error!("Dialog reference is None");
            return;
        };
        let _ = dialog.show_modal();
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
                        <Show when={
                            let user = user.clone();
                            move || match user.clone() {
                                CurrentUser::Authenticated(u) => u.state == 1,
                                _ => false,
                            }
                        }>
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

            {
                let user = user.clone();
                move || match user.clone() {
                    CurrentUser::Authenticated(_) => {
                        view! {
                            <button on:click=open_dialog>
                                <Icon name="horizontal-menu".into()/>
                            </button>
                        }
                            .into_any()
                    }
                    _ => view! { <button id="nav">"login"</button> }.into_any(),
                }
            }

        </div>
    }
}