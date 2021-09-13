use std::cmp;

/// Make it easy to use Ord on a key, while keeping a value that doesn't implement Ord
#[derive(Debug)]
pub struct OrdKey<K, V> {
	pub key: K,
	pub value: V,
}

impl<K, V> PartialEq for OrdKey<K, V>
where
	K: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.key.eq(&other.key)
	}
}

impl<K, V> Eq for OrdKey<K, V> where K: Eq {}

impl<K, V> PartialOrd for OrdKey<K, V>
where
	K: PartialOrd,
{
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		self.key.partial_cmp(&other.key)
	}
}

impl<K, V> Ord for OrdKey<K, V>
where
	K: Ord,
{
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.key.cmp(&other.key)
	}
}
