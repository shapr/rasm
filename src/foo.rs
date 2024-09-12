use itertools;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = vec![];
    for f in factors {
        // hypothesis: rust magic deref or ref can only go ONE STEP AWAY
        // and we were able to get it to &&u32 + &&u32 which is two steps away from u32 + u32 !
        let many_vals: Vec<_> = itertools::iterate(*f, |v| f + *v)
            .take_while(|&v| v < limit)
            .collect();
        // let legal_vals: Vec<_> = many_vals..take_while(|&v| v < &limit).collect();
        multiples.extend(many_vals);
    }
    multiples.sort();
    multiples.dedup();
    multiples.iter().sum()
    // todo!("Sum the multiples of all of {factors:?} which are less than {limit}")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_sum_of_multiples() {
        assert_eq!(sum_of_multiples(20, &[3, 5]), 78);
    }
}
