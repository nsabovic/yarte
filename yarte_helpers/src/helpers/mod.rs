pub mod integers;
pub mod io_fmt;

#[repr(align(32))]
#[cfg(target_pointer_width = "64")]
pub struct Aligned256<T>(pub T);

#[cfg(not(target_pointer_width = "64"))]
pub struct Aligned256<T>(pub T);

#[cfg(feature = "big-num-32")]
pub mod big_num_32;
#[cfg(feature = "display-fn")]
pub mod display_fn;

#[cfg(feature = "bytes-buf")]
mod bytes;
#[cfg(feature = "fixed")]
mod fixed;
#[cfg(feature = "markup")]
mod markup;

#[cfg(feature = "bytes-buf")]
pub use self::bytes::{RenderBytes, RenderBytesA, RenderBytesSafe, RenderBytesSafeA};
#[cfg(feature = "fixed")]
pub use self::fixed::{RenderFixed, RenderFixedA, RenderSafe, RenderSafeA};
#[cfg(feature = "markup")]
pub use self::markup::{Render, RenderA};
#[cfg(feature = "ryu-ad")]
pub mod ryu;

pub trait IntoCopyIterator: IntoIterator {
    fn __into_citer(self) -> <Self as IntoIterator>::IntoIter;
}

impl<I: IntoIterator + Sized> IntoCopyIterator for I {
    #[inline]
    fn __into_citer(self) -> <Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}
