use leptos::prelude::*;

use crate::models::user::User;

#[server]
async fn submit_user_form(
    user_id: Option<String>,
    first_name: String,
    last_name: String,
    phone_number: String,
    email: Option<String>,
    state: i32,
) -> crate::Result<User> {
    let _ = crate::functions::current_admin_user_id().await?;

    let email = email
        .map(|email| email.trim().to_ascii_lowercase())
        .filter(|email| !email.is_empty());

    match user_id {
        Some(id) => User {
            id,
            first_name,
            last_name,
            phone_number,
            email,
            state,
        }
        .update()
        .await
        .map_err(Into::into),
        None => User::insert(&first_name, &last_name, &phone_number, email, state)
            .await
            .map_err(Into::into),
    }
}

#[server]
async fn load_user_form(user_id: String) -> crate::Result<User> {
    use crate::models::user::UserDB;
    use uuid::Uuid;

    let _ = crate::functions::current_admin_user_id().await?;

    let user_id = Uuid::parse_str(&user_id).map_err(|_| crate::Error::Unauthorized)?;

    UserDB::get(user_id)
        .await
        .map(User::from)
        .map_err(Into::into)
}

#[component]
pub fn UserForm(uuid: Option<String>) -> impl IntoView {
    let user_resource = uuid
        .clone()
        .map(|id| Resource::new(move || id.clone(), load_user_form));

    view! {
        {move || match user_resource {
            Some(resource) => {
                view! {
                    <Suspense fallback=move || view! { <p>"Loading user"</p> }>
                        {move || match resource.get() {
                            Some(Ok(user)) => view! { <UserFormFields user=Some(user)/> }.into_any(),
                            Some(Err(error)) => {
                                view! { <p data-state="error">{error.to_string()}</p> }.into_any()
                            }
                            None => view! { <p>"Loading user"</p> }.into_any(),
                        }}
                    </Suspense>
                }
                    .into_any()
            }
            None => view! { <UserFormFields user=None/> }.into_any(),
        }}
    }
}

#[component]
fn UserFormFields(user: Option<User>) -> impl IntoView {
    use leptos::form::ActionForm;

    let action = ServerAction::<SubmitUserForm>::new();
    let value = action.value();
    let pending = action.pending();

    let user_id = user.as_ref().map(|user| user.id.clone());
    let first_name = user
        .as_ref()
        .map(|user| user.first_name.clone())
        .unwrap_or_default();
    let last_name = user
        .as_ref()
        .map(|user| user.last_name.clone())
        .unwrap_or_default();
    let phone_number = user
        .as_ref()
        .map(|user| user.phone_number.clone())
        .unwrap_or_default();
    let email = user
        .as_ref()
        .and_then(|user| user.email.clone())
        .unwrap_or_default();
    let state = user.as_ref().map(|user| user.state).unwrap_or(2);

    view! {
        <ActionForm action>
            {user_id
                .map(|user_id| view! { <input name="user_id" type="hidden" value=user_id/> })}
            <div>
                <label>"First Name"</label>
                <input type="text" name="first_name" placeholder="First Name" value=first_name required/>
            </div> <div>
                <label>"Last Name"</label>
                <input type="text" name="last_name" placeholder="Last Name" value=last_name required/>
            </div> <div>
                <label>"Phone Number"</label>
                <input type="tel" name="phone_number" placeholder="Phone Number" value=phone_number required/>
            </div> <div>
                <label>"Email"</label>
                <input type="email" name="email" placeholder="Email" value=email/>
            </div> <fieldset class="picklist" name="state">
                <label>"User Type"</label>
                <div>
                    <label for="user">"User"</label>
                    <input id="user" name="state" type="radio" value="2" checked=state == 2/>
                </div>
                <div>
                    <label for="admin">"Admin"</label>
                    <input id="admin" name="state" type="radio" value="1" checked=state == 1/>
                </div>
                <div>
                    <label for="inactive">"Inactive"</label>
                    <input id="inactive" name="state" type="radio" value="0" checked=state == 0/>
                </div>
            </fieldset>
            <button type="submit" disabled=move || pending.get()>
                {move || if pending.get() { "Saving" } else { "Save user" }}
            </button>
            {move || {
                value
                    .get()
                    .map(|result| match result {
                        Ok(_) => view! { <p>"Saved."</p> }.into_any(),
                        Err(error) => {
                            view! { <p data-state="error">{error.to_string()}</p> }.into_any()
                        }
                    })
            }}
        </ActionForm>
    }
}
