use leptos::prelude::*;

use crate::models::sessions::Session;
use crate::models::time_sheets::TimeSheet;
use crate::utils::miliseconds_to_string;

#[component]
pub fn TimeSheetDisplay(timesheet: TimeSheet) -> impl IntoView {
    let (_entry_total, _adjustment_total, statuatory_total, vacation_total) =
        timesheet.summary_totals;
    let entries = timesheet.entries.clone();
    move || {
        view! {
            <table id="timesheet_summary">
                <thead>
                    <tr>
                        <th>"Day"</th>
                        <th>"Hours"</th>
                        {if statuatory_total > 0 {
                            view! { <th>"Statutory"</th> }.into_any()
                        } else {
                            view! {}.into_any()
                        }}

                        {if vacation_total > 0 {
                            view! { <th>"Vacation"</th> }.into_any()
                        } else {
                            view! {}.into_any()
                        }}

                    </tr>
                </thead>
                {timesheet
                    .summary
                    .iter()
                    .map(|(day, (time, b, c, d))| {
                        view! {
                            <tr>
                                <td data-title="Day">{day.to_string()}</td>
                                <td data-title="Hours">{miliseconds_to_string(&(time + b))}</td>
                                {if statuatory_total > 0 {
                                    view! {
                                        <td data-title="Statutory">{miliseconds_to_string(c)}</td>
                                    }
                                        .into_any()
                                } else {
                                    view! {}.into_any()
                                }}

                                {if vacation_total > 0 {
                                    view! {
                                        <td data-title="Vacation">{miliseconds_to_string(d)}</td>
                                    }
                                        .into_any()
                                } else {
                                    view! {}.into_any()
                                }}

                            </tr>
                        }
                    })
                    .collect_view()}
            </table>

            <table>
                <thead>
                    <tr>
                        <th>"Day"</th>
                        <th>"Entries"</th>
                    </tr>
                </thead>

                {entries
                    .iter()
                    .map(|(day, entries)| {
                        view! {
                            <tr class="entry">
                                <td>{day.to_string()}</td>
                                <td class="entries">
                                    {entries
                                        .iter()
                                        .map(|_| view! {  })
                                        .collect_view()}
                                </td>
                            </tr>
                        }
                    })
                    .collect_view()}
            </table>
        }
    }
}

#[component]
fn EntryEdit(_entry: Session) -> impl IntoView {
    view! { <div></div> }
}
