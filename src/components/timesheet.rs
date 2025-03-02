use leptos::prelude::*;

#[component]
pub fn TimeSheetDisplay() -> impl IntoView {
    
    view! {
        <table id="timesheet_summary">
            <thead>
                <tr>
                    <th>"Week"</th>
                    <th>"Hours"</th>
                </tr>
            </thead>
            <tr>
                <td data-title="Week">/* day title goes here */</td>
                <td data-title="Hours">/* total hours goes here */</td>
            </tr>
        </table>
        <table>
            <thead>
                <tr>
                    <th>"Day"</th>
                    <th>"Entries"</th>
                </tr>
            </thead>
            <tr class="entry">
                <td>/* date goes here */</td>
                <td class="entries">
                    <ul>
                        <li>
                            <span>/* start time */</span>
                            <span>/* end time */</span>
                        </li>
                    </ul>
                    // entries would go here
                </td>
                <td>
                    // day total would go here
                </td>
                <td>
                    // add entry button would go here
                </td>
            </tr>
        </table>
    }
}
