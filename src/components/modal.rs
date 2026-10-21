use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// A modal dialog built on the native `<dialog>` element.
///
/// Give it an `id` and open it with a [`modal_trigger`] for that id. Set `open` to show it
/// as soon as the page loads, for example to keep a form with errors on screen; this
/// opens it as a real modal (focus trapped, background inert, Escape closes it) through a
/// one-line inline script. Do not pass a raw `open` attribute: the browser would show a
/// non-modal dialog that looks modal. Escape and a click outside the panel close it. Pass
/// the content as children and end it with a [`modal_action`] row. `attrs` are forwarded
/// to the `<dialog>`, with extra classes added to its classes. Name it with
/// `aria-labelledby` pointing at its heading.
///
/// ```ignore
/// view! {
///     modal(
///         id: "delete-project",
///         <h3 id="delete-project-title" class="font-serif text-xl">"Delete project?"</h3>
///         <p class="text-sm text-base-content/60">"This cannot be undone."</p>
///         modal_action(
///             modal_close(target: "delete-project", attrs: attributes! { class="btn btn-ghost" }, "Cancel")
///             <button type="submit" form="delete" class="btn btn-error">"Delete"</button>
///         )
///     )
/// }
/// ```
#[component]
pub async fn modal(
    id: &str,
    #[default] open: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <dialog id=(id) class=(class!("modal", attrs.remove("class"))) (attrs)>
            <div class="modal-box modal-polished">(child)</div>
            <form method="dialog" class="modal-backdrop">
                <button aria-label="Close">"close"</button>
            </form>
        </dialog>
        if open {
            <script>"document.currentScript.previousElementSibling.showModal()"</script>
        }
    })
}

/// Runs the button's command by hand where the browser lacks invoker commands
/// (`commandfor`), for example Safari before 26.2. Supporting browsers skip it.
const OPEN_FALLBACK: &str = "if(!('commandForElement' in HTMLButtonElement.prototype))document.getElementById(this.getAttribute('commandfor')).showModal()";
const CLOSE_FALLBACK: &str = "if(!('commandForElement' in HTMLButtonElement.prototype))document.getElementById(this.getAttribute('commandfor')).close()";

/// A button that opens the [`modal`] with the id `target`, without script.
///
/// Uses the browser's invoker commands (`commandfor`); browsers without them fall back
/// to an inline `onclick` handler, which a Content Security Policy that forbids inline
/// script blocks. Pass the label as children. `attrs` are forwarded to the `<button>`,
/// with extra classes added to its classes.
///
/// ```ignore
/// view! {
///     modal_trigger(target: "delete-project", attrs: attributes! { class="btn btn-error" }, "Delete")
/// }
/// ```
#[component]
pub async fn modal_trigger(
    target: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            commandfor=(target)
            command="show-modal"
            onclick=(OPEN_FALLBACK)
            class=(class!(attrs.remove("class")))
            (attrs)
        >
            (child)
        </button>
    })
}

/// A button that closes the [`modal`] with the id `target`.
///
/// A plain `type="button"` with an invoker command, so it is safe inside a form in the
/// modal: it never submits or validates that form. Browsers without invoker commands use
/// the same inline fallback as [`modal_trigger`]. Pass the label as children. `attrs` are
/// forwarded to the `<button>`, with extra classes added to its classes.
#[component]
pub async fn modal_close(
    target: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            commandfor=(target)
            command="close"
            onclick=(CLOSE_FALLBACK)
            class=(class!(attrs.remove("class")))
            (attrs)
        >
            (child)
        </button>
    })
}

/// The row of actions at the bottom of a [`modal`], aligned to the end.
#[component]
pub async fn modal_action(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("modal-action", attrs.remove("class"))) (attrs)>(child)</div>
    })
}
