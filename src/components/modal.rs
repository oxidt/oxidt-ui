use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// A modal dialog built on the native `<dialog>` element.
///
/// Give it an `id` and open it with a [`modal_trigger`] for that id, or render it with
/// `open` for a fixed state. Escape and a click outside the panel close
/// it. Pass the content as children and end it with a [`modal_action`] row. `attrs` are
/// forwarded to the `<dialog>`, with extra classes added to its classes. Name it with
/// `aria-labelledby` pointing at its heading.
///
/// ```ignore
/// view! {
///     modal(
///         id: "delete-project",
///         <h3 id="delete-project-title" class="font-serif text-xl">"Delete project?"</h3>
///         <p class="text-sm text-base-content/60">"This cannot be undone."</p>
///         modal_action(
///             modal_close(attrs: attributes! { class="btn btn-ghost" }, "Cancel")
///             <button type="submit" form="delete" class="btn btn-error">"Delete"</button>
///         )
///     )
/// }
/// ```
#[component]
pub async fn modal(
    id: &str,
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
    })
}

/// A button that opens the [`modal`] with the id `target`, without script.
///
/// Uses the browser's invoker commands (`commandfor`). Pass the label as children.
/// `attrs` are forwarded to the `<button>`, with extra classes added to its classes.
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
            class=(class!(attrs.remove("class")))
            (attrs)
        >
            (child)
        </button>
    })
}

/// A button that closes the [`modal`] it sits in.
///
/// Pass the label as children. `attrs` are forwarded to the `<button>`, with extra
/// classes added to its classes.
#[component]
pub async fn modal_close(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <form method="dialog" class="contents">
            <button class=(class!(attrs.remove("class"))) (attrs)>(child)</button>
        </form>
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
