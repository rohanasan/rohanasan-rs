extern crate async_std;
/// **async_std** implementation of async-std just a wrapper for making things easier for you
#[macro_export]
macro_rules! async_std {
    ($($body:tt)*) => {
        #[async_std::main]
        $($body)*
    };
}
// this was just an effort to make async_std available inside rohanasan
// a way so that just importing async_std will provide you with async_std attribute features.
// I have mentioned about async_std in the documentation comment above.