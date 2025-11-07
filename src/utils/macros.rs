//! Module containing just useful macros.

/// Implements `PartialEq`, `Eq` and `Hash` for a type based on a single field
/// (usually component name).
/// 
/// # Example
/// ```rust
/// struct Component {
///     name: String,
///     // other fields...
/// }
/// 
/// impl_eq_name!(Component::name);
#[macro_export]
macro_rules! impl_eq_name {
    ($type:ident :: $name:ident) => {
        impl PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                self.$name == other.$name
            }
        }

        impl std::hash::Hash for $type {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.$name.hash(state);
            }
        }

        impl Eq for $type {}
    };
}
