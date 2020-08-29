pub mod chunk_random;
pub mod java_random;

#[cfg(test)]
mod tests {
    use super::chunk_random::*;

    #[test]
    fn test_random() {
        let mut cr = ChunkRandom::default();
        cr.get_random(94824734);
        assert_eq!(cr.get_seed_state(), 25153900403);
        cr.rand.next_long();
        assert_eq!(cr.get_seed_state(), 135493623266021);
    }
}
