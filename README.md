# metgroup

A sample `group_by` aggregation of metrics in rust lang.

The easiest option is to use crate `itertools`. There are still issues with `f64` not having `Eq Hash` trait. 

We'll have to use `OrderedFloat` as our metric data has `f64`.
