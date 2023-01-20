macro_rules! cfg_v1 {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "v1")]
            $item
        )*
    };
}

macro_rules! cfg_v2 {
    ($($item:item)*) => {
        $(
            #[cfg(feature = "v2")]
            $item
        )*
    };
}

cfg_v1! {
    pub mod api_v1;
    pub use api_v1 as api;
}

cfg_v2! {
    pub mod api_v2;
    pub use api_v2 as api;
}

#[cfg(feature = "util")]
pub mod util;
