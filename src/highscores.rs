#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
	todo!("Construct a HighScores struct, given the scores: {scores:?}")
    }

    pub fn scores(&self) -> &[u32] {
	todo!("Return all the scores as a slice")
    }

    pub fn latest(&self) -> Option<u32> {
	todo!("Return the latest (last) score")
    }

    pub fn personal_best(&self) -> Option<u32> {
	// test whether
	// BAD *self.scores.is_empty()
	// GOOD (*self).scores.is_empty()
	// GOOD (*self.scores).is_empty()
	// XXX understand why?
	if (*self.scores).is_empty() {
	    None
	} else {
	    // why do I have to deref this?
	    // oh! because the argument is &self?
	    // in that case, why no deref in the if ?
	    // Some(*self.scores.iter().max().unwrap())
	    let bop = *self.scores.iter().max().unwrap();
	    Some(bop)
	}
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
	todo!("Return 3 highest scores")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn list_of_scores() {
	let expected = [30, 50, 20, 70];
	let high_scores = HighScores::new(&expected);
	assert_eq!(high_scores.scores(), &expected);
    }
}
