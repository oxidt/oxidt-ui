//! Every component in its awkward states: empty, single, huge and long
//! values, every tone, disabled and loading. Problems that sample data hides
//! show up here first.

use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use crate::demo;
use crate::empty::empty_state;
use crate::modal::{modal, modal_action, modal_close, modal_trigger};
use crate::popover::{popover, popover_trigger};
use crate::radar::{RadarSeries, radar_chart};
use crate::timeline::{ChartPoint, time_chart};
use crate::toast::{ToastTone, toast};

const LONG_ID: &str = "proj_01JB8ZQ4K3V7M2N9XWTR5HCYD6EFGA";
const LONG_TEXT: &str = "The export of 48,213 events from the Lisbon office project finished, but 12 rows were skipped because their timestamps fell outside the retention window.";

/// The states demos, in one theme.
#[component]
pub async fn states(theme: &str) -> Result<impl View> {
    let point = |label: &str, front: i64, back: i64| ChartPoint {
        label: label.into(),
        front,
        back,
    };
    let single = [point("Today", 3, 5)];
    let zeros: Vec<_> = (1..=7).map(|i| point(&format!("#{i}"), 0, 0)).collect();
    let huge: Vec<_> = (1..=12)
        .map(|i| point(&format!("#{i}"), i * 310_000_000, i * 520_000_000))
        .collect();
    let profile = |name: &str, values: Vec<f64>| RadarSeries {
        name: name.into(),
        values,
    };
    let base = profile("Current plan", vec![40.0, 140.0, -10.0]);
    let others = [profile(
        "A much longer comparison name",
        vec![90.0, 20.0, 60.0],
    )];
    let axes = [
        "Customer satisfaction score",
        "Time to first response",
        "Net revenue retention",
    ];
    let (radar, dialog, menu) = (
        format!("states-radar-{theme}"),
        format!("states-modal-{theme}"),
        format!("states-menu-{theme}"),
    );
    let title_id = format!("{dialog}-title");
    Ok(view! {
        <section data-theme=(theme) class="bg-base-100 text-base-content p-6 space-y-6">
            <p class="text-xs uppercase tracking-widest text-base-content/50">
                (theme)
            </p>
            demo(
                title: "time_chart · no points",
                note: "A new project with nothing recorded yet.",
                time_chart(points: &[], front_name: "Visitors", back_name: "Pageviews")
            )
            demo(
                title: "time_chart · one point, all zeros",
                note: "A single bucket, then a week of zeros.",
                <div class="space-y-6">
                    time_chart(
                        points: &single,
                        front_name: "Visitors",
                        back_name: "Pageviews"
                    )
                    time_chart(
                        points: &zeros,
                        front_name: "Visitors",
                        back_name: "Pageviews"
                    )
                </div>
            )
            demo(
                title: "time_chart · billions",
                note: "Axis labels must fit beside the plot.",
                time_chart(
                    points: &huge,
                    front_name: "Visitors",
                    back_name: "Pageviews"
                )
            )
            demo(
                title: "radar_chart · three long axes",
                note: "The minimum axis count, long labels and values outside 0–100.",
                radar_chart(id: &radar, axes: &axes, base: &base, others: &others)
            )
            demo(
                title: "toast · every tone",
                note: "Inline here, outside a toaster, and kept on screen.",
                <div class="space-y-2">
                    for tone in [
                        ToastTone::Neutral,
                        ToastTone::Info,
                        ToastTone::Success,
                        ToastTone::Warning,
                        ToastTone::Error,
                    ] {
                        toast(
                            tone: tone,
                            attrs: attributes! { style="--toast-delay: 1e9s" },
                            (format!("{tone:?}: project saved."))
                        )
                    }
                    toast(
                        tone: ToastTone::Error,
                        (format!("Could not delete {LONG_ID}. {LONG_TEXT}"))
                    )
                </div>
            )
            demo(
                title: "empty_state",
                note: "Short, then long copy in a narrow column.",
                <div class="space-y-4">
                    empty_state(
                        title: "No projects yet",
                        <p>"Create one to start collecting events."</p>
                        <a href="#" class="btn btn-primary btn-sm">"New project"</a>
                    )
                    <div class="max-w-xs">
                        empty_state(
                            title: "No events match the filters you picked for this report",
                            <p>(LONG_TEXT)</p>
                        )
                    </div>
                </div>
            )
            demo(
                title: "modal · popover",
                note: "Long titles and menu items.",
                <div class="flex flex-wrap gap-2">
                    modal_trigger(
                        target: &dialog,
                        attrs: attributes! { class="btn btn-sm" },
                        "Open modal"
                    )
                    popover_trigger(
                        target: &menu,
                        attrs: attributes! { class="btn btn-sm btn-outline" },
                        "Open menu"
                    )
                </div>
                modal(
                    id: &dialog,
                    attrs: attributes! { aria-labelledby=(title_id.as_str()) },
                    <h3 id=(title_id.as_str()) class="font-serif text-xl">
                        "Delete the Lisbon office analytics project and all of its data?"
                    </h3>
                    <p class="text-sm text-base-content/60 mt-2">(LONG_TEXT)</p>
                    modal_action(
                        modal_close(
                            target: &dialog,
                            attrs: attributes! { class="btn btn-ghost" },
                            "Cancel"
                        )
                        modal_close(
                            target: &dialog,
                            attrs: attributes! { class="btn btn-error" },
                            "Delete project"
                        )
                    )
                )
                popover(
                    id: &menu,
                    <ul class="menu menu-sm p-0">
                        <li><a href="#">"Settings"</a></li>
                        <li>
                            <a href="#">
                                "Export all events as CSV for the last 90 days"
                            </a>
                        </li>
                        <li><a href="#" class="text-error">"Delete project"</a></li>
                    </ul>
                )
            )
            demo(
                title: "buttons · badges",
                note: "Default, disabled, loading; long labels in a narrow box.",
                <div class="space-y-3">
                    <div class="flex flex-wrap gap-2">
                        <button class="btn btn-primary btn-sm">"Save"</button>
                        <button class="btn btn-primary btn-sm" disabled=(true)>
                            "Save"
                        </button>
                        <button class="btn btn-primary btn-sm">
                            <span class="loading loading-spinner loading-xs"></span>
                            "Saving"
                        </button>
                        <button class="btn btn-outline btn-sm" disabled=(true)>
                            "Disabled outline"
                        </button>
                    </div>
                    <div
                        class="max-w-48 flex flex-col gap-2 border border-dashed border-base-300 p-2"
                    >
                        <button class="btn btn-sm">
                            "Invite everyone on the team"
                        </button>
                        <span class="badge badge-soft badge-warning">
                            <span class="truncate">"Pending review by an admin"</span>
                        </span>
                        <span class="badge badge-outline font-mono">
                            <span class="truncate">(LONG_ID)</span>
                        </span>
                    </div>
                </div>
            )
            demo(
                title: "table",
                note: "Numbers line up; a long id stays inside the card.",
                <div class="overflow-x-auto">
                    <table class="table table-sm">
                        <thead>
                            <tr>
                                <th>"Project"</th>
                                <th class="text-end">"Events"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>"Berlin"</td>
                                <td class="text-end">"1,111"</td>
                            </tr>
                            <tr>
                                <td>"Toronto"</td>
                                <td class="text-end">"88,888"</td>
                            </tr>
                            <tr>
                                <td class="font-mono break-all">(LONG_ID)</td>
                                <td class="text-end">"0"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            )
        </section>
    })
}
