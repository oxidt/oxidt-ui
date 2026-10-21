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
  lacks: empty_state, modal, popover, toast. For buttons, cards, inputs and the
  like, write the classes directly (`<button class="btn btn-primary">`) or keep
  an app's own wrappers.

Overlays need no script:

- **modal:** a native `<dialog>`. `modal_trigger` opens it through invoker
  commands (`commandfor`), `modal_close` or Escape or a backdrop click closes it.
- **popover:** the Popover API. `popover_trigger` toggles it, Escape and outside
  clicks close it, and CSS anchor positioning puts it under the trigger.
  Browsers without anchor positioning centre it instead.
- **toast:** `toaster` holds the stack. A `toast` hides itself after 5s
  (`--toast-delay`), pauses on hover or focus, and has a close control built on
  a hidden checkbox. Error toasts stay until closed.

## Use it in an app

```toml
[dependencies]
topcoat = { version = "=0.10.0", features = ["ui", "tailwind"] }
oxidt-ui = { git = "https://github.com/oxidt/oxidt-ui", tag = "v0.1.1" }

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
```

Open `tests/fixture/out/showcase.html` over HTTP (`?theme=dark` for dark).
`topcoat-tailwind` caches the CLI at
`<app>/target/topcoat/cache/tailwind/tailwindcss-<version>-<platform>`.
