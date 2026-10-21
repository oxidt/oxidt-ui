use topcoat::{
    Result,
    view::{AttributeValue, Attributes, Child, View, class, component, view},
};

/// `own` followed by the caller's `style` attribute, if any, so neither is lost.
fn merge_style(own: String, theirs: Option<AttributeValue>) -> String {
    match theirs {
        Some(AttributeValue::StaticStr { value, .. }) => format!("{own}; {value}"),
        Some(AttributeValue::PromotedStr { value, .. }) => format!("{own}; {}", *value),
        Some(AttributeValue::String { value, .. }) => format!("{own}; {value}"),
        _ => own,
    }
}

/// A button that opens and closes the [`popover`] with the id `target`.
///
/// It anchors the popover, so the panel opens under it. Pass the label as children.
/// `attrs` are forwarded to the `<button>`, with extra classes added to its classes and
/// an extra `style` appended to its own; style it with `btn` classes.
///
/// ```ignore
/// view! {
///     popover_trigger(target: "project-menu", attrs: attributes! { class="btn btn-sm btn-outline" }, "Options")
/// }
/// ```
#[component]
pub async fn popover_trigger(
    target: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            popovertarget=(target)
            style=(merge_style(format!("anchor-name: --{target}"), attrs.remove("style")))
            class=(class!(attrs.remove("class")))
            (attrs)
        >
            (child)
        </button>
    })
}

/// A floating panel opened by a [`popover_trigger`] with the same `id`.
///
/// Built on the native Popover API: Escape and a click outside close it, with no
/// script. It opens under its trigger and flips when it would leave the screen;
/// browsers without CSS anchor positioning centre it instead. Put a
/// `<ul class="menu menu-sm">` inside for an action menu. `attrs` are forwarded to
/// the `<div>`, with extra classes added to its classes and an extra `style` appended
/// to its own; add `popover-end` to align it with the trigger's end edge and
/// `popover-top` to open it upwards (they combine).
///
/// ```ignore
/// view! {
///     popover(
///         id: "project-menu",
///         <ul class="menu menu-sm p-0">
///             <li><a href="/projects/acme/settings">"Settings"</a></li>
///             <li><button type="submit" form="delete">"Delete"</button></li>
///         </ul>
///     )
/// }
/// ```
#[component]
pub async fn popover(
    id: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            id=(id)
            popover="auto"
            style=(merge_style(format!("position-anchor: --{id}"), attrs.remove("style")))
            class=(class!("popover", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}
