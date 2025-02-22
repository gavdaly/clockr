use crate::components::{Icon, Loading};
use crate::functions::user::use_user;
use crate::models::user::CurrentUser;
use leptos::html::Dialog;
use leptos::prelude::*;

#[island]
pub fn Menu() -> impl IntoView {
    let current_user = use_user();
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
                        <Show when=move || {
                            if let CurrentUser::Authenticated(user) = current_user.get() {
                                user.state == 1
                            } else {
                                false
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

            <Suspense fallback=move || {
                view! { <Loading/> }
            }>
                {move || {
                    let user_context = use_user();
                    match user_context.get() {
                        CurrentUser::Authenticated(_) => {
                            view! {
                                <button on:click=open_dialog>
                                    <Icon name="horizontal-menu".into()/>
                                </button>
                            }
                                .into_any()
                        }
                        CurrentUser::Guest => view! { <button id="nav">"login"</button> }.into_any(),
                    }
                }}

            </Suspense>
        </div>
    }
}