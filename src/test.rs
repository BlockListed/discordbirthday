#[cfg(test)]
mod tests {
    #[test]
    fn test_id_generation() {
        use crate::utils;
        let id = utils::gen_id();
        println!("Generated Id: {id}");
        assert_eq!(id.len(), 21);
        // Test if there is something causing it to always generate the same id.
        assert_ne!(id, utils::gen_id());
    }
}
