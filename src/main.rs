extern crate itertools;
extern crate ordered_float;

use itertools::Itertools;
use ordered_float::OrderedFloat;

use std::collections::{HashMap, BTreeMap};

type Timestamp = u32;
type Value = u32;

type Scalar = (Timestamp, Value);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MatrixItem {
    metric: BTreeMap<String, String>,
    values: Vec<Scalar>,
}

impl MatrixItem {
    pub fn new(values: Vec<Scalar>) -> MatrixItem {
        MatrixItem {
            metric: BTreeMap::new(),
            values: values,
        }
    }
}

fn main() {

    let ra1: Vec<Scalar> = vec![(1, 10), (1, 12), (2, 32)];

    let ra2: Vec<Scalar> = vec![(1, 55), (1, 66), (6, 9)];

    let ra3: Vec<Scalar> = vec![(3, 98), (2, 23), (5, 48), (7, 89), (8, 91)];

    let cr = vec![
        MatrixItem::new(ra1.to_vec()),
        MatrixItem::new(ra2.to_vec()),
        MatrixItem::new(ra3.to_vec()),
    ];

    let mut fms = cr.iter()
        .flat_map(|s| s.values.clone())
       //.map(|x| (OrderedFloat(x), x.1))
        .collect::<Vec<_>>();

    // Sort them and group them
    fms.sort_by(|a, b| Ord::cmp(&a, &b));

    let mut metgroups_map = HashMap::<&u32, u32>::new();

    for (metkey, metvalues_group) in &fms.iter().group_by(|fm| &fm.0) {
        let aggregate: u32 = metvalues_group.map(|x| x.1).sum();

        // assign a aggregate
        metgroups_map.entry(metkey).or_insert_with(|| aggregate);
    }

    println!("{:?}", metgroups_map);
}
