//! Bot relative API low-level type define.

macro_rules! impl_method {
    ($MethodType: ty, $url_fragment: expr, $ReTurnType: ty) => {
        impl $crate::bot::methods::Method for $MethodType {
            const NAME: &'static str = $url_fragment;
            type Item = $ReTurnType;
        }
    };
}

macro_rules! impl_method_table {

    ($([$MethodType: ty, $url_fragment: expr, $ReTurnType: ty]),*) => {
        $(impl_method!($MethodType, $url_fragment, $ReTurnType);)*
    };
}

pub mod games;
pub mod inline_mode;
pub mod methods;
pub mod types;
mod utils;
