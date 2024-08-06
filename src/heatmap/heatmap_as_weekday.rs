use charming::{
    component::{Axis, Grid, VisualMap},
    datatype::{CompositeValue, DataFrame},
    df,
    element::{AxisLabel, AxisType, Emphasis, ItemStyle, Label, Orient, SplitArea},
    series::Heatmap,
    Chart,
};

use crate::PlanData;

pub fn chart(mut plan_data: Vec<PlanData>) -> Chart {
    plan_data.reverse();

    let mut data = vec![];
    for (idx, item) in plan_data.iter().enumerate() {
        data.extend(get_data(
            idx as i32,
            item.time_period.0,
            item.time_period.1,
            item.val,
        ));
    }

    Chart::new()
        // .tooltip(Tooltip::new().position("top"))
        .grid(
            Grid::new()
                .top(60)
                .height(60 * plan_data.len() as i64)
                .width("72%")
                .left("14%")
                .bottom(60), // .right("2%"),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(get_x_axis())
                .axis_label(AxisLabel::new().interval(7))
                .split_area(SplitArea::new().show(true))
                .position("top"),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(
                    plan_data
                        .iter()
                        .map(|item| item.name.clone())
                        .collect::<Vec<String>>(),
                )
                .split_area(SplitArea::new().show(true)),
        )
        .visual_map(
            VisualMap::new()
                .min(0)
                .max(10000)
                .calculable(true)
                .orient(Orient::Horizontal)
                .left("center")
                // .top(30)
                // .bottom(60),
        )
        .series(
            Heatmap::new()
                .name("Punch Card")
                .label(Label::new().show(false))
                .emphasis(
                    Emphasis::new().item_style(
                        ItemStyle::new()
                            .shadow_blur(10)
                            .shadow_color("rgba(0, 0, 0, 0.5)"),
                    ),
                )
                .data(data),
        )
}

fn get_data(
    item_idx: i32,
    start_at: (i32, i32),
    end_at: (i32, i32),
    mut val: i32,
) -> Vec<DataFrame> {
    let mut data: Vec<Vec<i32>> = vec![];
    let mut day = start_at.0;
    let mut hour = start_at.1;
    if val == 0 {
        val = 1
    }

    while !(day == end_at.0 && hour == end_at.1) {
        data.push(vec![day * 24 + hour, item_idx, val]);
        hour += 1;
        if hour == 24 {
            day += 1;
            hour = 0;
        }
        if day == 7 {
            day = 0;
        }
    }

    data.into_iter()
        .map(|d| {
            df![
                d[0],
                d[1],
                if d[2] == 0 {
                    CompositeValue::from("-")
                } else {
                    CompositeValue::from(d[2])
                }
            ]
        })
        .collect()
}

fn get_x_axis() -> Vec<String> {
    let mut result = vec![];
    for day in [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ] {
        for hour in 0..24 {
            result.push(if hour == 0 {
                day.to_owned()
            } else {
                hour.to_string()
            })
        }
    }
    result
}
