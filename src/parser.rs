use std::ops::RangeInclusive;

use nom::{
	branch::alt,
	character::complete::{char as cchar, u32 as cu32},
	combinator::{all_consuming, map},
	multi::separated_list0,
	sequence::separated_pair,
	IResult, ToUsize,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NumOrRange<T> {
	Num(T),
	Range(RangeInclusive<T>),
}

impl<T> From<T> for NumOrRange<T> {
	fn from(f: T) -> Self {
		Self::Num(f)
	}
}

impl<T> From<RangeInclusive<T>> for NumOrRange<T> {
	fn from(f: RangeInclusive<T>) -> Self {
		Self::Range(f)
	}
}

pub fn cusize(input: &str) -> IResult<&str, usize> {
	map(cu32, |v| v.to_usize())(input)
}

pub fn range_usize(input: &str) -> IResult<&str, RangeInclusive<usize>> {
	map(separated_pair(cusize, cchar('-'), cusize), |(a, b)| a..=b)(input)
}

pub fn set(input: &str) -> IResult<&str, Vec<NumOrRange<usize>>> {
	all_consuming(separated_list0(
		cchar(','),
		alt((map(range_usize, |v| v.into()), map(cusize, |v| v.into()))),
	))(input)
}
