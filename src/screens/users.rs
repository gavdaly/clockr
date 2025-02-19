use super::timesheets::load_hourly_users;
use crate::components::icon::Icon;
use crate::components::user_form::UserForm;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

/// Renders the home page of your application.
#[component]
pub fn Users() -> impl IntoView {
    let users = Resource::new(move || {}, move |_| load_hourly_users()).read();
    view! {
        <section class="stack users_list">
            <table>
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Phone Number</th>
                    </tr>
                </thead>
                {move || match users.clone() {
                    Some(Ok(users)) => {
                        users
                            .into_iter()
                            .map(|user| {
                                view! {
                                    <tr>
                                        <td>{user.last_name} ", " {user.first_name}</td>
                                        <td>{user.phone_number}</td>
                                    </tr>
                                }
                            })
                            .collect_view()
                            .into_any()
                    }
                    Some(Err(e)) => view! { <div>"Error: " {e.to_string()}</div> }.into_any(),
                    None => view! {}.into_any(),
                }}

            </table>
        </section>
    }
}

#[component]
pub fn AdminUsers(children: Children) -> impl IntoView {
    view! {
        <nav class="subWrapper">
            <A href="" exact=true>
                "Users List"
            </A>
            <A href="create" exact=true>
                "Add New User"
            </A>
        </nav>
        <section class="stack admin users_list">{children()}</section>
    }
}

#[component]
pub fn UsersList() -> impl IntoView {
    let users = Resource::new(move || {}, move |_| load_hourly_users()).read();
    view! {
        <AdminUsers>
            <section class="stack">
                {move || match users.clone() {
                    Some(Ok(users)) => {
                        view! {
                            <table>
                                <thead>
                                    <tr>
                                        <th>Name</th>
                                        <th>Phone Number</th>
                                        <th>Edit</th>
                                    </tr>
                                </thead>
                                {users
                                    .into_iter()
                                    .map(|user| {
                                        view! {
                                            <div class="user_list">
                                                <span>{user.last_name} ", " {user.first_name}</span>
                                                <span>{user.phone_number}</span>
                                                <span>
                                                    <A href=format!("/admin/user/edit/{}", user.id.to_string())>
                                                        <Icon name="pencil".into()/>
                                                    </A>
                                                </span>
                                            </div>
                                        }
                                    })
                                    .collect_view()}
                            </table>
                        }
                            .into_any()
                    }
                    Some(Err(e)) => view! { <div>"Error: " {e.to_string()}</div> }.into_any(),
                    None => view! {}.into_any(),
                }}

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
