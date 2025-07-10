pub struct DetectResult<T> {
    pub create: Vec<T>,
    pub delete: Vec<T>
}

pub fn detect_changes_in_vecs<T: PartialEq + Eq + std::hash::Hash + Clone>(old: Vec<T>, new: Vec<T>) -> DetectResult<T> {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    let old: HashSet<&T, std::hash::RandomState> = HashSet::from_iter(old.iter());
    let new: HashSet<&T, std::hash::RandomState> = HashSet::from_iter(new.iter());
    
    let mut merged: HashSet<&T, std::hash::RandomState> = old.clone();
    merged.extend(&new);
    let merged = merged;

    let mut to_add: HashSet<&T, std::hash::RandomState> = merged.clone();
    let mut to_delete: HashSet<&T, std::hash::RandomState> = HashSet::new();

    for item in merged {
        let old_contains = old.contains(item);
        let new_contains = new.contains(item);
        
        // not found in new
        if old_contains && !new_contains {
            // move to to_delete 
            to_add.remove(item);
            to_delete.insert(item);

            continue;
        }

        // found in both
        if old_contains && new_contains {
            // delete from to_add
            to_add.remove(item);
        }

        // else do nothing
    }

    let create: Vec<T> = to_add
        .into_iter()
        .cloned()
        .collect();
    
    let delete: Vec<T> = to_delete
        .into_iter()
        .cloned()
        .collect();

    return DetectResult { create, delete };
}
