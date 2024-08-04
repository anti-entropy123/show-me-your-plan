use askama::Template;
use axum::{
    extract,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use charming::HtmlRenderer;
use how_me_your_plan::CHARTS;

#[tokio::main]
async fn main() {
    println!("listen on: http://127.0.0.1:5555");
    let app = Router::new().route("/", get(render));

    axum::Server::bind(&"127.0.0.1:5555".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> impl IntoResponse {
    let mut template = IndexTemplate::new();
    for (key, value) in CHARTS.iter() {
        template.collection(key, value.iter().map(|(k, _)| *k).collect::<Vec<_>>());
    }
    HtmlTemplate(template)
}

async fn render() -> impl IntoResponse {
    let typ = "heatmap";
    let name = "heatmap_as_weekday";
    let renderer = HtmlRenderer::new(name, 1600, 800);

    let chart = match CHARTS.get(typ) {
        Some(charts) => match charts.get(name) {
            Some(chart) => chart(),
            None => return (StatusCode::NOT_FOUND, "Chart Not Found").into_response(),
        },
        None => return (StatusCode::NOT_FOUND, "Chart Type Not Found").into_response(),
    };
    Html(renderer.render(&chart).unwrap()).into_response()
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    collections: Vec<(String, Vec<String>)>,
}

impl IndexTemplate {
    fn new() -> Self {
        Self {
            collections: vec![],
        }
    }

    fn collection(&mut self, name: &str, charts: Vec<&str>) {
        self.collections.push((
            name.to_string(),
            charts.into_iter().map(|s| s.to_string()).collect(),
        ));
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
