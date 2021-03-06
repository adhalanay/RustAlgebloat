#![allow(trivial_numeric_casts)]

use std::fmt;
use std::ops::*;

use traits::{MatrixRawGet, MatrixRawSet, MatrixShape, MatrixView, SameShape};
use matrix::write_mat;

trait UnwrapRange
{
	#[inline]
	fn get(&self, end: usize) -> (usize, usize);
}

macro_rules! view_impl
{
	($row_range_type: ident, $col_range_type: ident, $idx_type: ident) =>
	{
		impl<T: MatrixShape>
		MatrixView<$row_range_type<$idx_type>, $col_range_type<$idx_type>> for
		T
		{
			unsafe fn unsafe_view(self, row_range: $row_range_type<$idx_type>, col_range: $col_range_type<$idx_type>) -> View<T>
			{
				let (row_start, row_end) = row_range.get(self.nrow());
				let (col_start, col_end) = col_range.get(self.ncol());
				View::unsafe_new(self, row_start, col_start, row_end, col_end)
			}

			fn view(self, row_range: $row_range_type<$idx_type>, col_range: $col_range_type<$idx_type>) -> View<T>
			{
				let (row_start, row_end) = row_range.get(self.nrow());
				let (col_start, col_end) = col_range.get(self.ncol());
				View::new(self, row_start, col_start, row_end, col_end)
			}
		}
	};
	
	($idx_type: ident) =>
	{
		impl UnwrapRange for Range<$idx_type>
		{
			#[inline]
			fn get(&self, _: usize) -> (usize, usize)
			{
				(self.start as usize, self.end as usize)
			}
		}

		impl UnwrapRange for RangeFrom<$idx_type>
		{
			#[inline]
			fn get(&self, end: usize) -> (usize, usize)
			{
				(self.start as usize, end)
			}
		}

		impl UnwrapRange for RangeTo<$idx_type>
		{
			#[inline]
			fn get(&self, _: usize) -> (usize, usize)
			{
				(0, self.end as usize)
			}
		}
		
		view_impl!(Range,     Range,     $idx_type);
		view_impl!(RangeFrom, Range,     $idx_type);
		view_impl!(RangeTo,   Range,     $idx_type);
		view_impl!(Range,     RangeFrom, $idx_type);
		view_impl!(RangeFrom, RangeFrom, $idx_type);
		view_impl!(RangeTo,   RangeFrom, $idx_type);
		view_impl!(Range,     RangeTo,   $idx_type);
		view_impl!(RangeFrom, RangeTo,   $idx_type);
		view_impl!(RangeTo,   RangeTo,   $idx_type);
	}
}

view_impl!(usize);
view_impl!(i32);

#[derive(Copy)]
pub struct View<T>
{
	base: T,
	row_start: usize,
	col_start: usize,
	row_end: usize,
	col_end: usize,
}

impl<T: MatrixShape>
View<T>
{
	pub unsafe fn unsafe_new(base: T, row_start: usize, col_start: usize, row_end: usize, col_end: usize) -> View<T>
	{
		View{ base: base, row_start: row_start, col_start: col_start, row_end: row_end, col_end: col_end }
	}

	pub fn new(base: T, row_start: usize, col_start: usize, row_end: usize, col_end: usize) -> View<T>
	{
		assert!(row_start <= row_end);
		assert!(col_start <= col_end);
		assert!(row_end <= base.nrow());
		assert!(col_end <= base.ncol());
		View{ base: base, row_start: row_start, col_start: col_start, row_end: row_end, col_end: col_end }
	}
}

impl<T: MatrixRawGet>
MatrixRawGet for
View<T>
{
	unsafe fn raw_get(&self, r: usize, c: usize) -> f64
	{
		self.base.raw_get(r + self.row_start, c + self.col_start)
	}
}

impl<T: MatrixRawSet>
MatrixRawSet for
View<T>
{
	unsafe fn raw_set(&self, r: usize, c: usize, val: f64)
	{
		self.base.raw_set(r + self.row_start, c + self.col_start, val)
	}
}

impl<T>
MatrixShape for
View<T>
{
	fn nrow(&self) -> usize
	{
		self.row_end - self.row_start
	}
	fn ncol(&self) -> usize
	{
		self.col_end - self.col_start
	}
}

impl<T: MatrixShape>
SameShape for
View<T>
{
	fn same_shape(&self, nrow: usize, ncol: usize) -> bool
	{
		self.nrow() == nrow && self.ncol() == ncol
	}
}

impl<T: Clone>
Clone for
View<T>
{
	fn clone(&self) -> View<T>
	{
		View{ base: self.base.clone(), row_start: self.row_start, col_start: self.col_start, row_end: self.row_end, col_end: self.col_end }
	}
}

impl<T: MatrixRawGet + MatrixShape>
fmt::Display for
View<T>
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		write_mat(buf, self)
	}
}
