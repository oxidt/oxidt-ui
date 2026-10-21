use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

/// The status a [`toast`] reports.
///
/// [`Default`] is `ToastTone::Success`, used when no tone is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ToastTone {
    /// A grey notice without a status.
    Neutral,
    /// Information the reader may want.
    Info,
    /// A completed action.
    #[default]
    Success,
    /// Something that needs attention.
    Warning,
    /// A failure. Error toasts stay until closed.
    Error,
}

impl ToastTone {
    /// Classes for the tone and its timing: errors slide in and stay, the rest hide
    /// themselves after five seconds.
    fn classes(self) -> StaticClass {
        match self {
            Self::Neutral => class!("toast-auto"),
            Self::Info => class!("alert-info toast-auto"),
            Self::Success => class!("alert-success toast-auto"),
            Self::Warning => class!("alert-warning toast-auto"),
            Self::Error => class!("alert-error toast-enter"),
        }
    }
}

/// Classes for a toast: a tinted alert sized for the corner of the screen.
const TOAST: StaticClass = class!("alert alert-soft max-w-md shadow-lg");

/// The fixed corner that stacks [`toast`]s, bottom right.
///
/// Render it once per page, for example in the layout, with the page's toasts as
/// children. `attrs` are forwarded to the `<div>`, with extra classes added to its
/// classes; add `toast-top` or `toast-start` to move it.
///
/// ```ignore
/// view! {
///     toaster(
///         if let Some(flash) = flash {
///             toast(tone: flash.tone, (flash.message))
///         }
///     )
/// }
/// ```
#[component]
pub async fn toaster(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("toast toast-end z-50", attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}

/// A short notice that slides into the [`toaster`].
///
/// `tone` defaults to `Success`. Toasts hide themselves after five seconds, paused
/// while hovered or focused; set `--toast-delay` in `style` to change that. Error
/// toasts stay until closed. Every toast has a close button that works without
/// script. Pass the message as children. `attrs` are forwarded to the `<div>`, with
/// extra classes added to its classes.
///
/// ```ignore
/// view! {
///     toast(tone: ToastTone::Error, "Could not save the project.")
/// }
/// ```
#[component]
pub async fn toast(
    #[default] tone: ToastTone,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let role = if tone == ToastTone::Error {
        "alert"
    } else {
        "status"
    };
    Ok(view! {
        <div
            role=(role)
            class=(class!(TOAST, tone.classes(), attrs.remove("class")))
            (attrs)
        >
            <span>(child)</span>
            <label class="toast-close" aria-label="Dismiss">
                <input type="checkbox" class="sr-only">
                <span aria-hidden="true">"×"</span>
            </label>
        </div>
    })
}
