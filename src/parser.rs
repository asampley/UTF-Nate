use std::ops::RangeInclusive;

use nom::{
	IResult,
	ToUsize,
	branch::alt,
	character::complete::{u32 as cu32, char as cchar},
	combinator::{all_consuming, map},
	multi::separated_list0,
	sequence::separated_pair,
};

pub enum NumOrRange {
	Num(usize),
	Range(RangeInclusive<usize>)
}

impl From<usize> for NumOrRange {
	fn from(f: usize) -> Self {
		Self::Num(f)
	}
}

impl From<RangeInclusive<usize>> for NumOrRange {
	fn from(f: RangeInclusive<usize>) -> Self {
		Self::Range(f)
	}
}

pub fn cusize(input: &str) -> IResult<&str, usize> {
	map(cu32, |v| v.to_usize())(input)
}

pub fn range_usize(input: &str) -> IResult<&str, RangeInclusive<usize>> {
	map(separated_pair(cusize, cchar('-'), cusize), |(a, b)| a..=b)(input)
}

pub fn set(input: &str) -> IResult<&str, Vec<NumOrRange>> {
	all_consuming(
		separated_list0(
			cchar(','),
			alt((
				map(range_usize, |v| v.into()),
				map(cusize, |v| v.into()),
			))
		)
	)(input)
}
