use crate::*;

pub fn apply_pagination<T>(items: Vec<T>, from_index: Option<usize>, limit: Option<usize>) -> Vec<T> {
    let start = from_index.unwrap_or(0);
    let limit = limit.unwrap_or_else(|| items.len());
    let end = (start + limit).min(items.len()); // Ensure we don't go out of bounds
    items.into_iter().skip(start).take(end - start).collect()
}