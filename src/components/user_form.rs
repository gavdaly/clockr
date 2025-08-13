use leptos::prelude::*;

use crate::models::user::User;

#[server]
async fn submit_user_form(
    user_id: Option<String>,
    first_name: String,
    last_name: String,
    phone_number: String,
    state: i32,
) -> Result<User, ServerFnError> {
    match user_id {
        Some(id) => User {
            id,
            first_name,
            last_name,
            phone_number,
            state,
        }
        .update()
        .await
        .map_err(|_| ServerFnError::Request("Error Updating User".into())),
        None => User::insert(&first_name, &last_name, &phone_number, state)
            .await
            .map_err(|_| ServerFnError::Request("".into())),
    }
}

#[component]
pub fn UserForm(uuid: Option<String>) -> impl IntoView {
    view! {
        <form>
            {match uuid {
                Some(u) => {
                    view! { <input name="user_id" type="hidden" value=u.to_string()/> }.into_any()
                }
                None => view! { <p>"unauthorized"</p> }.into_any(),
            }}
            <div>
                <label>"First Name"</label>
                <input type="text" placeholder="First Name"/>
            </div> <div>
                <label>"Last Name"</label>
                <input type="text" placeholder="Last Name"/>
            </div> <div>
                <label>"Prefered Name"</label>
                <input type="prefered" placeholder="Prefered Name"/>
            </div> <div>
                <label>"Phone Number"</label>
                <input type="phone" placeholder="Phone Number"/>
            </div> <fieldset class="picklist" name="state">
                <label>"User Type"</label>
                <div>
                    <label for="user">"User"</label>
                    <input id="user" name="state" type="radio" value="2"/>
                </div>
                <div>
                    <label for="admin">"Admin"</label>
                    <input id="admin" name="state" type="radio" value="1"/>
                </div>
                <div>
                    <label for="inactive">"Inactive"</label>
                    <input id="inactive" name="state" type="radio" value="0"/>
                </div>
            </fieldset>
        </form>
    }
}
