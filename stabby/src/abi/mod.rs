use core::ops::*;
#[doc(hidden)]
pub use typenum::*;

#[macro_export]
macro_rules! assert_optimal_layout {
    ($t: ty) => {
        const _: () = {
            assert!(<$t>::has_optimal_layout());
        };
    };
}

use stabby_macros::holes;

pub mod holes {
    include!(concat!(env!("OUT_DIR"), "/holes.rs"));
}
pub use fatptr::*;
mod fatptr;
pub use istabilize::IStabilize;
mod istabilize;
mod stable_impls;
pub mod vtable;

#[allow(type_alias_bounds)]
pub type Stable<Source: IStabilize> = Source::Stable;

pub struct AssertStable<T: IStable>(pub core::marker::PhantomData<T>);
impl<T: IStable> AssertStable<T> {
    pub const fn assert() -> Self {
        Self(core::marker::PhantomData)
    }
}

/// Lets you tell `stabby` that `T` has the same stable layout as `As`.
///
/// Lying about this link between `T` and `As` will cause UB.
pub struct StableLike<T, As> {
    pub value: T,
    marker: core::marker::PhantomData<As>,
}
impl<T: Clone, As> Clone for StableLike<T, As> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            marker: self.marker,
        }
    }
}
impl<T: Copy, As> Copy for StableLike<T, As> {}
impl<T, As: IStable> StableLike<T, As> {
    /// # Safety
    /// Refer to type documentation
    pub const unsafe fn stable(value: T) -> Self {
        Self {
            value,
            marker: core::marker::PhantomData,
        }
    }
}

impl<T, As: IStable> core::ops::Deref for StableLike<T, As> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T, As: IStable> core::ops::DerefMut for StableLike<T, As> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
unsafe impl<T, As: IStable> IStable for StableLike<T, As> {
    type Size = As::Size;
    type Align = As::Align;
    type IllegalValues = As::IllegalValues;
    type UnusedBits = As::UnusedBits;
    type HasExactlyOneNiche = As::HasExactlyOneNiche;
}

#[repr(C)]
pub struct Tuple2<A, B> {
    _0: A,
    _1: B,
}

#[repr(C)]
pub union Union<A, B> {
    pub(crate) _0: core::mem::ManuallyDrop<A>,
    pub(crate) _1: core::mem::ManuallyDrop<B>,
}
impl<A, B> Clone for Union<A, B> {
    fn clone(&self) -> Self {
        unsafe { core::ptr::read(self) }
    }
}

pub(crate) mod enums;

pub use istable::{Array, End, IStable};
mod istable;
pub type NonZeroHole = holes!([1, 0, 0, 0]);