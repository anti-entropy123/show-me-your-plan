pub mod heatmap;

use std::collections::BTreeMap;

use charming::Chart;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

macro_rules! insert {
    ($map:ident, $type:ident, $name:ident) => {
        $map.insert(
            stringify!($name),
            $type::$name::chart as fn(Vec<PlanData>) -> Chart,
        );
    };
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct PlanData {
    pub time_period: ((i32, i32), (i32, i32)),
    pub val: i32,
    pub name: String,
}

lazy_static! {
    static ref HEATMAP_CHARTS: BTreeMap<&'static str, fn(Vec<PlanData>) -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, heatmap, heatmap_as_weekday);
        m
    };
    pub static ref CHARTS: BTreeMap<&'static str, BTreeMap<&'static str, fn(Vec<PlanData>) -> Chart>> = {
        let mut m = BTreeMap::new();

        m.insert("heatmap", HEATMAP_CHARTS.clone());

        m
    };
}
