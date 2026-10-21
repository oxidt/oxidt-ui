use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// One bar of a [`bar_chart`]: a label and two counts drawn over each other.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BarPoint {
    /// Shown in the tooltip and, for the first, middle and last bar, under the chart.
    pub label: String,
    /// Drawn in front, in ink.
    pub front: i64,
    /// Drawn behind, faint. Usually the larger count, such as pageviews behind visitors.
    pub back: i64,
}

const W: f64 = 1000.0;
const H: f64 = 220.0;

/// A bar chart of two counts per bucket, as server-rendered SVG.
///
/// Each bar shows `front` over `back`, named `front_name` and `back_name` in the legend
/// and the tooltip (a native `<title>`, so no script). The SVG stretches to the width of
/// its container. Children go at the end of the legend row, for example the peak value.
/// `attrs` are forwarded to the wrapping `<div>`, with extra classes added to its classes.
///
/// ```ignore
/// view! {
///     bar_chart(
///         points: &points,
///         front_name: "Visitors",
///         back_name: "Pageviews",
///         <span>"Peak: 1.2k"</span>
///     )
/// }
/// ```
#[component]
pub async fn bar_chart(
    points: &[BarPoint],
    front_name: &str,
    back_name: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let max = points.iter().map(|p| p.back.max(p.front)).max().unwrap_or(0).max(1) as f64;
    let slot = W / points.len().max(1) as f64;
    let width = (slot * 0.7).max(1.0);
    // A non-zero count stays visible as a 2-unit sliver.
    let height = move |v: i64| (v as f64 / max * (H - 4.0)).max(if v > 0 { 2.0 } else { 0.0 });
    let ticks: Vec<&str> = match points {
        [] => Vec::new(),
        [only] => vec![&only.label],
        [first, .., last] => vec![&first.label, &points[points.len() / 2].label, &last.label],
    };
    let (front_lower, back_lower) = (front_name.to_lowercase(), back_name.to_lowercase());
    Ok(view! {
        <div class=(class!(attrs.remove("class"))) (attrs)>
            <div class="flex items-center gap-4 text-xs text-base-content/60 mb-5">
                <span class="inline-flex items-center gap-1">
                    <span class="inline-block w-3 h-3 rounded-sm bg-base-content/80"></span>
                    (front_name)
                </span>
                <span class="inline-flex items-center gap-1">
                    <span class="inline-block w-3 h-3 rounded-sm bg-base-content/15"></span>
                    (back_name)
                </span>
                <span class="ml-auto">(child)</span>
            </div>
            <svg
                viewBox=(format!("0 0 {W} {H}"))
                preserveAspectRatio="none"
                class="block w-full h-48 sm:h-56"
                role="img"
                aria-label=(format!("{front_name} / {back_name}"))
            >
                <line
                    x1="0"
                    x2=(W)
                    y1=(H - 0.5)
                    y2=(H - 0.5)
                    class="stroke-base-300"
                    stroke-width="1"
                ></line>
                for (i, point) in points.iter().enumerate() {
                    let x = i as f64 * slot + (slot - width) / 2.0;
                    <g>
                        <title>
                            (format!(
                                "{}: {} {front_lower}, {} {back_lower}",
                                point.label, point.front, point.back,
                            ))
                        </title>
                        // The whole column is the hover target, not just the bar.
                        <rect
                            x=(i as f64 * slot)
                            y="0"
                            width=(slot)
                            height=(H)
                            fill="transparent"
                        ></rect>
                        <rect
                            x=(x)
                            y=(H - height(point.back))
                            width=(width)
                            height=(height(point.back))
                            rx="2"
                            class="fill-base-content/15"
                        ></rect>
                        <rect
                            x=(x)
                            y=(H - height(point.front))
                            width=(width)
                            height=(height(point.front))
                            rx="2"
                            class="fill-base-content/80"
                        ></rect>
                    </g>
                }
            </svg>
            <div class="flex justify-between text-xs text-base-content/60 mt-4 tabular-nums">
                for tick in ticks {
                    <span>(tick)</span>
                }
            </div>
        </div>
    })
}
