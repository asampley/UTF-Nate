use itertools::Itertools;

use rand::{seq::SliceRandom, thread_rng};

use crate::util::Response;

#[tracing::instrument(level = "info", ret)]
pub async fn shuffle() -> Result<Response, Response> {
	let mut rng = thread_rng();
	let mut cards = [
		"A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
	]
	.into_iter()
	.cartesian_product(["♠", "♥", "♦", "♣"])
	.map(|(c, s)| format!("||`{:>2}{}`||", c, s))
	.collect::<Vec<_>>();

	cards[..].shuffle(&mut rng);

	Ok(cards.chunks(4).map(|c| c.join(" ")).join("\n").into())
}
