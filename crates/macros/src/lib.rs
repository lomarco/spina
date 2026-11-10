use symbol::Symbol;

macro_rules! symbols {
    (
        keywords {
            $($kw_name:ident : $kw_str:literal),* $(,)?
        }
        types {
            $($ty_name:ident : $ty_str:literal),* $(,)?
        }
    ) => {
        const SYMBOLS: &[&str] = &[$($kw_str,)* $($ty_str,)*];

        #[allow(non_upper_case_globals)]
        pub mod kw {
            symbols!(@consts 0u32; $($kw_name), *);
        }

        #[allow(non_upper_case_globals)]
        pub mod ty {
            symbols!(
                @consts
                symbols!(@count 0u32; $($kw_name), *);
                $($ty_name), *
            );
        }
    };

    (@consts $i:expr; $name:ident $(, $rest:ident)*) => {
        pub const $name: crate::Symbol = crate::Symbol::new($i);
        symbols!(@consts $i + 1; $($rest),*);
    };
    (@consts $i:expr;) => {};

    (@count $i:expr; $name:ident $(, $rest:ident)*) => {
        symbols!(@count $i + 1; $($rest),*)
    };
    (@count $i:expr;) => {
        $i
    };
}

symbols! {
    keywords {
        Let: "let",
        Const: "const",
        If: "if",
        For: "for",
        While: "while",
        Fn: "fn",
    }

    types {
        I8: "i8",
        U8: "u8",
        I16: "i16",
        U16: "u16",
        I32: "i32",
        U32: "u32",
        I64: "i64",
        U64: "u64",
        I128: "i128",
        U128: "u128",
        Isize: "isize",
        Usize: "usize",
        F16: "f16",
        F32: "f32",
        F64: "f64",
        F128: "f128",
    }
}
