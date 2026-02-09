macro_rules! enum_fields {
    ($($variant:ident),* $(,)?) => {
        enum Fields {
            $($variant),*
        }

        impl Fields {
            const VALUES: &'static [Fields] = &[$(Fields::$variant),*];
        }
    };
}

pub(crate) use enum_fields;
