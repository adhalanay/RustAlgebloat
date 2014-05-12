use std::fmt;

use traits::{MatrixRawGet, MatrixRawSet, MatrixShape};
use matrix::write_mat;

pub struct ColumnAccessor<T>
{
	base: T,
	col: uint
}

impl<T: MatrixShape>
ColumnAccessor<T>
{
	pub unsafe fn unsafe_new(base: T, col: uint) -> ColumnAccessor<T>
	{
		ColumnAccessor{ base: base, col: col }
	}

	pub fn new(base: T, col: uint) -> ColumnAccessor<T>
	{
		assert!(col < base.ncol());
		ColumnAccessor{ base: base, col: col }
	}
}

impl<T: MatrixShape>
MatrixShape for
ColumnAccessor<T>
{
	fn nrow(&self) -> uint
	{
		self.base.nrow()
	}
	fn ncol(&self) -> uint
	{
		1
	}
}

//~ impl<T: MatrixShape>
//~ LengthEq for
//~ ColumnAccessor<T>
//~ {
	//~ fn len_eq(&self, other_len: uint) -> bool
	//~ {
		//~ other_len == self.len()
	//~ }
//~ }

impl<T: MatrixRawGet + MatrixShape>
MatrixRawGet for
ColumnAccessor<T>
{
	unsafe fn raw_get(&self, r: uint, _: uint) -> f64
	{
		self.base.raw_get(r, self.col)
	}
}

impl<T: MatrixRawSet + MatrixShape>
MatrixRawSet for
ColumnAccessor<T>
{
	unsafe fn raw_set(&self, r: uint, _: uint, v: f64)
	{
		self.base.raw_set(r, self.col, v)
	}
}

impl<T: MatrixShape>
Container for
ColumnAccessor<T>
{
	fn len(&self) -> uint
	{
		self.base.nrow()
	}
}

impl<T: MatrixShape + MatrixRawGet>
fmt::Show for
ColumnAccessor<T>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf.buf, self)
	}
}

impl<T: Clone>
Clone for
ColumnAccessor<T>
{
	fn clone(&self) -> ColumnAccessor<T>
	{
		ColumnAccessor{ base: self.base.clone(), col: self.col }
	}
}
