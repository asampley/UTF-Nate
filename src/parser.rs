//! Parse custom data types from command arguments.
//!
//! In order to work with [`poise`], [`std::str::FromStr`] is implemented on
//! types that should be taken from arguments.

use serde::{Deserialize, Serialize};

use thiserror::Error;

use std::ops::RangeInclusive;

use nom::{
	branch::alt,
	character::complete::{char as cchar, multispace0, u64 as cu64},
	combinator::{all_consuming, map},
	multi::separated_list0,
	sequence::{delimited, separated_pair},
	Finish, IResult, ToUsize,
};

/// Represents a selection of several ranges of values.
#[derive(Clone, Debug, Deserialize, Serialize)]
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

impl<T> FromIterator<NumOrRange<T>> for Selection<T> {
	fn from_iter<I: IntoIterator<Item = NumOrRange<T>>>(iter: I) -> Self {
		Self(iter.into_iter().collect())
	}
}

impl<T> From<RangeInclusive<T>> for Selection<T> {
	fn from(v: RangeInclusive<T>) -> Self {
		Self::from_iter(std::iter::once(NumOrRange::from(v)))
	}
}

impl<T> IntoIterator for Selection<T>
where
	RangeInclusive<T>: Iterator<Item = T>,
	T: Copy,
{
	type Item = T;
	type IntoIter = std::iter::Flatten<std::vec::IntoIter<NumOrRange<T>>>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter().flatten()
	}
}

/// Represents the two possibilities of either a scalar or range of values.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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

impl<T> IntoIterator for NumOrRange<T>
where
	RangeInclusive<T>: Iterator<Item = T>,
	T: Copy,
{
	type Item = T;
	type IntoIter = RangeInclusive<T>;

	fn into_iter(self) -> Self::IntoIter {
		match self {
			NumOrRange::Range(r) => r,
			NumOrRange::Num(n) => n..=n,
		}
	}
}

/// [`nom`] style parser for turning a u64 into a usize.
pub fn cusize(input: &str) -> IResult<&str, usize> {
	map(cu64, |v| v.to_usize())(input)
}

/// [`nom`] style parser for turning "`usize`-`usize`" into a
/// [`RangeInclusive<usize>`].
pub fn range_usize(input: &str) -> IResult<&str, RangeInclusive<usize>> {
	map(
		separated_pair(
			cusize,
			delimited(multispace0, cchar('-'), multispace0),
			cusize,
		),
		|(a, b)| a..=b,
	)(input)
}

/// [`nom`] style parser for parsing either a range or num. See [`cusize`] and
/// [`range_usize`].
pub fn num_or_range_usize(input: &str) -> IResult<&str, NumOrRange<usize>> {
	delimited(
		multispace0,
		alt((map(range_usize, |v| v.into()), map(cusize, |v| v.into()))),
		multispace0,
	)(input)
}

/// [`nom`] style parser for getting a list of nums and ranges separated by
/// commas. Must consume the entire input.
pub fn selection(input: &str) -> IResult<&str, Selection<usize>> {
	all_consuming(map(separated_list0(cchar(','), num_or_range_usize), |v| {
		v.into()
	}))(input)
}

#[cfg(test)]
mod test {
	use itertools::Itertools;

	use super::*;

	#[test]
	fn selection_num() {
		for num in [usize::MIN, 1, 20, usize::MAX] {
			let s = num.to_string();

			let sel = selection(&s).expect("Error parsing");

			assert_eq!(vec![NumOrRange::Num(num)], sel.1 .0)
		}
	}

	#[test]
	fn selection_range() {
		for range in [0..=0, 0..=10, 3..=5, usize::MIN..=usize::MAX] {
			for left_space in ["", " "] {
				for right_space in ["", " "] {
					let s = format!(
						"{}{}-{}{}",
						range.start(),
						left_space,
						right_space,
						range.end()
					);

					let sel = selection(&s).expect("Error parsing");

					assert_eq!(vec![NumOrRange::Range(range.clone())], sel.1 .0)
				}
			}
		}
	}

	#[test]
	fn selection_list() {
		let target = [
			NumOrRange::Num(usize::MIN),
			NumOrRange::Range(6..=7),
			NumOrRange::Num(usize::MAX),
			NumOrRange::Range(usize::MAX..=usize::MAX),
			NumOrRange::Num(5),
		];

		["", " "]
			.into_iter()
			.cartesian_product(["", " "])
			.map(|(a, b)| {
				target
					.iter()
					.map(|t| {
						(
							a,
							match t {
								NumOrRange::Range(range) => {
									format!("{}-{}", range.start(), range.end())
								}
								NumOrRange::Num(num) => num.to_string(),
							},
							b,
						)
					})
					.map(|(a, t, b)| format!("{}{}{}", a, t, b))
					.join(",")
			})
			.for_each(|s| {
				let sel = selection(&s).expect("Error parsing");

				assert_eq!(target, sel.1 .0[..])
			});
	}
}
