use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// A placeholder for a list or page with nothing in it yet.
///
/// `title` is the headline. Pass the explanation and an optional action, such as a
/// button, as children. `attrs` are forwarded to the `<div>`, with extra classes added
/// to its classes.
///
/// ```ignore
/// view! {
///     empty_state(
///         title: "No projects yet",
///         <p>"Create one to start collecting events."</p>
///         <a href="/projects/new" class="btn btn-primary btn-sm">"New project"</a>
///     )
/// }
/// ```
#[component]
pub async fn empty_state(
    title: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(
                "empty-state flex flex-col items-center gap-3 px-6 py-12 text-center",
                attrs.remove("class"),
            ))
            (attrs)
        >
            <h3 class="font-serif text-xl">(title)</h3>
            <div
                class="flex max-w-sm flex-col items-center gap-4 text-sm text-base-content/60"
            >
                (child)
            </div>
        </div>
    })
}
