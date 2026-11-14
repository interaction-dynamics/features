//! Specs and decision records of each feature, see [`crate::records`].

use std::path::Path;

use super::for_each_feature_mut;
use crate::models::Feature;
use crate::records::{
    read_feature_records, read_root_records, resolve_links, resolve_links_with, sort_records,
    split_root_records,
};

/// Fill in each feature's `records` from its own documentation folders and
/// from the root records that name it in their `feature:` frontmatter
pub(super) fn populate_records(features: &mut [Feature], base_path: &Path) {
    let (mut attached, mut project) = split_root_records(read_root_records(base_path), features);
    resolve_links(&mut project);

    for_each_feature_mut(features, &mut |feature| {
        let mut records = read_feature_records(&base_path.join(&feature.path), base_path);
        let mut index = 0;
        while index < attached.len() {
            if attached[index].0 == feature.path {
                records.push(attached.remove(index).1);
            } else {
                index += 1;
            }
        }
        resolve_links_with(&mut records, &project);
        sort_records(&mut records);
        feature.records = records;
    });
}
