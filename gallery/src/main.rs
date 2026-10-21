//! A local page that renders oxidt-ui's components with sample data, in a
//! light and a dark column side by side. Run it with `just gallery`.
//!
//! The components are compiled straight from `../src/components`, so the
//! page always shows the registry's current code.

use topcoat::{
    Result,
    asset::{Asset, AssetBundle, RouterBuilderAssetExt},
    router::{module_router, page},
    view::{View, component, view},
};

#[allow(dead_code)]
#[path = "../../src/components/radar_chart.rs"]
mod radar;
#[allow(dead_code)]
#[path = "../../src/components/time_chart.rs"]
mod timeline;

use radar::{RadarSeries, radar_chart};
use timeline::{ChartPoint, ChartStyle, time_chart};

const CSS: Asset = topcoat::tailwind::stylesheet!();

#[tokio::main]
async fn main() {
    topcoat::start(
        module_router!()
            .assets(AssetBundle::load().unwrap())
            .build(),
    )
    .await
    .unwrap();
}

#[page]
async fn home() -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="en" data-theme="light">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"oxidt-ui gallery"</title>
                <link rel="stylesheet" href=(CSS)>
                topcoat::dev::script()
            </head>
            <body class="bg-base-200">
                <header class="px-6 py-8 max-w-7xl mx-auto">
                    <h1 class="font-serif text-4xl">"oxidt-ui gallery"</h1>
                    <p class="text-base-content/60 mt-2 max-w-2xl">
                        "The registry's charts with sample data, light and dark side by side. "
                        "Hover the charts, or Tab through the radar's axes."
                    </p>
                </header>
                <main class="grid lg:grid-cols-2 gap-px bg-base-300 border-y border-base-300">
                    themed(theme: "light")
                    themed(theme: "dark")
                </main>
            </body>
        </html>
    })
}

/// Every demo, in one theme.
#[component]
async fn themed(theme: &str) -> Result<impl View> {
    let daily = series(30, 420, 7);
    let hourly = series(24, 60, 3);
    let monthly = series(12, 9_000, 11);
    let profile = |name: &str, values: [f64; 6]| RadarSeries {
        name: name.into(),
        values: values.to_vec(),
    };
    let lisbon = profile("Lisbon", [70.0, 80.0, 62.0, 78.0, 86.0, 72.0]);
    let cities = vec![
        profile("Berlin", [88.0, 55.0, 85.0, 72.0, 78.0, 45.0]),
        profile("Toronto", [75.0, 50.0, 70.0, 40.0, 82.0, 60.0]),
        profile("Austin", [65.0, 68.0, 35.0, 55.0, 70.0, 90.0]),
    ];
    let axes = [
        "Talent pool",
        "Affordability",
        "Transit",
        "Time zone fit",
        "Quality of life",
        "Tax climate",
    ];
    let (offices, pair) = (format!("offices-{theme}"), format!("pair-{theme}"));
    Ok(view! {
        <section data-theme=(theme) class="bg-base-100 text-base-content p-6 space-y-6">
            <p class="text-xs uppercase tracking-widest text-base-content/50">(theme)</p>
            demo(
                title: "time_chart · bars",
                note: "Last 30 days. The last day is still filling, so it is lighter.",
                time_chart(
                    points: &daily,
                    front_name: "Visitors",
                    back_name: "Pageviews",
                    shown: Some(30),
                    partial: true,
                    partial_label: "so far",
                )
            )
            demo(
                title: "time_chart · line",
                note: "Today by hour. Hours after now stay empty; the live hour is dashed.",
                time_chart(
                    points: &hourly,
                    front_name: "Visitors",
                    back_name: "Pageviews",
                    style: ChartStyle::Line,
                    shown: Some(15),
                    partial: true,
                    partial_label: "so far",
                )
            )
            demo(
                title: "time_chart · months",
                note: "Last 12 months, all complete.",
                time_chart(points: &monthly, front_name: "Visitors", back_name: "Pageviews")
            )
            demo(
                title: "radar_chart",
                note: "Lisbon compared with three cities. Switch them with the tabs.",
                radar_chart(
                    id: &offices,
                    axes: &axes,
                    base: &lisbon,
                    others: &cities,
                )
            )
            demo(
                title: "radar_chart · one comparison",
                note: "With a single comparison there are no tabs.",
                radar_chart(
                    id: &pair,
                    axes: &axes[..5],
                    base: &lisbon,
                    others: &cities[..1],
                )
            )
        </section>
    })
}

/// A titled card around one example.
#[component]
async fn demo(title: &str, note: &str, child: topcoat::view::Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div class="card card-elevated bg-base-100">
            <div class="card-body p-5">
                <h2 class="font-mono text-sm">(title)</h2>
                <p class="text-sm text-base-content/60 -mt-1 mb-2">(note)</p>
                (child)
            </div>
        </div>
    })
}

/// `n` buckets of made-up traffic around `scale`, wavy but repeatable.
fn series(n: usize, scale: i64, seed: u64) -> Vec<ChartPoint> {
    (0..n)
        .map(|i| {
            let wave = ((i as f64 + seed as f64) * 0.7).sin() * 0.35 + 0.65;
            let noise = ((i as u64 * 2_654_435_761 + seed) % 97) as f64 / 97.0 * 0.3;
            let back = ((wave + noise) * scale as f64) as i64;
            ChartPoint {
                label: format!("#{}", i + 1),
                front: back * 3 / 5,
                back,
            }
        })
        .collect()
}
