fn sumup_items(items: &[u32]) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::collections::sumup_items;

    #[test]
    fn sum_of_all_items_in_collection() {
        // given
        let collection_of_items = vec![1, 5, 6, 3, 4, 6, 3, 4, 5, 6, 7, 7, 123, 1, 3, 5, 23, 4, 234, 234234, 5, 23, 4, 2, 3, 4442];

        // when
        let sum = sumup_items(&collection_of_items);

        // then
        assert_eq!(sum, 239163);
    }
}