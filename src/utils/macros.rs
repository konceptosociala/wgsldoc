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