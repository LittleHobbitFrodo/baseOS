//	mem/text.rs (ministd crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        use RENDERER;
        let _ = write!(*RENDERER.lock(), $($arg)*);
    }};
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        use RENDERER;
        let _ = writeln!(*RENDERER.lock(), $($arg)*);
    };
}