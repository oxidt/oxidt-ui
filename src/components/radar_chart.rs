use std::f64::consts::{FRAC_PI_2, TAU};

use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

/// One profile on a [`radar_chart`]: a name and one value per axis.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RadarSeries {
    pub name: String,
    /// One value per axis, in axis order, between 0 and the chart's `max`.
    pub values: Vec<f64>,
}

/// Radius of the outer ring, in SVG units.
const R: f64 = 100.0;
/// Half the viewBox, which leaves room for the axis labels.
const HALF_W: f64 = 190.0;
const HALF_H: f64 = 135.0;
/// Most comparisons the tabs can switch between (the CSS has a rule per slot).
pub const MAX_COMPARISONS: usize = 6;

/// A radar chart comparing `base` with one of `others`, as server-rendered SVG.
///
/// With several `others`, radio tabs above the chart switch the comparison; the shapes
/// cross-fade, with no script. Hovering or focusing an axis (Tab moves between them)
/// shows a tooltip with both values and the difference. `max` is the value at the
/// outer ring (100 when left at 0). `id` must be unique on the page; it names the tabs.
/// Colours come from `--radar-a` (base, info blue) and `--radar-b` (comparison, the
/// accent); set them on the element to restyle. `attrs` are forwarded to the wrapping
/// `<div>`, with extra classes added to its classes.
///
/// ```ignore
/// view! {
///     radar_chart(
///         id: "offices",
///         axes: &["Talent pool", "Affordability", "Transit"],
///         base: &lisbon,
///         others: &[berlin, toronto],
///     )
/// }
/// ```
#[component]
pub async fn radar_chart(
    id: &str,
    axes: &[&str],
    base: &RadarSeries,
    others: &[RadarSeries],
    #[default] max: f64,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let max = if max > 0.0 { max } else { 100.0 };
    let others = &others[..others.len().min(MAX_COMPARISONS)];
    let n = axes.len().max(3);
    let angle = move |i: usize| -FRAC_PI_2 + TAU * i as f64 / n as f64;
    let point = move |i: usize, r: f64| (r * angle(i).cos(), r * angle(i).sin());
    let scaled = move |v: f64| v.clamp(0.0, max) / max * R;
    let shape = move |values: &[f64]| {
        (0..n)
            .map(|i| {
                let (x, y) = point(i, scaled(values.get(i).copied().unwrap_or(0.0)));
                format!("{x:.1},{y:.1}")
            })
            .collect::<Vec<_>>()
            .join(" ")
    };
    let ring = move |f: f64| shape(&vec![max * f; n]);
    let value_of = |s: &RadarSeries, i: usize| s.values.get(i).copied().unwrap_or(0.0);
    Ok(view! {
        <div class=(class!("radar", attrs.remove("class"))) (attrs)>
            if others.len() > 1 {
                <div class="radar-tabs tabs tabs-box tabs-sm">
                    for (k, other) in others.iter().enumerate() {
                        <input
                            type="radio"
                            name=(format!("{id}-compare"))
                            class=(format!("tab radar-pick-{}", k + 1))
                            aria-label=(&other.name)
                            checked=(k == 0)
                        >
                    }
                </div>
            }
            <svg
                viewBox=(format!("{} {} {} {}", -HALF_W, -HALF_H, 2.0 * HALF_W, 2.0 * HALF_H))
                class="radar-svg"
                role="group"
                aria-label=(std::iter::once(base.name.as_str()).chain(others.iter().map(|o| o.name.as_str())).collect::<Vec<_>>().join(" / "))
            >
                <g aria-hidden="true">
                    for f in [0.25, 0.5, 0.75, 1.0] {
                        <polygon points=(ring(f)) class="radar-grid"></polygon>
                    }
                    for i in 0..n {
                        let (x, y) = point(i, R);
                        <line x1="0" y1="0" x2=(format!("{x:.1}")) y2=(format!("{y:.1}")) class="radar-grid"></line>
                    }
                    for (i, axis) in axes.iter().enumerate() {
                        let (x, y) = point(i, R * 1.16);
                        <text
                            x=(format!("{x:.1}"))
                            y=(format!("{:.1}", y + 3.5))
                            text-anchor=(if x.abs() < 1.0 { "middle" } else if x > 0.0 { "start" } else { "end" })
                            class="radar-label"
                        >
                            (*axis)
                        </text>
                    }
                    <polygon points=(shape(&base.values)) class="radar-shape radar-a"></polygon>
                    for (k, other) in others.iter().enumerate() {
                        <polygon
                            points=(shape(&other.values))
                            class=(format!("radar-shape radar-b radar-only-{}", k + 1))
                        ></polygon>
                    }
                </g>
                for (i, axis) in axes.iter().enumerate() {
                    let a = value_of(base, i);
                    let (ax, ay) = point(i, scaled(a));
                    let (wx1, wy1) = (R * 1.3 * (angle(i) - TAU / n as f64 / 2.0).cos(), R * 1.3 * (angle(i) - TAU / n as f64 / 2.0).sin());
                    let (wx2, wy2) = (R * 1.3 * (angle(i) + TAU / n as f64 / 2.0).cos(), R * 1.3 * (angle(i) + TAU / n as f64 / 2.0).sin());
                    let (sx, sy) = point(i, R);
                    let rows = 1 + others.len().min(1);
                    let (tw, th) = (118.0, 18.0 + 14.0 * rows as f64);
                    let tx = (if ax >= 0.0 { ax + 10.0 } else { ax - 10.0 - tw }).clamp(-HALF_W, HALF_W - tw);
                    let ty = (ay - th / 2.0).clamp(-HALF_H, HALF_H - th);
                    <g
                        class="radar-axis"
                        tabindex="0"
                        aria-label=(format!(
                            "{axis}: {} {}{}",
                            base.name,
                            fmt(a),
                            others.iter().map(|o| format!(", {} {}", o.name, fmt(value_of(o, i)))).collect::<String>(),
                        ))
                    >
                        <polygon points=(format!("0,0 {wx1:.1},{wy1:.1} {wx2:.1},{wy2:.1}")) class="radar-hit"></polygon>
                        <g class="radar-focus" aria-hidden="true">
                            <line x1="0" y1="0" x2=(format!("{sx:.1}")) y2=(format!("{sy:.1}")) class="radar-spoke"></line>
                            <circle cx=(format!("{ax:.1}")) cy=(format!("{ay:.1}")) r="3.5" class="radar-dot radar-a"></circle>
                            for (k, other) in others.iter().enumerate() {
                                let (bx, by) = point(i, scaled(value_of(other, i)));
                                <circle
                                    cx=(format!("{bx:.1}"))
                                    cy=(format!("{by:.1}"))
                                    r="3.5"
                                    class=(format!("radar-dot radar-b radar-only-{}", k + 1))
                                ></circle>
                            }
                            <g class="radar-tip" transform=(format!("translate({tx:.1} {ty:.1})"))>
                                <rect width=(tw) height=(th) rx="6" class="radar-tip-box"></rect>
                                <text x="9" y="13" class="radar-tip-title">(*axis)</text>
                                <line x1="9" x2="19" y1="24" y2="24" class="radar-key radar-a"></line>
                                <text x="24" y="27" class="radar-tip-text">(&base.name)</text>
                                <text x="86" y="27" text-anchor="end" class="radar-tip-value">(fmt(a))</text>
                                for (k, other) in others.iter().enumerate() {
                                    let b = value_of(other, i);
                                    <g class=(format!("radar-only-{}", k + 1))>
                                        <line x1="9" x2="19" y1="38" y2="38" class="radar-key radar-b"></line>
                                        <text x="24" y="41" class="radar-tip-text">(&other.name)</text>
                                        <text x="86" y="41" text-anchor="end" class="radar-tip-value">(fmt(b))</text>
                                        <text x="110" y="41" text-anchor="end" class="radar-tip-delta">(delta(b - a))</text>
                                    </g>
                                }
                            </g>
                        </g>
                    </g>
                }
            </svg>
            <div class="radar-legend" aria-hidden="true">
                <span><i class="radar-swatch radar-a"></i>(&base.name)</span>
                for (k, other) in others.iter().enumerate() {
                    <span class=(format!("radar-only-{}", k + 1))>
                        <i class="radar-swatch radar-b"></i>
                        (&other.name)
                    </span>
                }
            </div>
        </div>
    })
}

/// Whole numbers without decimals, others with one.
fn fmt(v: f64) -> String {
    if v.fract() == 0.0 { format!("{v:.0}") } else { format!("{v:.1}") }
}

/// A signed difference with a real minus sign; nothing for no difference.
fn delta(d: f64) -> String {
    match d {
        d if d > 0.0 => format!("+{}", fmt(d)),
        d if d < 0.0 => format!("\u{2212}{}", fmt(-d)),
        _ => String::new(),
    }
}
