use core::borrow::Borrow;

pub trait DotProduct {
	type Output;
	fn dot<T: Borrow<Self>>(&self, other: T) -> Self::Output;
}

pub trait CrossProduct {
	fn cross<T: Borrow<Self>>(&self, other: T) -> Self;
}
