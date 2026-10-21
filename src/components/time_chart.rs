use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

/// One bucket of a [`time_chart`]: its label and two counts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChartPoint {
    /// Shown in the tooltip and on the x-axis.
    pub label: String,
    /// The main count, drawn in ink.
    pub front: i64,
    /// The second count, drawn faint behind it. Usually the larger one, such as
    /// pageviews behind visitors.
    pub back: i64,
}

/// How a [`time_chart`] draws its buckets.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartStyle {
    /// A bar per bucket, for days and months: a line between them would invent
    /// values in between.
    #[default]
    Bars,
    /// A line through the buckets, for hours.
    Line,
}

const W: f64 = 1000.0;
const H: f64 = 240.0;

/// A chart of two counts over time, as server-rendered SVG with a y-axis.
///
/// `front` is drawn in ink and named `front_name`, `back` faint and named `back_name`.
/// Buckets from `shown` on (a count; all when left out) are left empty, for buckets
/// still in the future. With `partial` the last shown bucket is still filling: it is
/// drawn lighter, so its dip doesn't read as a drop, and its tooltip adds
/// `partial_label` (such as "so far"). Hovering a bucket shows its values, with CSS
/// only; the tooltips stay in the page for screen readers. `attrs` are forwarded to
/// the wrapping `<div>`, with extra classes added to its classes.
///
/// ```ignore
/// view! {
///     time_chart(
///         points: &points,
///         front_name: "Visitors",
///         back_name: "Pageviews",
///         style: ChartStyle::Line,
///         shown: Some(14),
///         partial: true,
///         partial_label: "so far",
///     )
/// }
/// ```
#[component]
pub async fn time_chart(
    points: &[ChartPoint],
    front_name: &str,
    back_name: &str,
    #[default] style: ChartStyle,
    #[default] shown: Option<usize>,
    #[default] partial: bool,
    #[default] partial_label: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let peak = points
        .iter()
        .map(|p| p.back.max(p.front))
        .max()
        .unwrap_or(0);
    let step = nice_step(peak);
    let top = (((peak.max(1) + step - 1) / step) * step) as f64;
    let n = points.len().max(1);
    let slot = W / n as f64;
    let point = move |i: usize, v: i64| ((i as f64 + 0.5) * slot, H - v as f64 / top * H);
    let shown = shown.unwrap_or(points.len()).min(points.len());
    let partial = partial && shown > 0;
    let bars = style == ChartStyle::Bars;
    let line = |value: fn(&ChartPoint) -> i64| {
        points[..shown]
            .iter()
            .enumerate()
            .map(|(i, p)| point(i, value(p)))
            .collect::<Vec<_>>()
    };
    let front_points = line(|p| p.front);
    let front_segments = segments(&front_points);
    let settled = front_segments.len() - usize::from(partial && !front_segments.is_empty());
    let front_path = path(&front_points, &front_segments[..settled]);
    let live = match front_points.len() {
        len if settled < front_segments.len() => {
            path(&front_points[len - 2..], &front_segments[settled..])
        }
        _ => String::new(),
    };
    let back_points = line(|p| p.back);
    let back_path = path(&back_points, &segments(&back_points));
    let area = match (front_points.first(), front_points.last()) {
        (Some((first, _)), Some((last, _))) => format!(
            "{} L{last:.1},{H} L{first:.1},{H} Z",
            path(&front_points, &front_segments),
        ),
        _ => String::new(),
    };
    // The latest value gets a dot: it is where "now" is, and a lone first
    // bucket has no line to show it otherwise, so then `back` gets one too.
    let marker = (!bars && shown > 0 && (partial || shown == 1)).then(|| {
        let latest = &points[shown - 1];
        (
            (shown as f64 - 0.5) / n as f64 * 100.0,
            latest.front as f64 / top * 100.0,
            (shown == 1).then(|| latest.back as f64 / top * 100.0),
        )
    });
    let y_ticks: Vec<i64> = (0..=(top as i64 / step)).map(|k| k * step).collect();
    let every = points.len().div_ceil(6).max(1);
    // Each bar is (x, width, back y, front y, still filling).
    let bar_rects: Vec<(f64, f64, f64, f64, bool)> = if bars {
        points[..shown]
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let width = (slot * 0.7).min(48.0);
                (
                    (i as f64 + 0.5) * slot - width / 2.0,
                    width,
                    point(i, p.back).1,
                    point(i, p.front).1,
                    partial && i + 1 == shown,
                )
            })
            .collect()
    } else {
        Vec::new()
    };
    Ok(view! {
        <div class=(class!(attrs.remove("class"))) (attrs)>
            <div class="flex items-center gap-4 text-xs text-base-content/70 mb-3">
                <span class="inline-flex items-center gap-1.5">
                    <span
                        class=(if bars {
                            "inline-block size-2.5 rounded-sm bg-base-content"
                        } else {
                            "inline-block w-2.5 h-0.5 rounded bg-base-content"
                        })
                    ></span>
                    (front_name)
                </span>
                <span class="inline-flex items-center gap-1.5">
                    <span
                        class=(if bars {
                            "inline-block size-2.5 rounded-sm bg-base-content/15"
                        } else {
                            "inline-block w-2.5 h-0.5 rounded bg-base-content/35"
                        })
                    ></span>
                    (back_name)
                </span>
            </div>
            <div class="flex gap-3">
                <div
                    class="relative w-8 shrink-0 h-48 sm:h-56 text-xs text-base-content/50 tabular-nums"
                >
                    for tick in &y_ticks {
                        <span
                            class="absolute right-0 translate-y-1/2"
                            style=(format!("bottom: {:.2}%", *tick as f64 / top * 100.0))
                        >
                            (fmt_count(*tick))
                        </span>
                    }
                </div>
                <div class="relative flex-1 min-w-0">
                    <div class="relative h-48 sm:h-56">
                        <svg
                            viewBox=(format!("0 0 {W} {H}"))
                            preserveAspectRatio="none"
                            class="absolute inset-0 w-full h-full overflow-visible"
                            role="img"
                            aria-label=(format!("{front_name} / {back_name}"))
                        >
                            for tick in &y_ticks {
                                let y = H - *tick as f64 / top * H;
                                <line
                                    x1="0"
                                    x2=(W)
                                    y1=(y)
                                    y2=(y)
                                    class="stroke-base-300"
                                    stroke-width="1"
                                    vector-effect="non-scaling-stroke"
                                ></line>
                            }
                            for (x, width, back_y, front_y, filling) in &bar_rects {
                                <rect
                                    x=(format!("{x:.1}"))
                                    y=(format!("{back_y:.1}"))
                                    width=(format!("{width:.1}"))
                                    height=(format!("{:.1}", H - back_y))
                                    class="fill-base-content/15"
                                ></rect>
                                <rect
                                    x=(format!("{x:.1}"))
                                    y=(format!("{front_y:.1}"))
                                    width=(format!("{width:.1}"))
                                    height=(format!("{:.1}", H - front_y))
                                    class=(if *filling {
                                        "fill-base-content/45"
                                    } else {
                                        "fill-base-content"
                                    })
                                ></rect>
                            }
                            if !bars {
                                <path d=(area) class="fill-base-content/[0.06]"></path>
                                <path
                                    d=(back_path)
                                    fill="none"
                                    class="stroke-base-content/35"
                                    stroke-width="1.25"
                                    stroke-dasharray="4 3"
                                    vector-effect="non-scaling-stroke"
                                ></path>
                                <path
                                    d=(front_path)
                                    fill="none"
                                    class="stroke-base-content"
                                    stroke-width="1.75"
                                    stroke-linejoin="round"
                                    vector-effect="non-scaling-stroke"
                                ></path>
                                <path
                                    d=(live)
                                    fill="none"
                                    class="stroke-base-content/70"
                                    stroke-width="1.75"
                                    stroke-dasharray="3 3"
                                    vector-effect="non-scaling-stroke"
                                ></path>
                            }
                        </svg>
                        if let Some((left, bottom, back_bottom)) = marker {
                            if let Some(back_bottom) = back_bottom {
                                <span
                                    class="absolute size-2 -translate-x-1/2 translate-y-1/2 rounded-full bg-base-content/35"
                                    style=(format!(
                                        "left: {left:.2}%; bottom: {back_bottom:.2}%",
                                    ))
                                ></span>
                            }
                            <span
                                class="absolute size-2 -translate-x-1/2 translate-y-1/2 rounded-full bg-base-content"
                                style=(format!("left: {left:.2}%; bottom: {bottom:.2}%"))
                            ></span>
                        }
                        <div class="absolute inset-0 flex">
                            for (i, p) in points.iter().enumerate() {
                                let pct = p.front as f64 / top * 100.0;
                                let align = if i < n / 4 {
                                    "left-0"
                                } else if i >= n - n / 4 {
                                    "right-0"
                                } else {
                                    "left-1/2 -translate-x-1/2"
                                };
                                <div class="group relative flex-1">
                                    if i < shown {
                                        if bars {
                                            <span
                                                class="absolute inset-0 rounded-sm bg-base-content/5 hidden group-hover:block"
                                            ></span>
                                        } else {
                                            <span
                                                class="absolute inset-y-0 left-1/2 w-px bg-base-content/30 hidden group-hover:block"
                                            ></span>
                                            <span
                                                class="absolute left-1/2 size-2.5 -translate-x-1/2 translate-y-1/2 rounded-full border-2 border-base-content bg-base-200 hidden group-hover:block"
                                                style=(format!("bottom: {pct:.2}%"))
                                            ></span>
                                        }
                                        // Transparent rather than `hidden`, so screen
                                        // readers still read every bucket's values.
                                        <div
                                            class=(format!(
                                                "absolute {align} z-10 bottom-3 opacity-0 group-hover:opacity-100 pointer-events-none whitespace-nowrap rounded-field border border-base-300 bg-base-100 px-3 py-2 text-xs shadow-lg",
                                            ))
                                        >
                                            <div class="text-base-content/60">
                                                (&p.label)
                                                if partial && i + 1 == shown && !partial_label.is_empty() {
                                                    " · "
                                                    (partial_label)
                                                }
                                            </div>
                                            <div class="font-medium tabular-nums text-sm">
                                                (fmt_int(p.front))
                                                " "
                                                <span class="font-normal text-base-content/60">
                                                    (front_name)
                                                </span>
                                            </div>
                                            <div class="tabular-nums text-base-content/60">
                                                (fmt_int(p.back))
                                                " "
                                                (back_name)
                                            </div>
                                        </div>
                                    }
                                </div>
                            }
                        </div>
                    </div>
                    <div
                        class="relative h-5 mt-2 text-xs text-base-content/50 tabular-nums"
                    >
                        for (i, p) in points
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| i % every == 0) {
                            <span
                                class="absolute -translate-x-1/2 whitespace-nowrap"
                                style=(format!(
                                    "left: {:.2}%",
                                    (i as f64 + 0.5) / n as f64 * 100.0,
                                ))
                            >
                                (&p.label)
                            </span>
                        }
                    </div>
                </div>
            </div>
        </div>
    })
}

/// A 1/2/2.5/5 × 10^k step that splits `peak` into about three bands.
fn nice_step(peak: i64) -> i64 {
    let raw = (peak.max(3) as f64 / 3.0).max(1.0);
    let magnitude = 10f64.powf(raw.log10().floor());
    let step = [1.0, 2.0, 2.5, 5.0, 10.0]
        .into_iter()
        .map(|m| m * magnitude)
        .find(|s| *s >= raw)
        .unwrap_or(10.0 * magnitude);
    (step.round() as i64).max(1)
}

/// One line command per gap between `points`, split so the last gap can be
/// drawn on its own.
fn segments(points: &[(f64, f64)]) -> Vec<String> {
    points
        .iter()
        .skip(1)
        .map(|(x, y)| format!(" L{x:.1},{y:.1}"))
        .collect()
}

/// `segments` drawn from the first of `points`.
fn path(points: &[(f64, f64)], segments: &[String]) -> String {
    match points.first() {
        Some((x, y)) => format!("M{x:.1},{y:.1}{}", segments.concat()),
        None => String::new(),
    }
}

/// A compact count for the y-axis: 950, 1.2k, 36k, 1.5M.
fn fmt_count(n: i64) -> String {
    match n {
        n if n >= 1_000_000 => format!("{:.1}M", n as f64 / 1_000_000.0),
        n if n >= 10_000 => format!("{:.0}k", n as f64 / 1_000.0),
        n if n >= 1_000 => format!("{:.1}k", n as f64 / 1_000.0),
        n => n.to_string(),
    }
}

/// A full count with thousands separators, for the tooltip: 36,686.
fn fmt_int(n: i64) -> String {
    let digits = n.unsigned_abs().to_string();
    let mut out = String::new();
    if n < 0 {
        out.push('-');
    }
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths_split_per_gap() {
        let points = [(0.0, 240.0), (500.0, 0.0), (1000.0, 120.0)];
        let lines = segments(&points);
        assert_eq!(lines, [" L500.0,0.0", " L1000.0,120.0"]);
        assert_eq!(path(&points[1..], &lines[1..]), "M500.0,0.0 L1000.0,120.0");
        assert_eq!(segments(&[]), Vec::<String>::new());
        assert_eq!(path(&[], &[]), "");
    }

    #[test]
    fn scale_and_separators() {
        assert_eq!(nice_step(1_460), 500);
        assert_eq!(nice_step(0), 1);
        assert_eq!(nice_step(70), 25);
        assert_eq!(fmt_int(1_052), "1,052");
        assert_eq!(fmt_int(36_686), "36,686");
        assert_eq!(fmt_int(999), "999");
        assert_eq!(fmt_count(1_460), "1.5k");
        assert_eq!(fmt_count(36_686), "37k");
    }
}
