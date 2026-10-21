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
oxidt-ui = { path = "../oxidt-ui" }
```

The app's Tailwind input:

```css
@import "tailwindcss";
@plugin "@tailwindcss/typography"; /* bundled with the standalone CLI */
@import "../oxidt-ui/theme/oxidt.css";
@source "./src";
```

Components:

```sh
topcoat ui add toast popover --registry oxidt-ui
topcoat ui add --all --registry oxidt-ui
```

`topcoat ui init` only installs themes from Topcoat's built-in registry, so
the theme is imported by path as shown, not installed.

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
