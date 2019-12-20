/// Constructs a new span.
///
/// See [the top-level documentation][lib] for details on the syntax accepted by
/// this macro.
///
/// [lib]: index.html#using-the-macros
///
/// # Examples
///
/// Creating a new span:
/// ```
/// # use tracing::{span, Level};
/// # fn main() {
/// let span = span!(Level::TRACE, "my span");
/// let _enter = span.enter();
/// // do work inside the span...
/// # }
/// ```
#[macro_export]
macro_rules! span {
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr) => {
        $crate::span!(target: $target, parent: $parent, $lvl, $name,)
    };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr, $($fields:tt)*) => {
        $crate::__tracing_mk_span!(target: $target, parent: $parent, $lvl, $name, $($fields)*)
    };
    (target: $target:expr, $lvl:expr, $name:expr, $($fields:tt)*) => {
        $crate::__tracing_mk_span!(target: $target, $lvl, $name, $($fields)*)
    };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr) => {
        $crate::span!(target: $target, parent: $parent, $lvl, $name,)
    };
    (parent: $parent:expr, $lvl:expr,  $name:expr, $($fields:tt)*) => {
        $crate::span!(
            target: module_path!(),
            parent: $parent,
            $lvl,
            $name,
            $($fields)*
        )
    };
    (parent: $parent:expr, $lvl:expr, $name:expr) => {
        $crate::span!(
            target: module_path!(),
            parent: $parent,
            $lvl,
            $name,
        )
    };
    (target: $target:expr, $lvl:expr, $name:expr, $($fields:tt)*) => {
        $crate::span!(
            target: $target,
            $lvl,
            $name,
            $($fields)*
        )
    };
    (target: $target:expr, $lvl:expr, $name:expr) => {
        $crate::span!(target: $target, $lvl, $name,)
    };
    ($lvl:expr, $name:expr, $($fields:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $lvl,
            $name,
            $($fields)*
        )
    };
    ($lvl:expr, $name:expr) => {
        $crate::span!(
            target: module_path!(),
            $lvl,
            $name,
        )
    };
}

/// Constructs a span at the trace level.
///
/// [Fields] and [attributes] are set using the same syntax as the [`span!`]
/// macro.
///
/// See [the top-level documentation][lib] for details on the syntax accepted by
/// this macro.
///
/// [lib]: index.html#using-the-macros
/// [attributes]: index.html#configuring-attributes
/// [Fields]: index.html#recording-fields
/// [`span!`]: macro.span.html
///
/// # Examples
///
/// ```rust
/// # use tracing::{trace_span, span, Level};
/// # fn main() {
/// trace_span!("my_span");
/// // is equivalent to:
/// span!(Level::TRACE, "my_span");
/// # }
/// ```
///
/// ```rust
/// # use tracing::{trace_span, span, Level};
/// # fn main() {
/// let span = trace_span!("my span");
/// span.in_scope(|| {
///     // do work inside the span...
/// });
/// # }
/// ```
#[macro_export]
macro_rules! trace_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            parent: $parent,
            $crate::Level::TRACE,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, parent: $parent:expr, $name:expr) => {
        $crate::trace_span!(target: $target, parent: $parent, $name,)
    };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            $name,
            $($field)*
        )
    };
    (parent: $parent:expr, $name:expr) => {
        $crate::trace_span!(parent: $parent, $name,)
    };
    (target: $target:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            $crate::Level::TRACE,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, $name:expr) => {
        $crate::trace_span!(target: $target, $name,)
    };
    ($name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $crate::Level::TRACE,
            $name,
            $($field)*
        )
    };
    ($name:expr) => { $crate::trace_span!($name,) };
}

/// Constructs a span at the debug level.
///
/// [Fields] and [attributes] are set using the same syntax as the [`span!`]
/// macro.
///
/// See [the top-level documentation][lib] for details on the syntax accepted by
/// this macro.
///
/// [lib]: index.html#using-the-macros
/// [attributes]: index.html#configuring-attributes
/// [Fields]: index.html#recording-fields
/// [`span!`]: macro.span.html
///
/// # Examples
///
/// ```rust
/// # use tracing::{debug_span, span, Level};
/// # fn main() {
/// debug_span!("my_span");
/// // is equivalent to:
/// span!(Level::DEBUG, "my_span");
/// # }
/// ```
///
/// ```rust
/// # use tracing::debug_span;
/// # fn main() {
/// let span = debug_span!("my span");
/// span.in_scope(|| {
///     // do work inside the span...
/// });
/// # }
/// ```
#[macro_export]
macro_rules! debug_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            parent: $parent,
            $crate::Level::DEBUG,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, parent: $parent:expr, $name:expr) => {
        $crate::debug_span!(target: $target, parent: $parent, $name,)
    };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            $name,
            $($field)*
        )
    };
    (parent: $parent:expr, $name:expr) => {
        $crate::debug_span!(parent: $parent, $name,)
    };
    (target: $target:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            $crate::Level::DEBUG,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, $name:expr) => {
        $crate::debug_span!(target: $target, $name,)
    };
    ($name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $crate::Level::DEBUG,
            $name,
            $($field)*
        )
    };
    ($name:expr) => {$crate::debug_span!($name,)};
}

/// Constructs a span at the info level.
///
/// [Fields] and [attributes] are set using the same syntax as the [`span!`]
/// macro.
///
/// See [the top-level documentation][lib] for details on the syntax accepted by
/// this macro.
///
/// [lib]: index.html#using-the-macros
/// [attributes]: index.html#configuring-attributes
/// [Fields]: index.html#recording-fields
/// [`span!`]: macro.span.html
///
/// # Examples
///
/// ```rust
/// # use tracing::{span, info_span, Level};
/// # fn main() {
/// info_span!("my_span");
/// // is equivalent to:
/// span!(Level::INFO, "my_span");
/// # }
/// ```
///
/// ```rust
/// # use tracing::info_span;
/// # fn main() {
/// let span = info_span!("my span");
/// span.in_scope(|| {
///     // do work inside the span...
/// });
/// # }
/// ```
#[macro_export]
macro_rules! info_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            parent: $parent,
            $crate::Level::INFO,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, parent: $parent:expr, $name:expr) => {
        $crate::info_span!(target: $target, parent: $parent, $name,)
    };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            $name,
            $($field)*
        )
    };
    (parent: $parent:expr, $name:expr) => {
        $crate::info_span!(parent: $parent, $name,)
    };
    (target: $target:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            $crate::Level::INFO,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, $name:expr) => {
        $crate::info_span!(target: $target, $name,)
    };
    ($name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $crate::Level::INFO,
            $name,
            $($field)*
        )
    };
    ($name:expr) => {$crate::info_span!($name,)};
}

/// Constructs a span at the warn level.
///
/// [Fields] and [attributes] are set using the same syntax as the [`span!`]
/// macro.
///
/// See [the top-level documentation][lib] for details on the syntax accepted by
/// this macro.
///
/// [lib]: index.html#using-the-macros
/// [attributes]: index.html#configuring-attributes
/// [Fields]: index.html#recording-fields
/// [`span!`]: macro.span.html
///
/// # Examples
///
/// ```rust
/// # use tracing::{warn_span, span, Level};
/// # fn main() {
/// warn_span!("my_span");
/// // is equivalent to:
/// span!(Level::WARN, "my_span");
/// # }
/// ```
///
/// ```rust
/// use tracing::warn_span;
/// # fn main() {
/// let span = warn_span!("my span");
/// span.in_scope(|| {
///     // do work inside the span...
/// });
/// # }
/// ```
#[macro_export]
macro_rules! warn_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            parent: $parent,
            $crate::Level::WARN,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, parent: $parent:expr, $name:expr) => {
        $crate::warn_span!(target: $target, parent: $parent, $name,)
    };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            $name,
            $($field)*
        )
    };
    (parent: $parent:expr, $name:expr) => {
        $crate::warn_span!(parent: $parent, $name,)
    };
    (target: $target:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            $crate::Level::WARN,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, $name:expr) => {
        $crate::warn_span!(target: $target, $name,)
    };
    ($name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $crate::Level::WARN,
            $name,
            $($field)*
        )
    };
    ($name:expr) => {$crate::warn_span!($name,)};
}
/// Constructs a span at the error level.
///
/// [Fields] and [attributes] are set using the same syntax as the [`span!`]
/// macro.
///
/// See [the top-level documentation][lib] for details on the syntax accepted by
/// this macro.
///
/// [lib]: index.html#using-the-macros
/// [attributes]: index.html#configuring-attributes
/// [Fields]: index.html#recording-fields
/// [`span!`]: macro.span.html
///
/// # Examples
///
/// ```rust
/// # use tracing::{span, error_span, Level};
/// # fn main() {
/// error_span!("my_span");
/// // is equivalent to:
/// span!(Level::ERROR, "my_span");
/// # }
/// ```
///
/// ```rust
/// # use tracing::error_span;
/// # fn main() {
/// let span = error_span!("my span");
/// span.in_scope(|| {
///     // do work inside the span...
/// });
/// # }
/// ```
#[macro_export]
macro_rules! error_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            parent: $parent,
            $crate::Level::ERROR,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, parent: $parent:expr, $name:expr) => {
        $crate::error_span!(target: $target, parent: $parent, $name,)
    };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            $name,
            $($field)*
        )
    };
    (parent: $parent:expr, $name:expr) => {
        $crate::error_span!(parent: $parent, $name,)
    };
    (target: $target:expr, $name:expr, $($field:tt)*) => {
        $crate::span!(
            target: $target,
            $crate::Level::ERROR,
            $name,
            $($field)*
        )
    };
    (target: $target:expr, $name:expr) => {
        $crate::error_span!(target: $target, $name,)
    };
    ($name:expr, $($field:tt)*) => {
        $crate::span!(
            target: module_path!(),
            $crate::Level::ERROR,
            $name,
            $($field)*
        )
    };
    ($name:expr) => {$crate::error_span!($name,)};
}

/// Constructs a new `Event`.
///
/// The event macro is invoked with a `Level` and up to 32 key-value fields.
/// Optionally, a format string and arguments may follow the fields; this will
/// be used to construct an implicit field named "message".
///
/// See [the top-level documentation][lib] for details on the syntax accepted by
/// this macro.
///
/// [lib]: index.html#using-the-macros
///
/// # Examples
///
/// ```rust
/// use tracing::{event, Level};
///
/// # fn main() {
/// let data = (42, "forty-two");
/// let private_data = "private";
/// let error = "a bad error";
///
/// event!(Level::ERROR, %error, "Received error");
/// event!(
///     target: "app_events",
///     Level::WARN,
///     private_data,
///     ?data,
///     "App warning: {}",
///     error
/// );
/// event!(Level::INFO, the_answer = data.0);
/// # }
/// ```
///
// /// Note that *unlike `span!`*, `event!` requires a value for all fields. As
// /// events are recorded immediately when the macro is invoked, there is no
// /// opportunity for fields to be recorded later. A trailing comma on the final
// /// field is valid.
// ///
// /// For example, the following does not compile:
// /// ```rust,compile_fail
// /// # #[macro_use]
// /// # extern crate tracing;
// /// # use tracing::Level;
// /// # fn main() {
// /// event!(Level::INFO, foo = 5, bad_field, bar = "hello")
// /// #}
// /// ```
#[macro_export]
macro_rules! event {
    (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } )=> ({
        {
            $crate::__tracing_log!(
                target: $target,
                $lvl,
                $($fields)*
            );

            if $crate::level_enabled!($lvl) {
                #[allow(unused_imports)]
                use $crate::{callsite, dispatcher, Event, field::{Value, ValueSet}};
                use $crate::callsite::Callsite;
                let callsite = callsite! {
                    name: concat!(
                        "event ",
                        file!(),
                        ":",
                        line!()
                    ),
                    kind: $crate::metadata::Kind::EVENT,
                    target: $target,
                    level: $lvl,
                    fields: $($fields)*
                };
                if $crate::is_enabled!(callsite) {
                    let meta = callsite.metadata();
                    Event::child_of($parent, meta, &$crate::valueset!(meta.fields(), $($fields)*) );
                }
            }
        }
    });

    (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => ({
        $crate::event!(
            target: $target,
            parent: $parent,
            $lvl,
            { message = format_args!($($arg)+), $($fields)* }
        )
    });
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $lvl, { $($k).+ = $($fields)* })
    );
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $($arg:tt)+) => (
        $crate::event!(target: $target, parent: $parent, $lvl, { $($arg)+ })
    );
    (target: $target:expr, $lvl:expr, { $($fields:tt)* } )=> ({
        {
            $crate::__tracing_log!(
                target: $target,
                $lvl,
                $($fields)*
            );
            if $crate::level_enabled!($lvl) {
                #[allow(unused_imports)]
                use $crate::{callsite, dispatcher, Event, field::{Value, ValueSet}};
                use $crate::callsite::Callsite;
                let callsite = callsite! {
                    name: concat!(
                        "event ",
                        file!(),
                        ":",
                        line!()
                    ),
                    kind: $crate::metadata::Kind::EVENT,
                    target: $target,
                    level: $lvl,
                    fields: $($fields)*
                };
                if $crate::is_enabled!(callsite) {
                    let meta = callsite.metadata();
                    Event::dispatch(meta, &$crate::valueset!(meta.fields(), $($fields)*) );
                }
            }
        }
    });
    (target: $target:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => ({
        $crate::event!(
            target: $target,
            $lvl,
            { message = format_args!($($arg)+), $($fields)* }
        )
    });
    (target: $target:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => (
        $crate::event!(target: $target, $lvl, { $($k).+ = $($fields)* })
    );
    (target: $target:expr, $lvl:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $lvl, { $($arg)+ })
    );
    (parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $lvl,
            { message = format_args!($($arg)+), $($fields)* }
        )
    );
    (parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            parent: $parent,
            { message = format_args!($($arg)+), $($fields)* }
        )
    );
    (parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $lvl,
            { $($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $lvl:expr, ?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $lvl,
            { ?$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $lvl:expr, %$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $lvl,
            { %$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $lvl:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $lvl,
            { $($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $lvl:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $lvl,
            { %$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $lvl:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $lvl,
            { ?$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $lvl:expr, $($arg:tt)+ ) => (
        $crate::event!(target: module_path!(), parent: $parent, $lvl, { $($arg)+ })
    );
    ( $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { message = format_args!($($arg)+), $($fields)* }
        )
    );
    ( $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { message = format_args!($($arg)+), $($fields)* }
        )
    );
    ($lvl:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { $($k).+ = $($field)*}
        )
    );
    ($lvl:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { $($k).+, $($field)*}
        )
    );
    ($lvl:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { ?$($k).+, $($field)*}
        )
    );
    ($lvl:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $lvl,
            { %$($k).+, $($field)*}
        )
    );
    ($lvl:expr, ?$($k:ident).+) => (
        $crate::event!($lvl, ?$($k).+,)
    );
    ($lvl:expr, %$($k:ident).+) => (
        $crate::event!($lvl, %$($k).+,)
    );
    ($lvl:expr, $($k:ident).+) => (
        $crate::event!($lvl, $($k).+,)
    );
    ( $lvl:expr, $($arg:tt)+ ) => (
        $crate::event!(target: module_path!(), $lvl, { $($arg)+ })
    );
}

/// Constructs an event at the trace level.
///
/// This functions similarly to the [`event!`] macro. See [the top-level
/// documentation][lib] for details on the syntax accepted by
/// this macro.
///
/// [`event!`]: macro.event.html
/// [lib]: index.html#using-the-macros
///
/// # Examples
///
/// ```rust
/// use tracing::trace;
/// # #[derive(Debug, Copy, Clone)] struct Position { x: f32, y: f32 }
/// # impl Position {
/// # const ORIGIN: Self = Self { x: 0.0, y: 0.0 };
/// # fn dist(&self, other: Position) -> f32 {
/// #    let x = (other.x - self.x).exp2(); let y = (self.y - other.y).exp2();
/// #    (x + y).sqrt()
/// # }
/// # }
/// # fn main() {
/// let pos = Position { x: 3.234, y: -1.223 };
/// let origin_dist = pos.dist(Position::ORIGIN);
///
/// trace!(position = ?pos, ?origin_dist);
/// trace!(
///     target: "app_events",
///     position = ?pos,
///     "x is {} and y is {}",
///     if pos.x >= 0.0 { "positive" } else { "negative" },
///     if pos.y >= 0.0 { "positive" } else { "negative" }
/// );
/// # }
/// ```
#[macro_export]
macro_rules! trace {
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE,  { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::TRACE, {}, $($arg)+)
    );
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { $($field)+ },
            $($arg)+
        )
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { $($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { ?$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { %$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { $($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { ?$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            { %$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::TRACE,
            {},
            $($arg)+
        )
    );
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, { $($k).+ $($field)+ })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, { $($k).+ $($field)+ })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, { $($k).+ $($field)+ })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::TRACE, {}, $($arg)+)
    );
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { $($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            {},
            $($arg)+
        )
    );
}

/// Constructs an event at the debug level.
///
/// This functions similarly to the [`event!`] macro. See [the top-level
/// documentation][lib] for details on the syntax accepted by
/// this macro.
///
/// [`event!`]: macro.event.html
/// [lib]: index.html#using-the-macros
///
/// # Examples
///
/// ```rust
/// use tracing::debug;
/// # fn main() {
/// # #[derive(Debug)] struct Position { x: f32, y: f32 }
///
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// debug!(?pos.x, ?pos.y);
/// debug!(target: "app_events", position = ?pos, "New position");
/// # }
/// ```
#[macro_export]
macro_rules! debug {
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG,  { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target,  parent: $parent, $crate::Level::DEBUG, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::DEBUG, {}, $($arg)+)
    );
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { $($field)+ },
            $($arg)+
        )
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { $($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { ?$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { %$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { $($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { ?$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            { %$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::DEBUG,
            {},
            $($arg)+
        )
    );
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, { $($k).+ $($field)+ })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, { $($k).+ $($field)+ })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, { $($k).+ $($field)+ })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::DEBUG, {}, $($arg)+)
    );
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { $($k).+ = $($field)*}
        )
    );
    (?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { ?$($k).+ = $($field)*}
        )
    );
    (%$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { %$($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::DEBUG,
            {},
            $($arg)+
        )
    );
}

/// Constructs an event at the info level.
///
/// This functions similarly to the [`event!`] macro. See [the top-level
/// documentation][lib] for details on the syntax accepted by
/// this macro.
///
/// [`event!`]: macro.event.html
/// [lib]: index.html#using-the-macros
///
/// # Examples
///
/// ```rust
/// use tracing::info;
/// # // this is so the test will still work in no-std mode
/// # #[derive(Debug)]
/// # pub struct Ipv4Addr;
/// # impl Ipv4Addr { fn new(o1: u8, o2: u8, o3: u8, o4: u8) -> Self { Self } }
/// # fn main() {
/// # struct Connection { port: u32,  speed: f32 }
/// use tracing::field;
///
/// let addr = Ipv4Addr::new(127, 0, 0, 1);
/// let conn = Connection { port: 40, speed: 3.20 };
///
/// info!(conn.port, "connected to {:?}", addr);
/// info!(
///     target: "connection_events",
///     ip = ?addr,
///     conn.port,
///     ?conn.speed,
/// );
/// # }
/// ```
#[macro_export]
macro_rules! info {
     (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::INFO, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::INFO, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target,  parent: $parent, $crate::Level::INFO, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::INFO, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::INFO, {}, $($arg)+)
    );
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { $($field)+ },
            $($arg)+
        )
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { $($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { ?$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { %$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { $($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { ?$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            { %$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::INFO,
            {},
            $($arg)+
        )
    );
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::INFO, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::INFO, { $($k).+ $($field)+ })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::INFO, { $($k).+ $($field)+ })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::INFO, { $($k).+ $($field)+ })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::INFO, {}, $($arg)+)
    );
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { $($k).+ = $($field)*}
        )
    );
    (?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { ?$($k).+ = $($field)*}
        )
    );
    (%$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { %$($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::INFO,
            {},
            $($arg)+
        )
    );
}

/// Constructs an event at the warn level.
///
/// This functions similarly to the [`event!`] macro. See [the top-level
/// documentation][lib] for details on the syntax accepted by
/// this macro.
///
/// [`event!`]: macro.event.html
/// [lib]: index.html#using-the-macros
///
/// # Examples
///
/// ```rust
/// use tracing::warn;
/// # fn main() {
///
/// let warn_description = "Invalid Input";
/// let input = &[0x27, 0x45];
///
/// warn!(?input, warning = warn_description);
/// warn!(
///     target: "input_events",
///     warning = warn_description,
///     "Received warning for input: {:?}", input,
/// );
/// # }
/// ```
#[macro_export]
macro_rules! warn {
     (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::WARN, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::WARN, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target,  parent: $parent, $crate::Level::WARN, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::WARN, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::WARN, {}, $($arg)+)
    );
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { $($field)+ },
            $($arg)+
        )
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { $($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { ?$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { %$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { $($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { ?$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            { %$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::WARN,
            {},
            $($arg)+
        )
    );
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::WARN, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::WARN, { $($k).+ $($field)+ })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::WARN, { $($k).+ $($field)+ })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::WARN, { $($k).+ $($field)+ })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::WARN, {}, $($arg)+)
    );
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { $($k).+ = $($field)*}
        )
    );
    (?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { ?$($k).+ = $($field)*}
        )
    );
    (%$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { %$($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::WARN,
            {},
            $($arg)+
        )
    );
}

/// Constructs an event at the error level.
///
/// This functions similarly to the [`event!`] macro. See [the top-level
/// documentation][lib] for details on the syntax accepted by
/// this macro.
///
/// [`event!`]: macro.event.html
/// [lib]: index.html#using-the-macros
///
/// # Examples
///
/// ```rust
/// use tracing::error;
/// # fn main() {
///
/// let (err_info, port) = ("No connection", 22);
///
/// error!(port, error = %err_info);
/// error!(target: "app_events", "App Error: {}", err_info);
/// error!({ info = err_info }, "error on port: {}", port);
/// # }
/// ```
#[macro_export]
macro_rules! error {
     (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::ERROR, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::ERROR, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target,  parent: $parent, $crate::Level::ERROR, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::ERROR, { $($k).+ $($field)+ })
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, parent: $parent, $crate::Level::ERROR, {}, $($arg)+)
    );
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { $($field)+ },
            $($arg)+
        )
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { $($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { ?$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { %$($k).+ = $($field)*}
        )
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { $($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { ?$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            { %$($k).+, $($field)*}
        )
    );
    (parent: $parent:expr, $($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            parent: $parent,
            $crate::Level::ERROR,
            {},
            $($arg)+
        )
    );
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, { $($field)* }, $($arg)*)
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, { $($k).+ $($field)+ })
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, { $($k).+ $($field)+ })
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, { $($k).+ $($field)+ })
    );
    (target: $target:expr, $($arg:tt)+ ) => (
        $crate::event!(target: $target, $crate::Level::ERROR, {}, $($arg)+)
    );
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { $($field)+ },
            $($arg)+
        )
    );
    ($($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { $($k).+ = $($field)*}
        )
    );
    (?$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { ?$($k).+ = $($field)*}
        )
    );
    (%$($k:ident).+ = $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::TRACE,
            { %$($k).+ = $($field)*}
        )
    );
    ($($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { $($k).+, $($field)*}
        )
    );
    (?$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { ?$($k).+, $($field)*}
        )
    );
    (%$($k:ident).+, $($field:tt)*) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { %$($k).+, $($field)*}
        )
    );
    (?$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { ?$($k).+ }
        )
    );
    (%$($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { %$($k).+ }
        )
    );
    ($($k:ident).+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            { $($k).+ }
        )
    );
    ($($arg:tt)+) => (
        $crate::event!(
            target: module_path!(),
            $crate::Level::ERROR,
            {},
            $($arg)+
        )
    );
}

/// Constructs a new static callsite for a span or event.
#[doc(hidden)]
#[macro_export]
macro_rules! callsite {
    (name: $name:expr, kind: $kind:expr, fields: $($fields:tt)*) => {{
        $crate::callsite! {
            name: $name,
            kind: $kind,
            target: module_path!(),
            level: $crate::Level::TRACE,
            fields: $($fields)*
        }
    }};
    (
        name: $name:expr,
        kind: $kind:expr,
        level: $lvl:expr,
        fields: $($fields:tt)*
    ) => {{
        $crate::callsite! {
            name: $name,
            kind: $kind,
            target: module_path!(),
            level: $lvl,
            fields: $($fields)*
        }
    }};
    (
        name: $name:expr,
        kind: $kind:expr,
        target: $target:expr,
        level: $lvl:expr,
        fields: $($fields:tt)*
    ) => {{
        use $crate::{callsite, subscriber::Interest, Metadata, __macro_support::*};
        struct MyCallsite;
        static META: Metadata<'static> = {
            $crate::metadata! {
                name: $name,
                target: $target,
                level: $lvl,
                fields: $crate::fieldset!( $($fields)* ),
                callsite: &MyCallsite,
                kind: $kind,
            }
        };
        static INTEREST: AtomicUsize = AtomicUsize::new(0);
        static REGISTRATION: Once = Once::new();
        impl MyCallsite {
            #[inline]
            fn interest(&self) -> Interest {
                match INTEREST.load(Ordering::Relaxed) {
                    0 => Interest::never(),
                    2 => Interest::always(),
                    _ => Interest::sometimes(),
                }
            }
        }
        impl callsite::Callsite for MyCallsite {
            fn set_interest(&self, interest: Interest) {
                let interest = match () {
                    _ if interest.is_never() => 0,
                    _ if interest.is_always() => 2,
                    _ => 1,
                };
                INTEREST.store(interest, Ordering::SeqCst);
            }

            fn metadata(&self) -> &Metadata {
                &META
            }
        }
        REGISTRATION.call_once(|| {
            callsite::register(&MyCallsite);
        });
        &MyCallsite
    }};
}

#[macro_export]
// TODO: determine if this ought to be public API?
#[doc(hidden)]
macro_rules! level_enabled {
    ($lvl:expr) => {
        $crate::dispatcher::has_been_set() && $lvl <= $crate::level_filters::STATIC_MAX_LEVEL
    };
}

#[macro_export]
// TODO: determine if this ought to be public API?
#[doc(hidden)]
macro_rules! is_enabled {
    ($callsite:expr) => {{
        let interest = $callsite.interest();
        if interest.is_never() {
            false
        } else if interest.is_always() {
            true
        } else {
            let meta = $callsite.metadata();
            $crate::dispatcher::get_default(|current| current.enabled(meta))
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! valueset {

    // === base case ===
    (@ { $(,)* $($val:expr),* $(,)* }, $next:expr $(,)*) => {
        &[ $($val),* ]
    };

    // === recursive case (more tts) ===

    // TODO(#1138): determine a new syntax for uninitialized span fields, and
    // re-enable this.
    // (@{ $(,)* $($out:expr),* }, $next:expr, $($k:ident).+ = _, $($rest:tt)*) => {
    //     $crate::valueset!(@ { $($out),*, (&$next, None) }, $next, $($rest)*)
    // };
    (@ { $(,)* $($out:expr),* }, $next:expr, $($k:ident).+ = ?$val:expr, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&debug(&$val) as &Value)) },
            $next,
            $($rest)*
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, $($k:ident).+ = %$val:expr, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&display(&$val) as &Value)) },
            $next,
            $($rest)*
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, $($k:ident).+ = $val:expr, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&$val as &Value)) },
            $next,
            $($rest)*
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, $($k:ident).+, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&$($k).+ as &Value)) },
            $next,
            $($rest)*
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, ?$($k:ident).+, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&debug(&$($k).+) as &Value)) },
            $next,
            $($rest)*
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, %$($k:ident).+, $($rest:tt)*) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&display(&$($k).+) as &Value)) },
            $next,
            $($rest)*
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, $($k:ident).+ = ?$val:expr) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&debug(&$val) as &Value)) },
            $next,
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, $($k:ident).+ = %$val:expr) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&display(&$val) as &Value)) },
            $next,
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, $($k:ident).+ = $val:expr) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&$val as &Value)) },
            $next,
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, $($k:ident).+) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&$($k).+ as &Value)) },
            $next,
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, ?$($k:ident).+) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&debug(&$($k).+) as &Value)) },
            $next,
        )
    };
    (@ { $(,)* $($out:expr),* }, $next:expr, %$($k:ident).+) => {
        $crate::valueset!(
            @ { $($out),*, (&$next, Some(&display(&$($k).+) as &Value)) },
            $next,
        )
    };
    // Remainder is unparseable, but exists --- must be format args!
    (@ { $(,)* $($out:expr),* }, $next:expr, $($rest:tt)+) => {
        $crate::valueset!(@ { (&$next, Some(&format_args!($($rest)+) as &Value)), $($out),* }, $next, )
    };

    // === entry ===
    ($fields:expr, $($kvs:tt)+) => {
        {
            #[allow(unused_imports)]
            use $crate::field::{debug, display, Value};
            let mut iter = $fields.iter();
            $fields.value_set($crate::valueset!(
                @ { },
                iter.next().expect("FieldSet corrupted (this is a bug)"),
                $($kvs)+
            ))
        }
    };
    ($fields:expr,) => {
        {
            $fields.value_set(&[])
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! fieldset {
    // == base case ==
    (@ { $(,)* $($out:expr),* $(,)* } $(,)*) => {
        &[ $($out),* ]
    };

    // == recursive cases (more tts) ==
    (@ { $(,)* $($out:expr),* } $($k:ident).+ = ?$val:expr, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),*, $crate::__tracing_stringify!($($k).+) } $($rest)*)
    };
    (@ { $(,)* $($out:expr),* } $($k:ident).+ = %$val:expr, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),*, $crate::__tracing_stringify!($($k).+) } $($rest)*)
    };
    (@ { $(,)* $($out:expr),* } $($k:ident).+ = $val:expr, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),*, $crate::__tracing_stringify!($($k).+) } $($rest)*)
    };
    // TODO(#1138): determine a new syntax for uninitialized span fields, and
    // re-enable this.
    // (@ { $($out:expr),* } $($k:ident).+ = _, $($rest:tt)*) => {
    //     $crate::fieldset!(@ { $($out),*, $crate::__tracing_stringify!($($k).+) } $($rest)*)
    // };
    (@ { $(,)* $($out:expr),* } ?$($k:ident).+, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),*, $crate::__tracing_stringify!($($k).+) } $($rest)*)
    };
    (@ { $(,)* $($out:expr),* } %$($k:ident).+, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),*, $crate::__tracing_stringify!($($k).+) } $($rest)*)
    };
    (@ { $(,)* $($out:expr),* } $($k:ident).+, $($rest:tt)*) => {
        $crate::fieldset!(@ { $($out),*, $crate::__tracing_stringify!($($k).+) } $($rest)*)
    };

    // Remainder is unparseable, but exists --- must be format args!
    (@ { $(,)* $($out:expr),* } $($rest:tt)+) => {
        $crate::fieldset!(@ { "message", $($out),*, })
    };

    // == entry ==
    ($($args:tt)*) => {
        $crate::fieldset!(@ { } $($args)*,)
    };

}

#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! level_to_log {
    ($level:expr) => {
        match $level {
            &$crate::Level::ERROR => $crate::log::Level::Error,
            &$crate::Level::WARN => $crate::log::Level::Warn,
            &$crate::Level::INFO => $crate::log::Level::Info,
            &$crate::Level::DEBUG => $crate::log::Level::Debug,
            _ => $crate::log::Level::Trace,
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __tracing_stringify {
    ($s:expr) => {
        stringify!($s)
    };
}

#[cfg(not(feature = "log"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __tracing_log {
    (target: $target:expr, $level:expr, $($field:tt)+ ) => {};
}

#[cfg(not(feature = "log"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __tracing_mk_span {
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr, $($fields:tt)*) => {
        {
            if $crate::level_enabled!($lvl) {
                use $crate::callsite;
                use $crate::callsite::Callsite;
                let callsite = $crate::callsite! {
                    name: $name,
                    kind: $crate::metadata::Kind::SPAN,
                    target: $target,
                    level: $lvl,
                    fields: $($fields)*
                };
                let meta = callsite.metadata();
                if $crate::is_enabled!(callsite) {
                    $crate::Span::child_of(
                        $parent,
                        meta,
                        &$crate::valueset!(meta.fields(), $($fields)*),
                    )
                } else {
                    $crate::Span::none()
                }
            } else {
                $crate::Span::none()
            }
        }
    };
    (target: $target:expr, $lvl:expr, $name:expr, $($fields:tt)*) => {
        {
            if $crate::level_enabled!($lvl) {
                use $crate::callsite;
                use $crate::callsite::Callsite;
                let callsite = $crate::callsite! {
                    name: $name,
                    kind: $crate::metadata::Kind::SPAN,
                    target: $target,
                    level: $lvl,
                    fields: $($fields)*
                };
                let meta = callsite.metadata();
                if $crate::is_enabled!(callsite) {
                    $crate::Span::new(
                        meta,
                        &$crate::valueset!(meta.fields(), $($fields)*),
                    )
                } else {
                    $crate::Span::none()
                }
            } else {
                $crate::Span::none()
            }
        }
    };
}

#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! __tracing_mk_span {
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr, $($fields:tt)*) => {
        {
            use $crate::callsite;
            use $crate::callsite::Callsite;
            let callsite = $crate::callsite! {
                name: $name,
                kind: $crate::metadata::Kind::SPAN,
                target: $target,
                level: $lvl,
                fields: $($fields)*
            };
            let meta = callsite.metadata();
            if $crate::level_enabled!($lvl) && $crate::is_enabled!(callsite) {
                $crate::Span::child_of(
                    $parent,
                    meta,
                    &$crate::valueset!(meta.fields(), $($fields)*),
                )
            } else {
                $crate::if_log_enabled! {{
                    let span = $crate::Span::new_disabled(meta);
                    span.record_all(&$crate::valueset!(meta.fields(), $($fields)*));
                    span
                } else {
                    $crate::Span::none()
                }}
            }
        }
    };
    (target: $target:expr, $lvl:expr, $name:expr, $($fields:tt)*) => {
        {
            use $crate::callsite;
            use $crate::callsite::Callsite;
            let callsite = $crate::callsite! {
                name: $name,
                kind: $crate::metadata::Kind::SPAN,
                target: $target,
                level: $lvl,
                fields: $($fields)*
            };
            let meta = callsite.metadata();
            if $crate::level_enabled!($lvl) && $crate::is_enabled!(callsite) {
                $crate::Span::new(
                    meta,
                    &$crate::valueset!(meta.fields(), $($fields)*),
                )
            } else {
                $crate::if_log_enabled! {{
                    let span = $crate::Span::new_disabled(meta);
                    span.record_all(&$crate::valueset!(meta.fields(), $($fields)*));
                    span
                } else {
                    $crate::Span::none()
                }}
            }
        }
    };
}

#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! __mk_format_string {
    // === base case ===
    (@ { $(,)* $($out:expr),* $(,)* } $(,)*) => {
        concat!( $($out),*)
    };

    // === recursive case (more tts), ===
    // ====== shorthand field syntax ===
    (@ { $(,)* $($out:expr),* }, ?$($k:ident).+, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { $($out),*, tracing::__tracing_stringify!($($k).+), "={:?} " }, $($rest)*)
    };
    (@ { $(,)* $($out:expr),* }, %$($k:ident).+, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { $($out),*, tracing::__tracing_stringify!($($k).+), "={} " }, $($rest)*)
    };
    (@ { $(,)* $($out:expr),* }, $($k:ident).+, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { $($out),*, tracing::__tracing_stringify!($($k).+), "={:?} " }, $($rest)*)
    };
    // ====== kv field syntax ===
    (@ { $(,)* $($out:expr),* }, message = $val:expr, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { $($out),*, "{} " }, $($rest)*)
    };
    (@ { $(,)* $($out:expr),* }, $($k:ident).+ = ?$val:expr, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { $($out),*, tracing::__tracing_stringify!($($k).+), "={:?} " }, $($rest)*)
    };
    (@ { $(,)* $($out:expr),* }, $($k:ident).+ = %$val:expr, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { $($out),*, tracing::__tracing_stringify!($($k).+), "={} " }, $($rest)*)
    };
    (@ { $(,)* $($out:expr),* }, $($k:ident).+ = $val:expr, $($rest:tt)*) => {
        tracing::__mk_format_string!(@ { $($out),*, tracing::__tracing_stringify!($($k).+), "={:?} " }, $($rest)*)
    };

    // === rest is unparseable --- must be fmt args ===
    (@ { $(,)* $($out:expr),* }, $($rest:tt)+) => {
        tracing::__mk_format_string!(@ { "{} ", $($out),* }, )
    };

    // === entry ===
    ($($kvs:tt)+) => {
        tracing::__mk_format_string!(@ { }, $($kvs)+,)
    };
    () => {
        ""
    }
}

#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! __mk_format_args {
    // === finished --- called into by base cases ===
    (@ { $(,)* $($out:expr),* $(,)* }, $fmt:expr, fields: $(,)*) => {
        format_args!($fmt, $($out),*)
    };

    // === base case (no more tts) ===
    (@ { $(,)* $($out:expr),* }, $fmt:expr, fields: $($k:ident).+ = ?$val:expr $(,)*) => {
        $crate::__mk_format_args!(@ { $($out),*, $val }, $fmt, fields: )
    };
    (@ { $(,)* $($out:expr),* }, $fmt:expr, fields: $($k:ident).+ = %$val:expr $(,)*) => {
        $crate::__mk_format_args!(@ { $($out),*, $val }, $fmt, fields: )
    };
    (@ { $(,)* $($out:expr),* }, $fmt:expr, fields: $($k:ident).+ = $val:expr $(,)*) => {
        $crate::__mk_format_args!(@ { $($out),*, $val }, $fmt, fields: )
    };
    // ====== shorthand field syntax ===
    (@ { $(,)* $($out:expr),* }, $fmt:expr, fields: ?$($k:ident).+ $(,)*) => {
        $crate::__mk_format_args!(@ { $($out),*, &$($k).+ }, $fmt, fields:)
    };
    (@ { $(,)* $($out:expr),* }, $fmt:expr, fields: %$($k:ident).+ $(,)*) => {
        $crate::__mk_format_args!(@ { $($out),*, &$($k).+ }, $fmt, fields: )
    };
    (@ { $(,)* $($out:expr),* }, $fmt:expr, fields: $($k:ident).+ $(,)*) => {
        $crate::__mk_format_args!(@ { $($out),*, &$($k).+ }, $fmt, fields: )
    };

    // === recursive case (more tts) ===
    (@ { $(,)* $($out:expr),* }, $fmt:expr, fields: $($k:ident).+ = ?$val:expr, $($rest:tt)+) => {
        $crate::__mk_format_args!(@ { $($out),*, $val }, $fmt, fields: $($rest)+)
    };
    (@ { $(,)* $($out:expr),* }, $fmt:expr, fields: $($k:ident).+ = %$val:expr, $($rest:tt)+) => {
        $crate::__mk_format_args!(@ { $($out),*, $val }, $fmt, fields: $($rest)+)
    };
    (@ { $(,)* $($out:expr),* }, $fmt:expr, fields: $($k:ident).+ = $val:expr, $($rest:tt)+) => {
        $crate::__mk_format_args!(@ { $($out),*, $val }, $fmt, fields: $($rest)+)
    };
    // ====== shorthand field syntax ===
    (@ { $(,)* $($out:expr),* }, $fmt:expr, fields: ?$($k:ident).+, $($rest:tt)+) => {
        $crate::__mk_format_args!(@ { $($out),*, &$($k).+ }, $fmt, fields: $($rest)+)
    };
    (@ { $(,)* $($out:expr),* }, $fmt:expr, fields: %$($k:ident).+, $($rest:tt)+) => {
        $crate::__mk_format_args!(@ { $($out),*, &$($k).+ }, $fmt, fields: $($rest)+)
    };
    (@ { $(,)* $($out:expr),* }, $fmt:expr, fields: $($k:ident).+, $($rest:tt)+) => {
        $crate::__mk_format_args!(@ { $($out),*, &$($k).+ }, $fmt, fields: $($rest)+)
    };

    // === rest is unparseable --- must be fmt args ===
    (@ { $(,)* $($out:expr),* }, $fmt:expr, fields: $($rest:tt)+) => {
        $crate::__mk_format_args!(@ { format_args!($($rest)+), $($out),* }, $fmt, fields: )
    };

    // === entry ===
    ($($kv:tt)*) => {
        {
            // use $crate::__mk_format_string;
            $crate::__mk_format_args!(@ { }, tracing::__mk_format_string!($($kv)*), fields: $($kv)*)
        }
    };
}

#[cfg(feature = "log")]
#[doc(hidden)]
#[macro_export]
macro_rules! __tracing_log {
    (target: $target:expr, $level:expr, $($field:tt)+ ) => {
        $crate::if_log_enabled! {{
            use $crate::log;
            let level = $crate::level_to_log!(&$level);
            if level <= log::STATIC_MAX_LEVEL {
                let log_meta = log::Metadata::builder()
                    .level(level)
                    .target($target)
                    .build();
                let logger = log::logger();
                if logger.enabled(&log_meta) {
                    logger.log(&log::Record::builder()
                        .file(Some(file!()))
                        .module_path(Some(module_path!()))
                        .line(Some(line!()))
                        .metadata(log_meta)
                        .args($crate::__mk_format_args!($($field)+))
                        .build());
                }
            }
        }}
    };
}

#[cfg(not(feature = "log"))]
#[doc(hidden)]
#[macro_export]
macro_rules! if_log_enabled {
    ($e:expr;) => {
        $crate::if_log_enabled! {{ $e }}
    };
    ($if_log:block) => {
        $crate::if_log_enabled! { $if_log else {} }
    };
    ($if_log:block else $else_block:block) => {
        $else_block
    };
}

#[cfg(all(feature = "log", not(feature = "log-always")))]
#[doc(hidden)]
#[macro_export]
macro_rules! if_log_enabled {
    ($e:expr;) => {
        $crate::if_log_enabled! {{ $e }}
    };
    ($if_log:block) => {
        $crate::if_log_enabled! { $if_log else {} }
    };
    ($if_log:block else $else_block:block) => {
        if !$crate::dispatcher::has_been_set() {
            $if_log
        } else {
            $else_block
        }
    };
}

#[cfg(all(feature = "log", feature = "log-always"))]
#[doc(hidden)]
#[macro_export]
macro_rules! if_log_enabled {
    ($e:expr;) => {
        $crate::if_log_enabled! {{ $e }}
    };
    ($if_log:block) => {
        $crate::if_log_enabled! { $if_log else {} }
    };
    ($if_log:block else $else_block:block) => {
        $if_log
    };
}
