#[macro_export]
macro_rules! async_rohanasan {
    ($($body:tt)*) => {
        #[$crate::async_std::main(crate = $crate::async_std)]
        $($body)*
    };
}
