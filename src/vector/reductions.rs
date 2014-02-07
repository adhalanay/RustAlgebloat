// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

//~ use vector::traits::VectorElems;
use vector::traits::VectorGet;

pub trait VectorReduce
{
	fn min(self) -> Option<(uint, f32)>;
	fn max(self) -> Option<(uint, f32)>;
}

impl<T: VectorGet + Container>
VectorReduce for
T
{
	fn min(self) -> Option<(uint, f32)>
	{
		let l = self.len();
		if l == 0
		{
			None
		}
		else
		{
			let mut ret = self.get(0);
			let mut ret_i = 0;
			for i in range(1, l)
			{
				unsafe
				{
					let c = self.unsafe_get(i);
					if c < ret
					{
						ret = c;
						ret_i = i;
					}
				}
			}
			Some((ret_i, ret))
		}
	}

	fn max(self) -> Option<(uint, f32)>
	{
		let l = self.len();
		if l == 0
		{
			None
		}
		else
		{
			let mut ret = self.get(0);
			let mut ret_i = 0;
			for i in range(1, l)
			{
				unsafe
				{
					let c = self.unsafe_get(i);
					if c > ret
					{
						ret = c;
						ret_i = i;
					}
				}
			}
			Some((ret_i, ret))
		}
	}
}
