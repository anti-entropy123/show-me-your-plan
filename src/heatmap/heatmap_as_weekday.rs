use charming::{
    component::{Axis, Grid},
    datatype::{CompositeValue, DataFrame},
    df,
    element::{AxisLabel, AxisType, Emphasis, ItemStyle, Label, SplitArea},
    series::Heatmap,
    Chart,
};

pub fn chart() -> Chart {
    let mut data = vec![];
    data.extend(get_data(0, (2, 10), (4, 8)));
    data.extend(get_data(1, (6, 16), (1, 22)));
    // println!("{:?}", data);

    Chart::new()
        // .tooltip(Tooltip::new().position("top"))
        .grid(
            Grid::new()
                .height("10%")
                .top("5%")
                .width("72%")
                .left("14%"), // .right("2%"),
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
                .data(vec!["planA", "planB"])
                .split_area(SplitArea::new().show(true)),
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

fn get_data(item_idx: i32, start_at: (i32, i32), end_at: (i32, i32)) -> Vec<DataFrame> {
    let mut data: Vec<Vec<i32>> = vec![];
    let mut day = start_at.0;
    let mut hour = start_at.1;
    while !(day == end_at.0 && hour == end_at.1) {
        data.push(vec![day * 24 + hour, item_idx, 1]);
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
