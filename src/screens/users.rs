// use super::timesheets::load_hourly_users;
use crate::components::user_form::UserForm;
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

#[component]
pub fn AdminUsers(children: Children) -> impl IntoView {
    view! {
        <nav class="subWrapper">
            <a href="">"Users List"</a>
            <a href="create">"Add New User"</a>
        </nav>
        <section class="stack admin users_list">{children()}</section>
    }
}

#[component]
pub fn UsersList() -> impl IntoView {
    // let users = Resource::new(move || {}, move |_| load_hourly_users()).read();
    view! {
        <AdminUsers>
            // {move || match users.clone() {
            // Some(Ok(users)) => {
            // view! {
            // <table>
            // <thead>
            // <tr>
            // <th>Name</th>
            // <th>Phone Number</th>
            // <th>Edit</th>
            // </tr>
            // </thead>
            // // {users
            // //     .into_iter()
            // //     .map(|user| {
            // //         view! {
            <section class="stack">// //             <div class="user_list">
            // //                 <span>{user.last_name} ", " {user.first_name}</span>
            // //                 <span>{user.phone_number}</span>
            // //                 <span>
            // //                     <a href=format!("/app/admin/user/edit/{}", user.id.to_string())>
            // //                         <Icon name="pencil".into()/>
            // //                     </a>
            // //                 </span>
            // //             </div>
            // //         }
            // //     })
            // //     .collect_view()}
            // </table>
            // }
            // .into_any()
            // }
            // Some(Err(e)) => view! { <div>"Error: " {e.to_string()}</div> }.into_any(),
            // None => view! {}.into_any(),
            // }}

            </section>
        </AdminUsers>
    }
}

#[component]
pub fn UserCreate() -> impl IntoView {
    view! {
        <AdminUsers>
            <UserForm uuid=None/>
        </AdminUsers>
    }
}

#[derive(Clone, Params, PartialEq)]
struct UserUpdateParams {
    uuid: Option<String>,
}

#[component]
pub fn UserUpdate() -> impl IntoView {
    let params = use_params::<UserUpdateParams>();

    view! {
        {move || {
            match params.read().clone() {
                Ok(p) => {
                    match p.uuid {
                        Some(uuid) => {
                            view! {
                                <AdminUsers>
                                    <UserForm uuid=Some(uuid.clone())/>
                                </AdminUsers>
                            }
                                .into_any()
                        }
                        None => {
                            view! { <div data-state="error">{"Did not find the user!"}</div> }
                                .into_any()
                        }
                    }
                }
                Err(_) => view! { <div>"Invalid ID"</div> }.into_any(),
            }
        }}
    }
}
