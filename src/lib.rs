mod heatmap;

use std::collections::BTreeMap;

use charming::Chart;
use lazy_static::lazy_static;

macro_rules! insert {
    ($map:ident, $type:ident, $name:ident) => {
        $map.insert(stringify!($name), $type::$name::chart as fn() -> Chart);
    };
}

lazy_static! {
    static ref HEATMAP_CHARTS: BTreeMap<&'static str, fn() -> Chart> = {
        let mut m = BTreeMap::new();
        insert!(m, heatmap, heatmap_as_weekday);
        m
    };
    pub static ref CHARTS: BTreeMap<&'static str, BTreeMap<&'static str, fn() -> Chart>> = {
        let mut m = BTreeMap::new();

        m.insert("heatmap", HEATMAP_CHARTS.clone());

        m
    };
}
