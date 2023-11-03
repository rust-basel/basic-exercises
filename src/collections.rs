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

    #[test]
    fn get_the_amount_of_items() {
        // given
        let collection = vec![4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 14, 4, 4, 4, 4, 4, 4, 4, 3, 4, 4, 4, 4, 4, 4, 42, 2, 2, 2, 2, 2, 2, 33, 4];

        // when (use the collection above!)
        let amount = 0;

        // then
        assert_eq!(amount, 36);
    }
}