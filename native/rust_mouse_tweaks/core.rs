pub fn desired(enabled: bool, original: bool) -> bool {
    enabled || original
}
#[cfg(test)]
mod tests {
    #[test]
    fn disabling_preserves_original_preference() {
        assert!(super::desired(false, true));
        assert!(!super::desired(false, false));
        assert!(super::desired(true, false));
    }
}
