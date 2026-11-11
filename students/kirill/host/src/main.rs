mod decimal;
mod document;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{
        Html, IntoResponse, Response,
        sse::{Event, KeepAlive, Sse},
    },
    routing::get,
};
use document::{Analysis, Document, Game, Player, Snapshot};
use serde::Deserialize;
use std::{
    env, io,
    path::{Path as FilePath, PathBuf},
    sync::Arc,
};
use tokio::{
    fs,
    net::TcpListener,
    sync::{Mutex, watch},
};
use tokio_stream::{Stream, StreamExt, wrappers::WatchStream};
use utoipa::{OpenApi, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

const ABOUT: &str = "\
Holds one game document in the nash.md schema and shows it live at `/`.

A write replaces one whole entity and answers 204.
A rejected write answers with a plain-text reason and changes nothing.

- Text is plain text, not markdown; the markdown export escapes it.
- Every text field is one line; a `summary` is one paragraph.
- Player, game and move names are unique and not blank.
- A name in a path is percent-encoded; a body may repeat that name but must not contradict it.
- The server derives welfare, gap and status of each analysis; read them from `/api/markdown`.";

#[tokio::main]
async fn main() -> io::Result<()> {
    let path = env::args_os().nth(1).map(PathBuf::from);
    let address = env::var("NASH_HOST_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_owned());
    match &path {
        Some(path) => println!("state: {}", path.display()),
        None => println!("state: in memory, lost on exit"),
    }
    let host = Host::open(path).await?;
    let listener = TcpListener::bind(address).await?;
    println!("http://{}", listener.local_addr()?);
    axum::serve(listener, router(host)).await
}

#[derive(OpenApi)]
#[openapi(info(description = ABOUT))]
struct Api;

#[derive(Clone)]
struct Host(Arc<Shared>);

struct Shared {
    snapshot: watch::Sender<Snapshot>,
    file: Mutex<Option<PathBuf>>,
}

impl Host {
    async fn open(path: Option<PathBuf>) -> io::Result<Self> {
        let document = match &path {
            Some(path) => match fs::read(path).await {
                Ok(json) => serde_json::from_slice::<Document>(&json)?,
                Err(error) if error.kind() == io::ErrorKind::NotFound => Document::default(),
                Err(error) => return Err(error),
            },
            None => Document::default(),
        };
        Self::new(document, path)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))
    }

    fn new(document: Document, path: Option<PathBuf>) -> Result<Self, document::Error> {
        Ok(Self(Arc::new(Shared {
            snapshot: watch::Sender::new(Snapshot::new(document)?),
            file: Mutex::new(path),
        })))
    }

    async fn update(
        &self,
        change: impl FnOnce(&mut Document) -> Result<(), document::Error> + Send,
    ) -> Result<StatusCode, Failure> {
        let file = self.0.file.lock().await;
        let mut document = self.0.snapshot.borrow().document().clone();
        change(&mut document)?;
        let snapshot = Snapshot::new(document)?;
        if let Some(path) = file.as_deref() {
            save(path, snapshot.document()).await?;
        }
        self.0.snapshot.send_replace(snapshot);
        Ok(StatusCode::NO_CONTENT)
    }
}

#[derive(Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
struct Overview {
    title: String,
    summary: String,
}

struct Failure(StatusCode, String);

impl From<document::Error> for Failure {
    fn from(error: document::Error) -> Self {
        let status = match error {
            document::Error::NotFound(_) => StatusCode::NOT_FOUND,
            document::Error::Invalid(_) => StatusCode::UNPROCESSABLE_ENTITY,
        };
        Self(status, error.to_string())
    }
}

impl From<io::Error> for Failure {
    fn from(error: io::Error) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    }
}

impl IntoResponse for Failure {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}

fn router(host: Host) -> Router {
    let (router, api) = OpenApiRouter::with_openapi(Api::openapi())
        .routes(routes!(get_document, put_document))
        .routes(routes!(markdown))
        .routes(routes!(events))
        .routes(routes!(put_overview))
        .routes(routes!(put_player, remove_player))
        .routes(routes!(put_game, remove_game))
        .routes(routes!(put_analysis, remove_analysis))
        .with_state(host)
        .split_for_parts();
    router
        .route("/", get(index))
        .merge(SwaggerUi::new("/docs").url("/api/openapi.json", api))
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../assets/index.html"))
}

#[utoipa::path(
    get,
    path = "/api/document",
    tag = "document",
    summary = "Read the document as JSON",
    responses((status = 200, description = "The document", body = Document))
)]
async fn get_document(State(host): State<Host>) -> Json<Document> {
    Json(host.0.snapshot.borrow().document().clone())
}

#[utoipa::path(
    put,
    path = "/api/document",
    tag = "document",
    summary = "Replace the whole document",
    description = "Drops every player, game and analysis that the body does not list.",
    request_body = Document,
    responses(
        (status = 204, description = "Stored"),
        (status = 422, description = "Rejected; the body states the broken rule", body = String),
    )
)]
async fn put_document(
    State(host): State<Host>,
    Json(replacement): Json<Document>,
) -> Result<StatusCode, Failure> {
    host.update(|document| {
        *document = replacement;
        Ok(())
    })
    .await
}

#[utoipa::path(
    get,
    path = "/api/markdown",
    tag = "document",
    summary = "Read the document as markdown in the nash.md schema",
    description = "Includes the derived Payoff Analysis section: welfare, gap and status of each game.",
    responses((status = 200, description = "The document", body = String, content_type = "text/plain"))
)]
async fn markdown(State(host): State<Host>) -> String {
    host.0.snapshot.borrow().markdown()
}

#[utoipa::path(
    get,
    path = "/api/events",
    tag = "document",
    summary = "Stream the document and its derived payoff rows",
    description = "Server-sent events: one frame on connect and one per change, each a full snapshot.",
    responses((status = 200, description = "Snapshots", body = Snapshot, content_type = "text/event-stream"))
)]
async fn events(State(host): State<Host>) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let snapshots = WatchStream::new(host.0.snapshot.subscribe());
    Sse::new(snapshots.map(|snapshot| Event::default().json_data(snapshot)))
        .keep_alive(KeepAlive::default())
}

#[utoipa::path(
    put,
    path = "/api/overview",
    tag = "document",
    summary = "Set the title and the summary paragraph",
    request_body = Overview,
    responses(
        (status = 204, description = "Stored"),
        (status = 422, description = "Rejected; the body states the broken rule", body = String),
    )
)]
async fn put_overview(
    State(host): State<Host>,
    Json(overview): Json<Overview>,
) -> Result<StatusCode, Failure> {
    host.update(|document| {
        document.title = overview.title;
        document.summary = overview.summary;
        Ok(())
    })
    .await
}

#[utoipa::path(
    put,
    path = "/api/players/{name}",
    tag = "players",
    summary = "Add or replace a player",
    description = "Send every table in full; an empty list clears that table.",
    params(("name" = String, Path, description = "Player name")),
    request_body = Player,
    responses(
        (status = 204, description = "Stored"),
        (status = 422, description = "Rejected; the body states the broken rule", body = String),
    )
)]
async fn put_player(
    State(host): State<Host>,
    Path(name): Path<String>,
    Json(player): Json<Player>,
) -> Result<StatusCode, Failure> {
    let name = keyed(name, player.name)?;
    host.update(|document| {
        document.put_player(Player { name, ..player });
        Ok(())
    })
    .await
}

#[utoipa::path(
    delete,
    path = "/api/players/{name}",
    tag = "players",
    summary = "Remove a player",
    description = "Games and analyses that mention the player by name keep that text.",
    params(("name" = String, Path, description = "Player name")),
    responses(
        (status = 204, description = "Removed"),
        (status = 404, description = "No such player", body = String),
    )
)]
async fn remove_player(
    State(host): State<Host>,
    Path(name): Path<String>,
) -> Result<StatusCode, Failure> {
    host.update(|document| document.remove_player(&name)).await
}

#[utoipa::path(
    put,
    path = "/api/games/{name}",
    tag = "games",
    summary = "Add or replace a game",
    description = "Keeps the payoff analysis of the game as it is.",
    params(("name" = String, Path, description = "Game name")),
    request_body = Game,
    responses(
        (status = 204, description = "Stored"),
        (status = 422, description = "Rejected; the body states the broken rule", body = String),
    )
)]
async fn put_game(
    State(host): State<Host>,
    Path(name): Path<String>,
    Json(game): Json<Game>,
) -> Result<StatusCode, Failure> {
    let name = keyed(name, game.name)?;
    host.update(|document| {
        document.put_game(Game { name, ..game });
        Ok(())
    })
    .await
}

#[utoipa::path(
    delete,
    path = "/api/games/{name}",
    tag = "games",
    summary = "Remove a game and its payoff analysis",
    params(("name" = String, Path, description = "Game name")),
    responses(
        (status = 204, description = "Removed"),
        (status = 404, description = "No such game", body = String),
    )
)]
async fn remove_game(
    State(host): State<Host>,
    Path(name): Path<String>,
) -> Result<StatusCode, Failure> {
    host.update(|document| document.remove_game(&name)).await
}

#[utoipa::path(
    put,
    path = "/api/games/{name}/analysis",
    tag = "games",
    summary = "Set the payoff analysis of a game",
    params(("name" = String, Path, description = "Game name")),
    request_body = Analysis,
    responses(
        (status = 204, description = "Stored"),
        (status = 404, description = "No such game", body = String),
        (status = 422, description = "Rejected; the body states the broken rule", body = String),
    )
)]
async fn put_analysis(
    State(host): State<Host>,
    Path(name): Path<String>,
    Json(analysis): Json<Analysis>,
) -> Result<StatusCode, Failure> {
    let game = keyed(name, analysis.game)?;
    host.update(|document| document.put_analysis(Analysis { game, ..analysis }))
        .await
}

#[utoipa::path(
    delete,
    path = "/api/games/{name}/analysis",
    tag = "games",
    summary = "Remove the payoff analysis of a game",
    params(("name" = String, Path, description = "Game name")),
    responses(
        (status = 204, description = "Removed"),
        (status = 404, description = "The game has no analysis", body = String),
    )
)]
async fn remove_analysis(
    State(host): State<Host>,
    Path(name): Path<String>,
) -> Result<StatusCode, Failure> {
    host.update(|document| document.remove_analysis(&name))
        .await
}

fn keyed(path: String, body: String) -> Result<String, Failure> {
    if body.is_empty() || body == path {
        Ok(path)
    } else {
        Err(Failure(
            StatusCode::UNPROCESSABLE_ENTITY,
            format!("body names `{body}`, path names `{path}`"),
        ))
    }
}

async fn save(path: &FilePath, document: &Document) -> io::Result<()> {
    let mut json = serde_json::to_vec_pretty(document)?;
    json.push(b'\n');
    let staging = path.with_added_extension("tmp");
    fs::write(&staging, json).await?;
    fs::rename(staging, path).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{Body, to_bytes},
        http::{Method, Request, header},
    };
    use decimal::Decimal;
    use document::{Cell, Entry, Outcome, Pair, Profile, Stated};
    use pulldown_cmark::{Event as Markup, Options, Parser, Tag};
    use serde_json::{Value, json};
    use std::{mem, process};
    use tower::ServiceExt;

    const SEED: &str = include_str!("../../games/prisoners_dilemma.json");
    const REFERENCE: &str = include_str!("../../games/prisoners_dilemma.md");
    const HOSTILE: [&str; 43] = [
        "# heading",
        "> quote",
        "- bullet",
        "+ plus",
        "* star",
        "1. one",
        "2) two",
        "---",
        "===",
        "***",
        "___",
        "| --- | --- |",
        ":--",
        "```",
        "~~~ fence",
        "<div>",
        "<!-- open",
        "[ref]: http://x",
        "[^1]: note",
        "    indented",
        "\tTabbed",
        "a | b",
        "`a|b`",
        "trailing\\",
        "*",
        "-",
        "+",
        "=",
        "|",
        "what? #1 50% a/b",
        "AT&amp;T &#35; R&D",
        "$x$ :smile: @user www.example.com",
        "~~struck~~ ~one~",
        "![image](http://x/y.png)",
        "<script>alert(1)</script>",
        "- [ ] task",
        "\\newpage",
        "ratio a:b:c",
        "two trailing spaces  ",
        "trailing space ",
        "(Heads 50%, Tails 50%)",
        "Банда 囚徒",
        "plain",
    ];

    #[tokio::test]
    async fn requests_rebuild_the_reference_document() {
        let seed = serde_json::from_str::<Document>(SEED).unwrap();
        let path = env::temp_dir().join(format!("nash-host-{}.json", process::id()));
        let host = Host::open(Some(path.clone())).await.unwrap();

        let overview = json!({"title": seed.title, "summary": seed.summary});
        let mut requests = vec![("/api/overview".to_owned(), overview)];
        requests.extend(seed.players.iter().map(|player| {
            let uri = format!("/api/players/{}", encoded(&player.name));
            (uri, json!(player))
        }));
        requests.extend(
            seed.games
                .iter()
                .map(|game| (format!("/api/games/{}", encoded(&game.name)), json!(game))),
        );
        requests.extend(seed.analysis.iter().map(|analysis| {
            let uri = format!("/api/games/{}/analysis", encoded(&analysis.game));
            (uri, json!(analysis))
        }));
        for (uri, body) in requests {
            assert_eq!(
                send(&host, Method::PUT, &uri, Some(body)).await,
                (StatusCode::NO_CONTENT, String::new()),
                "{uri}"
            );
        }

        assert_eq!(
            send(&host, Method::GET, "/api/markdown", None).await,
            (StatusCode::OK, REFERENCE.to_owned())
        );
        assert_eq!(fs::read_to_string(&path).await.unwrap(), SEED);
        let reopened = Host::open(Some(path.clone())).await.unwrap();
        assert_eq!(*reopened.0.snapshot.borrow().document(), seed);
        fs::remove_file(path).await.unwrap();
    }

    #[tokio::test]
    async fn rejected_requests_leave_the_document_unchanged() {
        let seed = serde_json::from_str::<Document>(SEED).unwrap();
        let host = Host::new(seed.clone(), None).unwrap();
        let analysis = |field: &str, value: Value| {
            let mut body = json!(seed.analysis[0]);
            body[field] = value;
            Some(body)
        };
        let player = |field: &str, value: Value| {
            let mut body = json!(seed.players[0]);
            body[field] = value;
            Some(body)
        };
        let interrogation = "/api/games/Interrogation/analysis";
        let cases = [
            (
                Method::DELETE,
                "/api/players/Nobody",
                None,
                StatusCode::NOT_FOUND,
            ),
            (
                Method::PUT,
                "/api/games/Nowhere/analysis",
                analysis("game", json!("Nowhere")),
                StatusCode::NOT_FOUND,
            ),
            (
                Method::PUT,
                interrogation,
                analysis("equilibrium", json!(["Betray", "Confess"])),
                StatusCode::UNPROCESSABLE_ENTITY,
            ),
            (
                Method::PUT,
                interrogation,
                analysis("equilibrium", json!({"label": "Mixed", "welfare": -1.5})),
                StatusCode::UNPROCESSABLE_ENTITY,
            ),
            (
                Method::PUT,
                interrogation,
                analysis("equilibrium", json!({"label": "Mixed", "payoff": -3})),
                StatusCode::UNPROCESSABLE_ENTITY,
            ),
            (
                Method::PUT,
                interrogation,
                analysis("fix", json!("")),
                StatusCode::UNPROCESSABLE_ENTITY,
            ),
            (
                Method::PUT,
                interrogation,
                analysis("col_moves", json!(["Silence"])),
                StatusCode::UNPROCESSABLE_ENTITY,
            ),
            (
                Method::PUT,
                "/api/players/Prisoner%20A",
                player("name", json!("Prisoner B")),
                StatusCode::UNPROCESSABLE_ENTITY,
            ),
            (
                Method::PUT,
                "/api/players/Prisoner%20A",
                player("move", json!([])),
                StatusCode::UNPROCESSABLE_ENTITY,
            ),
            (
                Method::PUT,
                "/api/players/%2E%2E",
                player("name", json!("..")),
                StatusCode::UNPROCESSABLE_ENTITY,
            ),
            (
                Method::PUT,
                "/api/players/Prisoner%20A",
                player("summary", json!("One paragraph.\r\rAnother paragraph.")),
                StatusCode::UNPROCESSABLE_ENTITY,
            ),
        ];
        for (method, uri, body, expected) in cases {
            let (status, reason) = send(&host, method, uri, body).await;
            assert_eq!(status, expected, "{uri}: {reason}");
        }
        assert_eq!(*host.0.snapshot.borrow().document(), seed);
    }

    #[tokio::test]
    async fn hostile_text_keeps_the_export_structure() {
        let document = hostile_document();
        let host = Host::new(Document::default(), None).unwrap();
        assert_eq!(
            send(&host, Method::PUT, "/api/document", Some(json!(document))).await,
            (StatusCode::NO_CONTENT, String::new())
        );

        let (_, markdown) = send(&host, Method::GET, "/api/markdown", None).await;
        assert_eq!(outline(&markdown), hostile_outline());

        for name in HOSTILE {
            for kind in ["players", "games"] {
                let uri = format!("/api/{kind}/{}", encoded(name));
                assert_eq!(
                    send(&host, Method::DELETE, &uri, None).await,
                    (StatusCode::NO_CONTENT, String::new()),
                    "{uri}"
                );
            }
        }
        let emptied = Document {
            players: Vec::new(),
            games: Vec::new(),
            analysis: Vec::new(),
            ..document
        };
        assert_eq!(*host.0.snapshot.borrow().document(), emptied);
    }

    fn hostile_document() -> Document {
        let text = str::to_owned;
        let number = |value: f64| Decimal::try_from(value).unwrap();
        let entry = |name: &str| Entry {
            name: text(name),
            description: text(name),
        };
        let stated = |label: &str, welfare: f64| {
            Profile::Stated(Stated {
                label: text(label),
                welfare: number(welfare),
            })
        };
        let cells = HOSTILE.iter().enumerate().map(|(row, name)| {
            let cell = |other: &str, welfare: f64| Cell {
                outcomes: Pair([text(name), text(other)]),
                welfare: number(welfare),
            };
            vec![
                cell(neighbour(row, 1), row as f64 + 0.5),
                cell(neighbour(row, 2), -(row as f64)),
            ]
        });
        Document {
            title: text(HOSTILE[0]),
            summary: HOSTILE.join("\n"),
            players: HOSTILE
                .iter()
                .map(|name| Player {
                    name: text(name),
                    summary: text(name),
                    games: vec![entry(name)],
                    moves: vec![entry(name)],
                    outcomes: vec![Outcome {
                        name: text(name),
                        effect: text(name),
                    }],
                })
                .collect(),
            games: HOSTILE
                .iter()
                .map(|name| Game {
                    name: text(name),
                    trigger: text(name),
                    players: vec![text(name), text(name)],
                    steps: HOSTILE.map(text).to_vec(),
                })
                .collect(),
            analysis: vec![
                Analysis {
                    game: text(HOSTILE[0]),
                    row_player: text(" <div>"),
                    col_player: text("a | b"),
                    row_moves: HOSTILE.map(text).to_vec(),
                    col_moves: vec![text("    indented"), text("|")],
                    cells: cells.collect(),
                    equilibrium: stated("a | b", 0.1),
                    pareto_optimal: stated("`a|b`", 0.3),
                    fix: text("# heading"),
                    worse_than_deadweight: text("- bullet"),
                },
                Analysis {
                    game: text(HOSTILE[1]),
                    row_player: text("P"),
                    col_player: text("Q"),
                    row_moves: vec![text("Up")],
                    col_moves: vec![text("Left")],
                    cells: vec![vec![Cell {
                        outcomes: Pair([text("Win"), text("Lose")]),
                        welfare: number(2.5),
                    }]],
                    equilibrium: Profile::Cell(Pair([text("Up"), text("Left")])),
                    pareto_optimal: Profile::Cell(Pair([text("Up"), text("Left")])),
                    fix: String::new(),
                    worse_than_deadweight: String::new(),
                },
                Analysis {
                    game: text(HOSTILE[2]),
                    row_player: String::new(),
                    col_player: String::new(),
                    row_moves: Vec::new(),
                    col_moves: Vec::new(),
                    cells: Vec::new(),
                    equilibrium: stated("None", 0.0),
                    pareto_optimal: stated("Any", 1.5),
                    fix: text("Add a move"),
                    worse_than_deadweight: text("None"),
                },
            ],
        }
    }

    fn hostile_outline() -> Vec<String> {
        let shown = HOSTILE.map(str::trim);
        let row = |cells: &[&str]| {
            let cells = cells.iter().map(|cell| format!("cell({})", cell.trim()));
            format!("row({})", cells.collect::<String>())
        };
        let pairs = |header: [&str; 2], name: &str| {
            format!("table({}{})", row(&header), row(&[name, name]))
        };
        let mut outline = vec![
            format!("h1({})", shown[0]),
            format!("p({})", shown.join("\n")),
            "h2(Players)".to_owned(),
        ];
        for name in shown {
            outline.extend([
                format!("h3({name})"),
                format!("p({name})"),
                pairs(["Game", "Description"], name),
                pairs(["Move", "Description"], name),
                pairs(["Outcome", "Effect"], name),
            ]);
        }
        outline.push("h2(Games)".to_owned());
        for name in HOSTILE {
            let steps = shown.iter().map(|step| format!("li({step})"));
            outline.extend([
                format!("h3({})", name.trim()),
                format!(
                    "ul(li({})li({}))",
                    format!("Trigger: {name}").trim(),
                    format!("Players: {name}, {name}").trim()
                ),
                format!("ol({})", steps.collect::<String>()),
            ]);
        }
        let matrix = HOSTILE.iter().enumerate().map(|(index, name)| {
            row(&[
                &format!("<div>: {name}"),
                &format!("{name}, {} ({index}.5)", neighbour(index, 1)),
                &format!("{name}, {} ({})", neighbour(index, 2), -(index as i64)),
            ])
        });
        outline.extend([
            "p(\\newpage)".to_owned(),
            "h2(Payoff Analysis)".to_owned(),
            format!(
                "table({}{}{}{})",
                row(&[
                    "Game",
                    "Equilibrium",
                    "Welfare",
                    "Pareto-optimal",
                    "Gap",
                    "Status"
                ]),
                row(&[HOSTILE[0], "a | b", "0.1", "0.3", "0.2", "Deadweight"]),
                row(&[HOSTILE[1], "(Up, Left)", "2.5", "2.5", "0", "Clean"]),
                row(&[HOSTILE[2], "None", "0", "1.5", "1.5", "Deadweight"]),
            ),
            format!("h3({})", shown[0]),
            "p(Reduced payoff matrix:)".to_owned(),
            format!(
                "table({}{})",
                row(&["", "a | b:     indented", "a | b: |"]),
                matrix.collect::<String>()
            ),
            "ul(li(Equilibrium: a | b = 0.1)li(Pareto-optimal: `a|b` = 0.3)li(Fix: # heading)li(Worse than deadweight: - bullet))"
                .to_owned(),
            format!("h3({})", shown[2]),
            "p(Reduced payoff matrix:)".to_owned(),
            "table(row(cell()))".to_owned(),
            "ul(li(Equilibrium: None = 0)li(Pareto-optimal: Any = 1.5)li(Fix: Add a move)li(Worse than deadweight: None))"
                .to_owned(),
        ]);
        outline
    }

    fn neighbour(index: usize, distance: usize) -> &'static str {
        HOSTILE[(index + distance) % HOSTILE.len()]
    }

    fn outline(markdown: &str) -> Vec<String> {
        let mut blocks = Vec::new();
        let mut open = Vec::new();
        let mut block = String::new();
        let github = Options::ENABLE_TABLES
            | Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_TASKLISTS
            | Options::ENABLE_FOOTNOTES
            | Options::ENABLE_MATH
            | Options::ENABLE_GFM;
        for markup in Parser::new_ext(markdown, github) {
            match markup {
                Markup::Start(tag) => {
                    let name = match tag {
                        Tag::Heading { level, .. } => format!("h{}", level as usize),
                        Tag::Paragraph => "p".to_owned(),
                        Tag::Table(_) => "table".to_owned(),
                        Tag::TableHead | Tag::TableRow => "row".to_owned(),
                        Tag::TableCell => "cell".to_owned(),
                        Tag::List(None) => "ul".to_owned(),
                        Tag::List(Some(_)) => "ol".to_owned(),
                        Tag::Item => "li".to_owned(),
                        Tag::Strong => String::new(),
                        other => format!("{other:?}"),
                    };
                    if !name.is_empty() {
                        block.push_str(&name);
                        block.push('(');
                    }
                    open.push(!name.is_empty());
                }
                Markup::End(_) => {
                    if open.pop() == Some(true) {
                        block.push(')');
                    }
                    if open.is_empty() {
                        blocks.push(mem::take(&mut block));
                    }
                }
                Markup::Text(text) => block.push_str(&text),
                Markup::SoftBreak | Markup::HardBreak => block.push('\n'),
                other => block.push_str(&format!("{other:?}")),
            }
        }
        blocks
    }

    fn encoded(name: &str) -> String {
        name.bytes()
            .map(|byte| {
                if byte.is_ascii_alphanumeric() {
                    char::from(byte).to_string()
                } else {
                    format!("%{byte:02X}")
                }
            })
            .collect()
    }

    async fn send(
        host: &Host,
        method: Method,
        uri: &str,
        body: Option<Value>,
    ) -> (StatusCode, String) {
        let request = Request::builder().method(method).uri(uri);
        let request = match body {
            Some(body) => request
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body.to_string())),
            None => request.body(Body::empty()),
        };
        let response = router(host.clone())
            .oneshot(request.unwrap())
            .await
            .unwrap();
        let status = response.status();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        (status, String::from_utf8(body.to_vec()).unwrap())
    }
}
