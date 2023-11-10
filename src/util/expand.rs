pub fn expand<T>(collection: &mut Vec<T>, index: usize)
where
    T: Default,
{
    if collection.len() <= index {
        collection.resize_with(index + 1, Default::default);
    }
}
