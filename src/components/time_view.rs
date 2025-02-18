// use chrono::NaiveDate;
// use leptos::prelude::*;
// use uuid::Uuid;

// #[component]
// pub fn UserTimeOverviewView(
//     user_id: Uuid,
//     from_date: Option<NaiveDate>,
//     to_date: Option<NaiveDate>,
// ) -> impl IntoView {
//     // Use create_resource to call our server function.
//     let fetch_data = Resource::new(
//         move || (user_id, from_date, to_date),
//         move |(user_id, from_date, to_date)| {
//             fetch_user_time_overview_range(user_id, from_date, to_date)
//         },
//     );

//     view! {
//             move || match fetch_data.get() {
//                 None => view! {  <div>"Loading..."</div> }.into_any(),
//                 Some(Err(e)) => view! {  <div class="text-red-600">{format!("Error: {}", e)}</div> }.into_any(),
//                 Some(Ok(rows)) => {
//                     if rows.is_empty() {
//                         view! { <div>{"No records found."}</div> }.into_any()
//                     } else {
//                         view! {
//                             <table class="table-auto border-collapse w-full">
//                                 <thead>
//                                     <tr class="border-b">
//                                         <th class="px-2 py-1 text-left">{"Event Time"}</th>
//                                         <th class="px-2 py-1 text-left">{"State"}</th>
//                                         <th class="px-2 py-1 text-left">{"Reason"}</th>
//                                     </tr>
//                                 </thead>
//                                 <tbody>
//                                     {rows.into_iter().map(|row| {
//                                         let event_time_str = row.event_time.map(|dt| dt.to_rfc3339()).unwrap_or_else(|| "N/A".to_string());

//                                         let correction_state_str = row.correction_state.map(|s| s.to_string()).unwrap_or_else(|| "".to_string());
//                                         let correction_reason = row.correction_reason.unwrap_or_default();

//                                         view! {
//                                             <tr class="border-b">
//                                                 <td class="px-2 py-1">{event_time_str}</td>
//                                                 <td class="px-2 py-1">{correction_state_str}</td>
//                                                 <td class="px-2 py-1">{correction_reason}</td>
//                                             </tr>
//                                         }
//                                     }).collect_view()}
//                                 </tbody>
//                             </table>
//                         }.into_any()
//                    } }
//             },
//         }
//     }
// }

// /*
// USAGE EXAMPLE:

// In your router or main app file:

// #[component]
// fn App() -> impl IntoView {
//     let user_id = Uuid::parse_str("<some-user-uuid>").unwrap();

//     // By default, we won't pass any date range. That means 2 weeks before recent Sunday until now.
//     // If you want a custom date range, you can specify:
//     // let from_date = Some(NaiveDate::from_ymd_opt(2025, 2, 1).unwrap());
//     // let to_date = Some(NaiveDate::from_ymd_opt(2025, 2, 14).unwrap());

//     view! {
//         <main>
//             <UserTimeOverviewView user_id=user_id from_date=None to_date=None />
//         </main>
//     }
// }

// */
