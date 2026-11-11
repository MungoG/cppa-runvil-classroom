use crate::decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashSet, fmt, iter};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(default, deny_unknown_fields)]
pub struct Document {
    pub title: String,
    pub summary: String,
    pub players: Vec<Player>,
    pub games: Vec<Game>,
    pub analysis: Vec<Analysis>,
}

impl Document {
    pub fn put_player(&mut self, player: Player) {
        put(&mut self.players, player);
    }

    pub fn remove_player(&mut self, name: &str) -> Result<(), Error> {
        remove(&mut self.players, "player", name)
    }

    pub fn put_game(&mut self, game: Game) {
        put(&mut self.games, game);
    }

    pub fn remove_game(&mut self, name: &str) -> Result<(), Error> {
        remove(&mut self.games, "game", name)?;
        self.analysis.retain(|analysis| analysis.game != name);
        Ok(())
    }

    pub fn put_analysis(&mut self, analysis: Analysis) -> Result<(), Error> {
        if !self.games.iter().any(|game| game.name == analysis.game) {
            return Err(Error::NotFound(format!("game `{}`", analysis.game)));
        }
        put(&mut self.analysis, analysis);
        Ok(())
    }

    pub fn remove_analysis(&mut self, game: &str) -> Result<(), Error> {
        remove(&mut self.analysis, "analysis of game", game)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
#[schema(
    description = "`games` links the player to games by name. `moves` and `outcomes` name what the player can do and can get."
)]
pub struct Player {
    #[serde(default)]
    pub name: String,
    pub summary: String,
    pub games: Vec<Entry>,
    pub moves: Vec<Entry>,
    pub outcomes: Vec<Outcome>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct Outcome {
    pub name: String,
    pub effect: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
#[schema(
    description = "`players` lists player names. `steps` are short sentences in the order they happen."
)]
pub struct Game {
    #[serde(default)]
    pub name: String,
    pub trigger: String,
    pub players: Vec<String>,
    pub steps: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
#[schema(
    description = "Payoff analysis of one game, reduced to its key two-player interaction. `cells[r][c]` belongs to `row_moves[r]` and `col_moves[c]`. The server derives welfare, gap and status. A gap above zero requires `fix` and `worse_than_deadweight`; send empty strings when there is no gap."
)]
pub struct Analysis {
    #[serde(default)]
    pub game: String,
    pub row_player: String,
    pub col_player: String,
    pub row_moves: Vec<String>,
    pub col_moves: Vec<String>,
    pub cells: Vec<Vec<Cell>>,
    pub equilibrium: Profile,
    pub pareto_optimal: Profile,
    pub fix: String,
    pub worse_than_deadweight: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
#[schema(
    description = "`outcomes` names the outcome of the row player, then of the column player. `welfare` is the total welfare of the cell."
)]
pub struct Cell {
    pub outcomes: Pair,
    #[schema(value_type = f64)]
    pub welfare: Decimal,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(
    untagged,
    expecting = "a [row move, column move] pair or a {label, welfare} object"
)]
#[schema(
    description = "Either the `[row move, column move]` of one matrix cell, whose welfare the server reads from the matrix, or a stated `{label, welfare}` for anything that is not one cell: a mixed strategy, several equilibria, no equilibrium."
)]
pub enum Profile {
    Cell(Pair),
    Stated(Stated),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[schema(
    description = "One text for the row player, then one for the column player.",
    min_items = 2,
    max_items = 2
)]
pub struct Pair(pub [String; 2]);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
#[schema(
    description = "`label` is shown where a cell would show `(Move, Move)`. `welfare` is the expected or chosen total welfare."
)]
pub struct Stated {
    pub label: String,
    #[schema(value_type = f64)]
    pub welfare: Decimal,
}

#[derive(Clone, Debug, PartialEq, Serialize, ToSchema)]
#[schema(
    description = "Derived values of one analysis, as the Payoff Analysis section shows them. Numbers are exact decimal text."
)]
pub struct Payoff {
    pub equilibrium: String,
    pub welfare: String,
    pub pareto_optimal: String,
    pub pareto_welfare: String,
    pub gap: String,
    pub status: Status,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, ToSchema)]
pub enum Status {
    Clean,
    Deadweight,
}

impl fmt::Display for Status {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Clean => "Clean",
            Self::Deadweight => "Deadweight",
        })
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
#[schema(description = "`payoff[i]` belongs to `document.analysis[i]`.")]
pub struct Snapshot {
    document: Document,
    payoff: Vec<Payoff>,
}

impl Snapshot {
    pub fn new(document: Document) -> Result<Self, Error> {
        let json =
            serde_json::to_value(&document).map_err(|error| Error::Invalid(error.to_string()))?;
        check_text(&json, "document")?;
        unique("player", document.players.iter().map(Named::name))?;
        unique("game", document.games.iter().map(Named::name))?;
        unique("analysed game", document.analysis.iter().map(Named::name))?;
        let payoff = document
            .analysis
            .iter()
            .map(|analysis| {
                if document.games.iter().any(|game| game.name == analysis.game) {
                    analysis.payoff()
                } else {
                    Err(Error::Invalid(format!(
                        "analysis refers to unknown game `{}`",
                        analysis.game
                    )))
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { document, payoff })
    }

    pub fn document(&self) -> &Document {
        &self.document
    }

    pub fn markdown(&self) -> String {
        let document = &self.document;
        let mut blocks = vec![
            format!("# {}", escape(&document.title)),
            paragraph(&document.summary),
            "## Players".to_owned(),
        ];
        for player in &document.players {
            let entries = |entries: &[Entry]| {
                entries
                    .iter()
                    .map(|entry| [escape(&entry.name), escape(&entry.description)])
                    .collect::<Vec<_>>()
            };
            let outcomes = player
                .outcomes
                .iter()
                .map(|outcome| [escape(&outcome.name), escape(&outcome.effect)]);
            blocks.extend([
                format!("### {}", escape(&player.name)),
                paragraph(&player.summary),
                table(["Game", "Description"], entries(&player.games)),
                table(["Move", "Description"], entries(&player.moves)),
                table(["Outcome", "Effect"], outcomes),
            ]);
        }
        blocks.push("## Games".to_owned());
        for game in &document.games {
            let players = game
                .players
                .iter()
                .map(|name| escape(name))
                .collect::<Vec<_>>();
            let steps = game
                .steps
                .iter()
                .enumerate()
                .map(|(index, step)| format!("{}. {}", index + 1, guard(step)));
            blocks.extend([
                format!("### {}", escape(&game.name)),
                format!(
                    "- Trigger: {}\n- Players: {}",
                    escape(&game.trigger),
                    players.join(", ")
                ),
                steps.collect::<Vec<_>>().join("\n"),
            ]);
        }
        blocks.extend(["\\newpage".to_owned(), "## Payoff Analysis".to_owned()]);
        let analyses = document.analysis.iter().zip(&self.payoff);
        let rows = analyses.clone().map(|(analysis, payoff)| {
            [
                escape(&analysis.game),
                escape(&payoff.equilibrium),
                payoff.welfare.clone(),
                payoff.pareto_welfare.clone(),
                payoff.gap.clone(),
                payoff.status.to_string(),
            ]
        });
        blocks.push(table(
            [
                "Game",
                "Equilibrium",
                "Welfare",
                "Pareto-optimal",
                "Gap",
                "Status",
            ],
            rows,
        ));
        for (analysis, payoff) in analyses.filter(|(_, payoff)| payoff.status == Status::Deadweight)
        {
            let Analysis {
                row_player,
                col_player,
                ..
            } = analysis;
            let header = iter::once(String::new())
                .chain(
                    analysis
                        .col_moves
                        .iter()
                        .map(|action| escape(&format!("{col_player}: {action}"))),
                )
                .collect::<Vec<_>>();
            let rows = analysis
                .row_moves
                .iter()
                .zip(&analysis.cells)
                .map(|(action, cells)| {
                    let label = format!("{row_player}: {action}");
                    iter::once(format!("**{}**", escape(label.trim())))
                        .chain(cells.iter().map(|cell| {
                            let Pair([row, col]) = &cell.outcomes;
                            escape(&format!("{row}, {col} ({})", cell.welfare))
                        }))
                        .collect::<Vec<_>>()
                });
            blocks.extend([
                format!("### {}", escape(&analysis.game)),
                "**Reduced payoff matrix:**".to_owned(),
                table(header, rows),
                format!(
                    "- **Equilibrium:** {} = {}\n- **Pareto-optimal:** {} = {}\n- **Fix:** {}\n- **Worse than deadweight:** {}",
                    escape(&payoff.equilibrium),
                    payoff.welfare,
                    escape(&payoff.pareto_optimal),
                    payoff.pareto_welfare,
                    escape(&analysis.fix),
                    escape(&analysis.worse_than_deadweight)
                ),
            ]);
        }
        blocks.retain(|block| !block.is_empty());
        blocks.join("\n\n") + "\n"
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    NotFound(String),
    Invalid(String),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(what) => write!(formatter, "{what} not found"),
            Self::Invalid(reason) => formatter.write_str(reason),
        }
    }
}

trait Named {
    fn name(&self) -> &str;
}

impl Named for Player {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Named for Game {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Named for Analysis {
    fn name(&self) -> &str {
        &self.game
    }
}

impl Analysis {
    fn payoff(&self) -> Result<Payoff, Error> {
        let invalid =
            |reason: String| Error::Invalid(format!("analysis of `{}`: {reason}", self.game));
        unique("row move", self.row_moves.iter().map(String::as_str))?;
        unique("column move", self.col_moves.iter().map(String::as_str))?;
        let (rows, columns) = (self.row_moves.len(), self.col_moves.len());
        if self.cells.len() != rows || self.cells.iter().any(|row| row.len() != columns) {
            return Err(invalid(format!(
                "cells must hold {rows} rows of {columns} cells"
            )));
        }
        let resolve = |field: &str, profile: &Profile| match profile {
            Profile::Stated(stated) => Ok((stated.label.clone(), stated.welfare)),
            Profile::Cell(Pair([row, col])) => self
                .row_moves
                .iter()
                .position(|action| action == row)
                .zip(self.col_moves.iter().position(|action| action == col))
                .and_then(|(row, col)| self.cells.get(row)?.get(col))
                .map(|cell| (format!("({row}, {col})"), cell.welfare))
                .ok_or_else(|| invalid(format!("{field} ({row}, {col}) is not a matrix cell"))),
        };
        let (equilibrium, welfare) = resolve("equilibrium", &self.equilibrium)?;
        let (pareto_optimal, pareto_welfare) = resolve("pareto_optimal", &self.pareto_optimal)?;
        let gap = pareto_welfare.checked_sub(welfare).ok_or_else(|| {
            invalid("welfare values are too far apart to subtract exactly".to_owned())
        })?;
        if gap.is_negative() {
            return Err(invalid(
                "pareto_optimal has less welfare than equilibrium".to_owned(),
            ));
        }
        let unexplained = [&self.fix, &self.worse_than_deadweight]
            .iter()
            .any(|text| text.trim().is_empty());
        if !gap.is_zero() && unexplained {
            return Err(invalid(
                "a gap requires fix and worse_than_deadweight".to_owned(),
            ));
        }
        Ok(Payoff {
            equilibrium,
            welfare: welfare.to_string(),
            pareto_optimal,
            pareto_welfare: pareto_welfare.to_string(),
            gap: gap.to_string(),
            status: if gap.is_zero() {
                Status::Clean
            } else {
                Status::Deadweight
            },
        })
    }
}

fn put<T: Named>(items: &mut Vec<T>, item: T) {
    match items
        .iter_mut()
        .find(|existing| existing.name() == item.name())
    {
        Some(existing) => *existing = item,
        None => items.push(item),
    }
}

fn remove<T: Named>(items: &mut Vec<T>, kind: &str, name: &str) -> Result<(), Error> {
    let before = items.len();
    items.retain(|item| item.name() != name);
    if items.len() == before {
        Err(Error::NotFound(format!("{kind} `{name}`")))
    } else {
        Ok(())
    }
}

fn unique<'a>(kind: &str, names: impl IntoIterator<Item = &'a str>) -> Result<(), Error> {
    let mut seen = HashSet::new();
    for name in names {
        if name.trim().is_empty() || [".", ".."].contains(&name) {
            return Err(Error::Invalid(format!(
                "{kind} name `{name}` is not usable"
            )));
        }
        if !seen.insert(name) {
            return Err(Error::Invalid(format!("{kind} `{name}` is listed twice")));
        }
    }
    Ok(())
}

fn check_text(value: &Value, path: &str) -> Result<(), Error> {
    match value {
        Value::String(text) if path.ends_with(".summary") => {
            if text.is_empty() || lines(text).all(|line| !line.trim().is_empty()) {
                Ok(())
            } else {
                Err(Error::Invalid(format!("{path} must be one paragraph")))
            }
        }
        Value::String(text) => {
            if text.contains(['\n', '\r']) {
                Err(Error::Invalid(format!("{path} must be one line")))
            } else {
                Ok(())
            }
        }
        Value::Array(items) => items
            .iter()
            .enumerate()
            .try_for_each(|(index, item)| check_text(item, &format!("{path}[{index}]"))),
        Value::Object(fields) => fields
            .iter()
            .try_for_each(|(key, field)| check_text(field, &format!("{path}.{key}"))),
        _ => Ok(()),
    }
}

fn lines(text: &str) -> impl Iterator<Item = &str> {
    text.split("\r\n").flat_map(|part| part.split(['\n', '\r']))
}

fn paragraph(text: &str) -> String {
    lines(text).map(guard).collect::<Vec<_>>().join("\n")
}

fn guard(line: &str) -> String {
    let line = escape(line.trim_start());
    let marker = line.split([' ', '\t']).next().unwrap_or_default();
    let numbered = marker.strip_suffix(['.', ')']).is_some_and(|digits| {
        !digits.is_empty() && digits.bytes().all(|digit| digit.is_ascii_digit())
    });
    let ruled = !line.is_empty() && line.chars().all(|symbol| "-=: \t".contains(symbol));
    if numbered {
        let (digits, rest) = line.split_at(marker.len() - 1);
        format!("{digits}\\{rest}")
    } else if ruled || line.starts_with('>') || ["-", "+"].contains(&marker) {
        format!("\\{line}")
    } else {
        line
    }
}

fn escape(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for (index, symbol) in text.char_indices() {
        let shortcode = symbol == ':'
            && text[index + 1..].split_once(':').is_some_and(|(name, _)| {
                !name.is_empty()
                    && name
                        .chars()
                        .all(|letter| letter.is_ascii_alphanumeric() || "_+-".contains(letter))
            });
        if "\\`*_[<&~|#$".contains(symbol) || shortcode {
            escaped.push('\\');
        }
        escaped.push(symbol);
    }
    escaped
}

fn table<H, R, S>(header: H, rows: impl IntoIterator<Item = R>) -> String
where
    H: IntoIterator<Item: Into<String>>,
    R: IntoIterator<Item = S>,
    S: Into<String>,
{
    let header = header.into_iter().map(Into::into).collect::<Vec<_>>();
    let rows = iter::once(header)
        .chain(
            rows.into_iter()
                .map(|row| row.into_iter().map(Into::into).collect()),
        )
        .collect::<Vec<Vec<String>>>();
    let columns = rows.iter().map(Vec::len).max().unwrap_or(0);
    let widths = (0..columns)
        .map(|column| {
            rows.iter()
                .filter_map(|row| row.get(column))
                .map(|cell| cell.chars().count())
                .max()
                .unwrap_or(0)
                .max(3)
        })
        .collect::<Vec<_>>();
    let line = |cells: Vec<String>| format!("| {} |", cells.join(" | "));
    let mut lines = rows
        .iter()
        .map(|row| {
            line(
                row.iter()
                    .zip(&widths)
                    .map(|(cell, &width)| format!("{cell:<width$}"))
                    .collect(),
            )
        })
        .collect::<Vec<_>>();
    let rule = widths
        .iter()
        .map(|&width| "-".repeat(width))
        .collect::<Vec<_>>();
    lines.insert(1, line(rule));
    lines.join("\n")
}
