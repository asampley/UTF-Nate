pub trait Conv {
	#[inline(always)]
	fn conv<T>(self) -> T
	where
		Self: Into<T>,
	{
		Into::<T>::into(self)
	}
}

impl<T> Conv for T {}
