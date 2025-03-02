use crate::components::{Icon, Loading};
use crate::models::CurrentUser;
use leptos::html::Dialog;
use leptos::prelude::*;

#[component]
pub fn Menu() -> impl IntoView {
    let user_context =
        use_context::<Resource<CurrentUser>>().expect("Not wrapped in `UserProvider`");
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
                    <Icon name="close"/>
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
                        <Suspense fallback=move || {
                            view! { <Loading/> }
                        }>
                            {move || match user_context.read().clone() {
                                Some(CurrentUser::Authenticated(u)) => {
                                    view! {
                                        <Show when=move || u.state == 1>
                                            <h2>"Admin"</h2>
                                            <li>
                                                <a href="/admin/timesheets">"timesheets"</a>
                                            </li>
                                            <li>
                                                <a href="/admin/users">"users"</a>
                                            </li>
                                        </Show>
                                    }
                                        .into_any()
                                }
                                _ => {}.into_any(),
                            }}

                        </Suspense>
                    </ul>
                </menu>
            </nav>
        </dialog>

        <div id="nav">

            <Suspense fallback=move || {
                view! { <Loading/> }
            }>
                {move || match user_context.read().clone() {
                    Some(CurrentUser::Authenticated(_)) => {
                        view! {
                            <button on:click=open_dialog>
                                <Icon name="horizontal-menu"/>
                            </button>
                        }
                            .into_any()
                    }
                    _ => view! { <button>"login"</button> }.into_any(),
                }}

            </Suspense>

        </div>
    }
}
