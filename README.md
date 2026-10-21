# oxidt-ui

Oxidt's look for Topcoat apps: a monochrome theme and daisyUI-compatible
component classes compiled by the standalone Tailwind CLI. No Node, no
`node_modules`, no daisyUI. Plus a Topcoat UI registry of Rust components built
on those classes.

- `theme/oxidt.css`: the entry point. It imports
  - `tokens.css`: colours, radii and fonts under daisyUI's names (`base-100`,
    `primary-content`…), light and `[data-theme="dark"]`
  - `components.css`: `btn`, `card`, `badge`, `input`, `select`, `textarea`,
    `checkbox`, `radio`, `toggle`, `table`, `join`, `alert`, `tabs`,
    `dropdown`, `menu`, `modal`, `drawer`, `loading`, `progress`, `divider`,
    `navbar`, `hero`, `footer`, `collapse`, `toast`, `status`, `link`, `kbd`,
    `breadcrumbs`, `skeleton`, `popover`, `toast-auto`/`toast-close` and
    their modifiers
  - `polish.css`: `card-elevated`, `card-hover`, `nav-link`, `glass-panel`,
    `admin-shell`, `table-wrap`, `table-polished`, `empty-state`,
    `modal-polished`, `auth-bg`, `auth-card`, `btn-strong`, motion
- `src/components/*.rs`: registry components that Topcoat's built-in registry
  lacks: empty_state, modal, popover, radar_chart, time_chart, toast. For buttons, cards, inputs and the
  like, write the classes directly (`<button class="btn btn-primary">`) or keep
  an app's own wrappers.

Overlays need no script in current browsers:

- **modal:** a native `<dialog>`. `modal_trigger` opens it through invoker
  commands (`commandfor`); `modal_close(target)`, Escape or a backdrop click
  closes it. `modal_close` is a plain button, so it is safe inside a form in
  the modal. Browsers without invoker commands (Safari before 26.2, Firefox
  before 144, Chrome before 135) fall back to an inline `onclick` handler,
  which a CSP that forbids inline script blocks. `modal(open: true)` shows it
  on load as a real modal.
- **popover:** the Popover API. `popover_trigger` toggles it, Escape and outside
  clicks close it, and CSS anchor positioning puts it under the trigger.
  Browsers without anchor positioning centre it instead.
- **time_chart, radar_chart:** server-rendered SVG that follows the theme
  tokens. `time_chart` draws two counts over time as bars or a line, with a
  y-axis, a lighter still-filling bucket and CSS-only hover tooltips. The radar switches comparisons
  with radio tabs (up to 6) and shows a tooltip per axis on hover or keyboard
  focus. A radar with 6 axes and 4 profiles is about 14 KB of HTML, 1.6 KB
  gzipped.
- **toast:** `toaster` holds the stack. A `toast` hides itself after 5s
  (`--toast-delay`), pauses on hover or focus, and has a close control built on
  a hidden checkbox. Error toasts stay until closed.

## Use it in an app

```toml
[dependencies]
topcoat = { version = "=0.10.0", features = ["ui", "tailwind"] }
oxidt-ui = { git = "https://github.com/oxidt/oxidt-ui", tag = "v0.1.7" }

[build-dependencies]
topcoat = { version = "=0.10.0", default-features = false, features = ["tailwind"] }
```

oxidt-ui's build script hands its theme path to the app's build script as
`DEP_OXIDT_UI_THEME`. Write a one-line import from it, then render Tailwind:

```rust
// build.rs
fn main() {
    let theme = std::env::var("DEP_OXIDT_UI_THEME").unwrap();
    let import = format!("@import \"{theme}/oxidt.css\";\n");
    // Rewriting an unchanged file would wake file watchers on every build.
    if std::fs::read_to_string("oxidt-ui.css").ok().as_deref() != Some(import.as_str()) {
        std::fs::write("oxidt-ui.css", import).unwrap();
    }
    println!("cargo::rerun-if-changed={theme}");
    println!("cargo::rerun-if-changed=tailwind.css");
    println!("cargo::rerun-if-changed=src");
    topcoat::tailwind::BuildConfig::new().input("tailwind.css").render().unwrap();
}
```

Ignore the generated `oxidt-ui.css` in git. The app's Tailwind input:

```css
@import "tailwindcss";
@plugin "@tailwindcss/typography"; /* bundled with the standalone CLI */
@import "./oxidt-ui.css";
@source "./src";
```

To upgrade, bump the tag and run `cargo update -p oxidt-ui`.

Components:

```sh
topcoat ui add toast popover --registry oxidt-ui
topcoat ui add --all --registry oxidt-ui
```

`topcoat ui init` only installs themes from Topcoat's built-in registry, so
the theme comes from the dependency as shown, not from `ui init`.

## Brand it

The theme is monochrome. Give an app its own accent, shape or fonts by
overriding tokens after the import, in an unlayered rule so it beats both the
light and the dark values. `light-dark()` picks a value per theme:

```css
@import "./oxidt-ui.css";

:root,
[data-theme] {
    --color-primary: light-dark(#c2410c, #fb923c);
    --color-primary-content: light-dark(#ffffff, #1c0a02);
}
```

Everything that reads `primary` follows: `btn-primary`, `badge-primary`,
`bg-primary/10`, `text-primary`, focus rings, checked toggles.

| Token | Default | Drives |
|---|---|---|
| `--color-primary`, `--color-primary-content` | ink, paper | primary buttons, badges, links, focus |
| `--color-secondary`, `--color-accent` (+ `-content`) | greys | the matching modifiers |
| `--color-base-100/200/300`, `--color-base-content` | white to light grey, ink | page, raised surfaces, borders, text |
| `--color-info/success/warning/error` (+ `-content`) | blue, green, amber, red | alerts, status badges |
| `--radius-box` | `0.875rem` | cards, modals, alerts |
| `--radius-field` | `0.5rem` | buttons, inputs, selects |
| `--radius-selector` | `0.375rem` | checkboxes, toggles, badges |
| `--font-sans`, `--font-serif`, `--font-mono` | Geist, Newsreader, system mono | body, `font-serif` headings, code |

The fonts are not bundled: serve Geist and Newsreader yourself or the system
fallbacks show. To change one component rather than a token, add utilities in
the markup (`btn h-12 rounded-full`); they always win over the component's own
rules.

## How the CSS works

Every class is a Tailwind `@utility`. Only the classes found in the sources end
up in the output, and variants work (`md:btn-sm`, `lg:drawer-open`). A
component's own rules sit in a nested `@layer oxidt`, so plain utilities
always win (`btn h-12`). Modifiers only set custom properties
(`btn-primary` sets `--btn-color`), so their order never matters and
combinations like `btn-outline btn-error` or `badge-soft badge-success`
compose.

## Develop

```sh
just check                                  # clippy + compile the components
TAILWIND=/path/to/tailwindcss just showcase # build tests/fixture/out/showcase.html
TAILWIND=/path/to/tailwindcss just browser-test # browser regression checks
just gallery                                # the components with sample data
```

`just gallery` serves `gallery/` at http://127.0.0.1:3100: every chart with
sample data, light and dark side by side. It compiles the components straight
from `src/components`, so it always shows the current code.

`just browser-test` compiles `bench/fixture/checks.html` with the given
Tailwind and checks it in Chromium (`bench/checks.test.ts`): one test per bug
fixed so far, such as checkbox ticks, dark-mode contrast, nested themes,
popover placement and modal behaviour. CI runs it on Tailwind 4.1.18 and
4.3.2, the versions the apps compile with. Add a fixture element and a test
when you fix a theme bug.

Check an app for visual changes before and after a CSS change: list its pages
as `<name> <path>` lines, then from the app directory

```sh
just -f ../oxidt-ui/justfile bench-shoot http://127.0.0.1:8080 bench/before pages.txt
# ...switch the CSS, restart...
just -f ../oxidt-ui/justfile bench-shoot http://127.0.0.1:8080 bench/after pages.txt
just -f ../oxidt-ui/justfile bench-diff bench/before bench/after
```

It shoots every page in light and dark and prints the share of changed pixels
per page, worst first, with the changed rows. An unchanged rerun shows 0.00%.
Signed-in pages need `--storage auth.json` (record it with
`bunx playwright codegen --save-storage=auth.json <url>`) or `--cookie name=value`.

Open `tests/fixture/out/showcase.html` over HTTP (`?theme=dark` for dark).
`topcoat-tailwind` caches the CLI at
`<app>/target/topcoat/cache/tailwind/tailwindcss-<version>-<platform>`.
