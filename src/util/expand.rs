pub fn expand<T>(collection: &mut Vec<T>, index: usize)
where
    T: Default,
{
    if collection.len() <= index {
        collection.resize_with(index + 1, Default::default);
    }
}

pub fn expand_with<T>(collection: &mut Vec<T>, index: usize, value: T)
where
    T: Copy,
{
    if collection.len() <= index {
        collection.resize_with(index + 1, || value);
    }
}
