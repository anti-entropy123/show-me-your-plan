use askama::Template;
use axum::{
    extract,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use charming::HtmlRenderer;
use serde::{Deserialize, Serialize};
use show_me_your_plan::{PlanData, CHARTS};

#[tokio::main]
async fn main() {
    println!("listen on: http://127.0.0.1:5555");
    let app = Router::new().route("/", get(index)).route("/", post(index));

    axum::Server::bind(&"127.0.0.1:5555".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize)]
struct MyForm {
    #[serde(rename = "textInput")]
    text_input: String,
}

#[axum::debug_handler]
async fn index(body: Option<extract::Form<MyForm>>) -> impl IntoResponse {
    let default_plan_data = vec![];

    let plan_data: Vec<PlanData> = if let Some(body) = body {
        serde_json::from_str(&body.text_input).unwrap_or(default_plan_data)
    } else {
        default_plan_data
    };
    println!("{:?}", plan_data);

    let mut template = IndexTemplate::new();

    let renderer = HtmlRenderer::new(
        "heatmap_as_weekday",
        1600,
        120 + 60 * plan_data.len() as u64,
    );
    template.render_content = renderer
        .render(&show_me_your_plan::heatmap::heatmap_as_weekday::chart(
            plan_data.clone(),
        ))
        .unwrap();
    template.height = 150 + 60 * plan_data.len();

    let plan_data = if plan_data.is_empty() {
        vec![PlanData::default()]
    } else {
        plan_data
    };
    template.plan_data = serde_json::to_string_pretty(&plan_data).unwrap();

    HtmlTemplate(template)
}

fn render() -> String {
    let typ = "heatmap";
    let name = "heatmap_as_weekday";
    let renderer = HtmlRenderer::new(name, 1600, 800);

    let chart = match CHARTS.get(typ) {
        Some(charts) => match charts.get(name) {
            Some(chart) => chart(vec![PlanData {
                time_period: ((1, 10), (5, 9)),
                val: 10,
                name: "Demo".to_owned(),
            }]),
            None => return String::new(),
        },
        None => return String::new(),
    };
    renderer.render(&chart).unwrap()
}

#[derive(Template)]
#[template(path = "my_index.html")]
struct IndexTemplate {
    render_content: String,
    plan_data: String,
    height: usize,
}

impl IndexTemplate {
    fn new() -> Self {
        Self {
            plan_data: String::new(),
            render_content: String::new(),
            height: 400,
        }
    }
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(body) => Html(body).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Template error: {}", e),
            )
                .into_response(),
        }
    }
}
