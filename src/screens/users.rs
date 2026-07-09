// use super::timesheets::load_hourly_users;
use crate::components::user_form::UserForm;
use crate::functions::{CreateMagicInviteLink, CreateMagicRecoveryLink};
use crate::models::user::User;
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

#[component]
pub fn AdminUsers(children: Children) -> impl IntoView {
    view! {
        <nav class="subWrapper">
            <a href="/app/admin/users">"Users List"</a>
            <a href="/app/admin/users/create">"Add New User"</a>
        </nav>
        <section class="stack admin users_list">{children()}</section>
    }
}

#[server]
async fn load_admin_users() -> crate::Result<Vec<User>> {
    use crate::models::user::UserDB;

    let _ = crate::functions::current_admin_user_id().await?;

    let users = UserDB::get_all()
        .await?
        .into_iter()
        .map(User::from)
        .collect();

    Ok(users)
}

#[component]
pub fn UsersList() -> impl IntoView {
    let users = Resource::new(|| (), async move |_| load_admin_users().await);

    view! {
        <AdminUsers>
            <h2>"Users"</h2>
            <Suspense fallback=move || view! { <p>"Loading users"</p> }>
                {move || match users.get() {
                    Some(Ok(users)) => {
                        view! {
                            <table>
                                <thead>
                                    <tr>
                                        <th>"Name"</th>
                                        <th>"Phone"</th>
                                        <th>"Email"</th>
                                        <th>"State"</th>
                                        <th>"Actions"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {users
                                        .into_iter()
                                        .map(|user| {
                                            let user_id = user.id.clone();
                                            view! {
                                                <tr>
                                                    <td>{user.last_name}", "{user.first_name}</td>
                                                    <td>{user.phone_number}</td>
                                                    <td>{user.email.unwrap_or_default()}</td>
                                                    <td>{user.state}</td>
                                                    <td>
                                                        <a href=format!("/app/admin/users/edit/{user_id}")>
                                                            "Edit"
                                                        </a>
                                                        <MagicUserLinks user_id=user.id/>
                                                    </td>
                                                </tr>
                                            }
                                        })
                                        .collect_view()}
                                </tbody>
                            </table>
                        }
                            .into_any()
                    }
                    Some(Err(error)) => {
                        view! { <div data-state="error">{error.to_string()}</div> }.into_any()
                    }
                    None => view! { <p>"Loading users"</p> }.into_any(),
                }}
            </Suspense>
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
    id: Option<String>,
}

#[component]
pub fn UserUpdate() -> impl IntoView {
    let params = use_params::<UserUpdateParams>();

    view! {
        {move || {
            match params.read().clone() {
                Ok(p) => {
                    match p.id {
                        Some(id) => {
                            view! {
                                    <AdminUsers>
                                        <UserForm uuid=Some(id.clone())/>
                                        <MagicUserLinks user_id=id/>
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

#[component]
fn MagicUserLinks(user_id: String) -> impl IntoView {
    let create_invite = ServerAction::<CreateMagicInviteLink>::new();
    let invite_value = create_invite.value();
    let create_recovery = ServerAction::<CreateMagicRecoveryLink>::new();
    let recovery_value = create_recovery.value();
    let invite_user_id = user_id.clone();
    let recovery_user_id = user_id;

    view! {
        <section class="stack">
            <h2>"Links"</h2>
            <ActionForm action=create_invite>
                <input type="hidden" name="user_id" value=invite_user_id/>
                <button type="submit">"Generate invite link"</button>
            </ActionForm>
            <ActionForm action=create_recovery>
                <input type="hidden" name="user_id" value=recovery_user_id/>
                <button type="submit">"Generate recovery link"</button>
            </ActionForm>
            {move || {
                invite_value
                    .get()
                    .map(|result| match result {
                        Ok(link) => view! {
                            <input type="text" readonly value=link/>
                        }
                            .into_any(),
                        Err(error) => {
                            view! { <p data-state="error">{error.to_string()}</p> }.into_any()
                        }
                    })
            }}
            {move || {
                recovery_value
                    .get()
                    .map(|result| match result {
                        Ok(link) => view! {
                            <input type="text" readonly value=link/>
                        }
                            .into_any(),
                        Err(error) => {
                            view! { <p data-state="error">{error.to_string()}</p> }.into_any()
                        }
                    })
            }}
        </section>
    }
}
