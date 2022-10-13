//! Parse custom data types from command arguments.
//!
//! In order to work with [`poise`], [`std::str::FromStr`] is implemented on
//! types that should be taken from arguments.

use thiserror::Error;

use std::ops::RangeInclusive;

use nom::{
	branch::alt,
	character::complete::{char as cchar, u32 as cu32},
	combinator::{all_consuming, map},
	multi::separated_list0,
	sequence::separated_pair,
	Finish, IResult, ToUsize,
};

/// Represents a selection of several ranges of values.
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Selection<T>(pub Vec<NumOrRange<T>>);

/// Error representing an error while parsing a [`Selection`].
#[derive(Debug, Error)]
#[error("expected a list of integers or ranges (e.g. 1,2-4), separated by commas")]
pub struct ParseSelectionError(#[from] nom::error::Error<String>);

impl<T> From<Vec<NumOrRange<T>>> for Selection<T> {
	fn from(f: Vec<NumOrRange<T>>) -> Self {
		Self(f)
	}
}

/// Represents the two possibilities of either a scalar or range of values.
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

impl std::str::FromStr for Selection<usize> {
	type Err = ParseSelectionError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		selection(s).finish().map(|v| v.1).map_err(|e| {
			nom::error::Error {
				input: e.input.to_string(),
				code: e.code,
			}
			.into()
		})
	}
}

/// [`nom`] style parser for turning a u32 into a usize.
pub fn cusize(input: &str) -> IResult<&str, usize> {
	map(cu32, |v| v.to_usize())(input)
}

/// [`nom`] style parser for turning "`usize`-`usize`" into a
/// [`RangeInclusive<usize>`].
pub fn range_usize(input: &str) -> IResult<&str, RangeInclusive<usize>> {
	map(separated_pair(cusize, cchar('-'), cusize), |(a, b)| a..=b)(input)
}

/// [`nom`] style parser for parsing either a range or num. See [`cusize`] and
/// [`range_usize`].
pub fn num_or_range_usize(input: &str) -> IResult<&str, NumOrRange<usize>> {
	alt((map(range_usize, |v| v.into()), map(cusize, |v| v.into())))(input)
}

/// [`nom`] style parser for getting a list of nums and ranges separated by
/// commas. Must consume the entire input.
pub fn selection(input: &str) -> IResult<&str, Selection<usize>> {
	all_consuming(map(separated_list0(cchar(','), num_or_range_usize), |v| {
		v.into()
	}))(input)
}
