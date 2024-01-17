use crate::token::Action::{Move, Pass};
use crate::token::Color::{Black, White};
use crate::token::Outcome::{Draw, WinnerByForfeit, WinnerByPoints, WinnerByResign, WinnerByTime};
use crate::{SgfError, SgfErrorKind};
use std::ops::Not;

/// Indicates what color the token is related to
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Color {
    Black,
    White,
}

impl Not for Color {
    type Output = Color;
    fn not(self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Outcome {
    WinnerByResign(Color),
    WinnerByForfeit(Color),
    WinnerByPoints(Color, f32),
    WinnerByTime(Color),
    Draw,
    Unknown(String),
}

impl Outcome {
    pub fn get_winner(self) -> Option<Color> {
        match self {
            WinnerByTime(color)
            | WinnerByForfeit(color)
            | WinnerByPoints(color, ..)
            | WinnerByResign(color) => Some(color),
            _ => None,
        }
    }
}

///Provides the used rules for this game.
///Because there are many different rules, SGF requires
///mandatory names only for a small set of well known rule sets.
///Note: it's beyond the scope of this specification to give an
///exact specification of these rule sets.
///Mandatory names for Go (GM[1]):
/// "AGA" (rules of the American Go Association)
/// "GOE" (the Ing rules of Goe)
/// "Japanese" (the Nihon-Kiin rule set)
/// "NZ" (New Zealand rules)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RuleSet {
    Japanese,
    NZ,
    GOE,
    AGA,
    Chinese,
    Unknown(String),
}

impl From<&str> for RuleSet {
    fn from(s: &str) -> Self {
        match s {
            "Japanese" => RuleSet::Japanese,
            "AGA" => RuleSet::AGA,
            "NZ" => RuleSet::NZ,
            "Chinese" => RuleSet::Chinese,
            "GOE" => RuleSet::GOE,
            value => RuleSet::Unknown(value.to_owned()),
        }
    }
}

impl ToString for RuleSet {
    fn to_string(&self) -> String {
        match self {
            RuleSet::Japanese => "Japanese",
            RuleSet::NZ => "NZ",
            RuleSet::GOE => "GOE",
            RuleSet::AGA => "AGA",
            RuleSet::Chinese => "Chinese",
            RuleSet::Unknown(v) => v,
        }
        .to_owned()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Action {
    Move(u8, u8),
    Pass,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Game {
    Go,
    Other(u8),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Encoding {
    UTF8,
    Other(String),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DisplayNodes {
    Children,
    Siblings,
}

/// Enum describing all possible SGF Properties
#[derive(Debug, PartialEq, Clone)]
pub enum SgfToken {
    Add {
        color: Color,
        coordinates: Vec<(u8, u8)>,
    },
    Move {
        color: Color,
        action: Action,
    },
    Time {
        color: Color,
        time: u32,
    },
    PlayerName {
        color: Color,
        name: String,
    },
    PlayerRank {
        color: Color,
        rank: String,
    },
    Game(Game),
    Rule(RuleSet),
    Result(Outcome),
    Komi(f32),
    Event(String),
    Copyright(String),
    GameName(String),
    VariationDisplay {
        nodes: DisplayNodes,
        on_board_display: bool,
    },
    Place(String),
    Date(String),
    Size(u32, u32),
    FileFormat(u8),
    Overtime(String),
    TimeLimit(u32),
    MovesRemaining {
        color: Color,
        moves: u32,
    },
    Handicap(u32),
    Comment(String),
    Charset(Encoding),
    Application {
        name: String,
        version: String,
    },
    Unknown((String, String)),
    Invalid((String, String)),
    Square {
        coordinates: Vec<(u8, u8)>,
    },
    Triangle {
        coordinates: Vec<(u8, u8)>,
    },
    Circle {
        coordinates: Vec<(u8, u8)>,
    },
    Cross {
        coordinates: Vec<(u8, u8)>,
    },
    Selected {
        coordinates: Vec<(u8, u8)>,
    },
    DimPoints {
        coordinates: Vec<(u8, u8)>,
    },
    TerritoryBlack {
        coordinates: Vec<(u8, u8)>,
    },
    TerritoryWhite {
        coordinates: Vec<(u8, u8)>,
    },
    Label {
        label: String,
        coordinate: (u8, u8),
    },
    NodeName(String),
    AddEmpty {
        coordinates: Vec<(u8, u8)>,
    },
    MoveNumber(u32),
    User(String),
    Source(String),
    GameComment(String),
}

impl SgfToken {
    /// Converts a `identifier` and `value` pair to a SGF token
    ///
    /// Returns `SgfToken::Unknown((identifier, value))` for tokens without a matching identifier
    ///
    /// Returns `SgfToken::Invalid((identifier, value))` for tokens with a matching identifier, but invalid value
    ///
    /// ```rust
    /// use sgf_parser::*;
    ///
    /// let token = SgfToken::from_pair("B", "aa");
    /// assert_eq!(token, SgfToken::Move { color: Color::Black, action: Action::Move(1, 1) });
    ///
    /// let token = SgfToken::from_pair("B", "");
    /// assert_eq!(token, SgfToken::Move { color: Color::Black, action: Action::Pass });
    ///
    /// let token = SgfToken::from_pair("B", "not_coord");
    /// assert_eq!(token, SgfToken::Invalid(("B".to_string(), "not_coord".to_string())));
    ///
    /// let token = SgfToken::from_pair("FOO", "aa");
    /// assert_eq!(token, SgfToken::Unknown(("FOO".to_string(), "aa".to_string())));
    /// ```
    pub fn from_pair(base_ident: &str, value: &str) -> SgfToken {
        let ident = base_ident
            .chars()
            .filter(|c| c.is_uppercase())
            .collect::<String>();
        let token: Option<SgfToken> = match ident.as_ref() {
            "LB" => split_label_text(value).and_then(|(coord, label)| {
                str_to_coordinates(coord)
                    .ok()
                    .map(|coordinate| SgfToken::Label {
                        label: label[1..].to_string(),
                        coordinate,
                    })
            }),
            "HA" => match value.parse() {
                Ok(value) => Some(SgfToken::Handicap(value)),
                _ => None,
            },
            "RU" => Some(SgfToken::Rule(RuleSet::from(value))),
            "SQ" | "TR" | "CR" | "MA" | "SL" | "TB" | "TW" | "DD" => {
                let coordinates = if let Some((start, end)) = value.split_once(':') {
                    let start = str_to_coordinates(start).ok().unwrap();
                    let end = str_to_coordinates(end).ok().unwrap();
                    (start.0..=end.0)
                        .flat_map(|x| (start.1..=end.1).map(move |y| (x, y)))
                        .collect()
                } else {
                    vec![str_to_coordinates(value).ok().unwrap()]
                };
                match ident.as_ref() {
                    "SQ" => Some(SgfToken::Square { coordinates }),
                    "TR" => Some(SgfToken::Triangle { coordinates }),
                    "CR" => Some(SgfToken::Circle { coordinates }),
                    "MA" => Some(SgfToken::Cross { coordinates }),
                    "SL" => Some(SgfToken::Selected { coordinates }),
                    "TB" => Some(SgfToken::TerritoryBlack { coordinates }),
                    "TW" => Some(SgfToken::TerritoryWhite { coordinates }),
                    "DD" => Some(SgfToken::DimPoints { coordinates }),
                    _ => None,
                }
            }
            "AB" | "AW" | "AE" => {
                let coordinates = if let Some((start, end)) = value.split_once(':') {
                    let start = str_to_coordinates(start).ok().unwrap();
                    let end = str_to_coordinates(end).ok().unwrap();
                    (start.0..=end.0)
                        .flat_map(|x| (start.1..=end.1).map(move |y| (x, y)))
                        .collect()
                } else {
                    vec![str_to_coordinates(value).ok().unwrap()]
                };
                match ident.as_ref() {
                    "AB" => Some(SgfToken::Add {
                        color: Color::Black,
                        coordinates,
                    }),
                    "AW" => Some(SgfToken::Add {
                        color: Color::White,
                        coordinates,
                    }),
                    "AE" => Some(SgfToken::AddEmpty { coordinates }),
                    _ => None,
                }
            }
            "B" | "W" => {
                let color = if ident == "B" {
                    Color::Black
                } else {
                    Color::White
                };
                move_str_to_coord(value)
                    .ok()
                    .map(|coordinate| SgfToken::Move {
                        color,
                        action: coordinate,
                    })
            }
            "BL" | "WL" => {
                let color = if ident == "BL" {
                    Color::Black
                } else {
                    Color::White
                };
                value
                    .parse()
                    .ok()
                    .map(|time| SgfToken::Time { color, time })
            }
            "PB" | "PW" => Some(SgfToken::PlayerName {
                color: if ident == "PB" {
                    Color::Black
                } else {
                    Color::White
                },
                name: value.to_string(),
            }),
            "BR" | "WR" => Some(SgfToken::PlayerRank {
                color: if ident == "BR" {
                    Color::Black
                } else {
                    Color::White
                },
                rank: value.to_string(),
            }),
            "RE" => parse_outcome_str(value).ok().map(SgfToken::Result),
            "KM" => value.parse().ok().map(SgfToken::Komi),
            "SZ" => {
                if let Some((width, height)) = split_size_text(value) {
                    Some(SgfToken::Size(width, height))
                } else {
                    value.parse().ok().map(|size| SgfToken::Size(size, size))
                }
            }
            "FF" => value.parse().ok().map(|v| match v {
                0..=4 => SgfToken::FileFormat(v),
                _ => SgfToken::Invalid((ident.to_string(), value.to_string())),
            }),
            "TM" => value.parse().ok().map(SgfToken::TimeLimit),
            "MN" => value.parse().ok().map(SgfToken::MoveNumber),
            "EV" => Some(SgfToken::Event(value.to_string())),
            "OT" => Some(SgfToken::Overtime(value.to_string())),
            "C" => Some(SgfToken::Comment(value.to_string())),
            "GN" => Some(SgfToken::GameName(value.to_string())),
            "N" => Some(SgfToken::NodeName(value.to_string())),
            "US" => Some(SgfToken::User(value.to_string())),
            "SO" => Some(SgfToken::Source(value.to_string())),
            "GC" => Some(SgfToken::GameComment(value.to_string())),
            "CP" => Some(SgfToken::Copyright(value.to_string())),
            "DT" => Some(SgfToken::Date(value.to_string())),
            "PC" => Some(SgfToken::Place(value.to_string())),
            "GM" => match value.parse::<u8>() {
                Ok(1) => Some(SgfToken::Game(Game::Go)),
                Ok(n) => Some(SgfToken::Game(Game::Other(n))),
                Err(_) => Some(SgfToken::Invalid((
                    base_ident.to_string(),
                    value.to_string(),
                ))),
            },
            "CA" => match value.to_string().to_lowercase().as_str() {
                "utf-8" => Some(SgfToken::Charset(Encoding::UTF8)),
                _ => Some(SgfToken::Charset(Encoding::Other(value.to_string()))),
            },
            "OB" | "OW" => match value.parse::<u32>() {
                Ok(n) => Some(SgfToken::MovesRemaining {
                    color: if ident == "OB" {
                        Color::Black
                    } else {
                        Color::White
                    },
                    moves: n,
                }),
                Err(_) => Some(SgfToken::Invalid((
                    base_ident.to_string(),
                    value.to_string(),
                ))),
            },
            "AP" => parse_application_str(value)
                .ok()
                .map(|(name, version)| SgfToken::Application { name, version }),
            "ST" => parse_variation_display_str(value)
                .ok()
                .map(|(nodes, on_board_display)| SgfToken::VariationDisplay {
                    nodes,
                    on_board_display,
                }),
            _ => Some(SgfToken::Unknown((
                base_ident.to_string(),
                value.to_string(),
            ))),
        };
        match token {
            Some(token) => token,
            _ => SgfToken::Invalid((base_ident.to_string(), value.to_string())),
        }
    }

    /// Checks if the token is a root token as defined by the SGF spec.
    ///
    /// Root tokens can only occur in the root of a gametree collection, and they are invalid
    /// anywhere else
    ///
    /// ```
    /// use sgf_parser::*;
    ///
    /// let token = SgfToken::from_pair("SZ", "19");
    /// assert!(token.is_root_token());
    ///
    /// let token = SgfToken::from_pair("B", "aa");
    /// assert!(!token.is_root_token());
    /// ```
    pub fn is_root_token(&self) -> bool {
        use SgfToken::*;
        matches!(
            self,
            Size(_, _)
                | Charset(_)
                | FileFormat(_)
                | Game(_)
                | VariationDisplay { .. }
                | Application { .. }
        )
    }

    /// Checks if the token is a setup token as defined by the SGF spec.
    ///
    /// Setup tokens modify the current position, and should not be on the same node as move tokens
    ///
    /// ```
    /// use sgf_parser::*;
    ///
    /// let token = SgfToken::from_pair("AB", "aa");
    /// assert!(token.is_setup_token());
    /// let token = SgfToken::from_pair("AW", "aa");
    /// assert!(token.is_setup_token());
    ///
    /// let token = SgfToken::from_pair("SZ", "19");
    /// assert!(!token.is_setup_token());
    /// ```
    pub fn is_setup_token(&self) -> bool {
        use SgfToken::*;
        matches!(self, Add { .. } | AddEmpty { .. })
    }

    /// Checks if the token is a game info token as defined by the SGF spec.
    ///
    /// Game info tokens provide some information about the game played, usually stored in the root
    /// node
    ///
    /// ```
    /// use sgf_parser::*;
    ///
    /// let token = SgfToken::from_pair("RE", "W+T");
    /// assert!(token.is_game_info_token());
    ///
    /// let token = SgfToken::from_pair("SZ", "19");
    /// assert!(!token.is_game_info_token());
    /// ```
    pub fn is_game_info_token(&self) -> bool {
        use SgfToken::*;
        matches!(
            self,
            Date(_)
                | GameName(_)
                | Handicap(_)
                | Komi(_)
                | Overtime(_)
                | Event(_)
                | Result(_)
                | Rule(_)
                | Place(_)
                | TimeLimit(_)
                | PlayerName { .. }
                | PlayerRank { .. }
                | Copyright(_)
        )
    }
}

impl Into<String> for &SgfToken {
    fn into(self) -> String {
        match self {
            SgfToken::Label { label, coordinate } => {
                let value = coordinate_to_str(*coordinate);
                format!("LB[{}:{}]", value, label)
            }
            SgfToken::Handicap(nb_stones) => format!("HA[{}]", nb_stones),
            SgfToken::Rule(rule) => format!("RU[{}]", rule.to_string()),
            SgfToken::Result(outcome) => match outcome.clone() {
                WinnerByPoints(color, points) => format!(
                    "RE[{}+{}]",
                    match color {
                        Black => "B",
                        White => "W",
                    },
                    points
                ),
                WinnerByResign(color) => format!(
                    "RE[{}+R]",
                    match color {
                        Black => "B",
                        White => "W",
                    }
                ),

                WinnerByTime(color) => format!(
                    "RE[{}+T]",
                    match color {
                        Black => "B",
                        White => "W",
                    }
                ),
                WinnerByForfeit(color) => format!(
                    "RE[{}+F]",
                    match color {
                        Black => "B",
                        White => "W",
                    }
                ),
                Draw => "RE[Draw]".to_string(),
                Outcome::Unknown(s) => format!("RE[{}]", s),
            },
            SgfToken::Square { coordinates } => {
                format!("SQ[{}]", compressed_coordinate_to_str(coordinates.clone()))
            }
            SgfToken::Triangle { coordinates } => {
                format!("TR[{}]", compressed_coordinate_to_str(coordinates.clone()))
            }
            SgfToken::Circle { coordinates } => {
                format!("CR[{}]", compressed_coordinate_to_str(coordinates.clone()))
            }
            SgfToken::Cross { coordinates } => {
                format!("MA[{}]", compressed_coordinate_to_str(coordinates.clone()))
            }
            SgfToken::Selected { coordinates } => {
                format!("SL[{}]", compressed_coordinate_to_str(coordinates.clone()))
            }
            SgfToken::DimPoints { coordinates } => {
                format!("DD[{}]", compressed_coordinate_to_str(coordinates.clone()))
            }
            SgfToken::TerritoryBlack { coordinates } => {
                format!("TB[{}]", compressed_coordinate_to_str(coordinates.clone()))
            }
            SgfToken::TerritoryWhite { coordinates } => {
                format!("TW[{}]", compressed_coordinate_to_str(coordinates.clone()))
            }
            SgfToken::Add { color, coordinates } => {
                let token = match color {
                    Color::Black => "AB",
                    Color::White => "AW",
                };
                let value = compressed_coordinate_to_str((*coordinates).clone());
                format!("{}[{}]", token, value)
            }
            SgfToken::AddEmpty { coordinates } => {
                let value = compressed_coordinate_to_str((*coordinates).clone());
                format!("AE[{}]", value)
            }
            SgfToken::Move { color, action } => {
                let token = match color {
                    Color::Black => "B",
                    Color::White => "W",
                };
                let value = match *action {
                    Move(x, y) => coordinate_to_str((x, y)),
                    Pass => String::new(),
                };
                format!("{}[{}]", token, value)
            }
            SgfToken::Time { color, time } => {
                let token = match color {
                    Color::Black => "BL",
                    Color::White => "WL",
                };
                format!("{}[{}]", token, time)
            }
            SgfToken::PlayerName { color, name } => {
                let token = match color {
                    Color::Black => "PB",
                    Color::White => "PW",
                };
                format!("{}[{}]", token, name)
            }
            SgfToken::PlayerRank { color, rank } => {
                let token = match color {
                    Color::Black => "BR",
                    Color::White => "WR",
                };
                format!("{}[{}]", token, rank)
            }
            SgfToken::Komi(komi) => format!("KM[{}]", komi),
            SgfToken::FileFormat(v) => format!("FF[{}]", v),
            SgfToken::Size(width, height) if width == height => format!("SZ[{}]", width),
            SgfToken::Size(width, height) => format!("SZ[{}:{}]", width, height),
            SgfToken::TimeLimit(time) => format!("TM[{}]", time),
            SgfToken::Event(value) => format!("EV[{}]", value),
            SgfToken::Comment(value) => format!("C[{}]", value),
            SgfToken::MoveNumber(mn) => format!("MN[{}]", mn),
            SgfToken::Overtime(value) => format!("OT[{}]", value),
            SgfToken::GameName(value) => format!("GN[{}]", value),
            SgfToken::User(value) => format!("US[{}]", value),
            SgfToken::Source(value) => format!("SO[{}]", value),
            SgfToken::GameComment(value) => format!("GC[{}]", value),
            SgfToken::NodeName(value) => format!("N[{}]", value),
            SgfToken::Copyright(value) => format!("CP[{}]", value),
            SgfToken::Date(value) => format!("DT[{}]", value),
            SgfToken::Place(value) => format!("PC[{}]", value),
            SgfToken::Game(game) => format!(
                "GM[{}]",
                match game {
                    Game::Go => &1u8,
                    Game::Other(n) => n,
                }
            ),
            SgfToken::Charset(_) => "CA[UTF-8]".to_string(),
            SgfToken::MovesRemaining { color, moves } => format!(
                "O{}[{}]",
                match color {
                    Color::Black => 'B',
                    Color::White => 'W',
                },
                moves
            ),
            SgfToken::VariationDisplay {
                nodes,
                on_board_display,
            } => {
                let num = match (nodes, on_board_display) {
                    (DisplayNodes::Children, true) => 0,
                    (DisplayNodes::Siblings, true) => 1,
                    (DisplayNodes::Children, false) => 2,
                    (DisplayNodes::Siblings, false) => 3,
                };
                format!("ST[{}]", num)
            }
            SgfToken::Application { name, version } => format!("AP[{}:{}]", name, version),
            SgfToken::Unknown((ident, prop)) => format!("{}[{}]", ident, prop),
            SgfToken::Invalid((ident, prop)) => format!("{}[{}]", ident, prop),
        }
    }
}

impl Into<String> for SgfToken {
    fn into(self) -> String {
        (&self).into()
    }
}

/// Splits size input text (NN:MM) to corresponding width and height
fn split_size_text(input: &str) -> Option<(u32, u32)> {
    let index = input.find(':')?;
    let (width_part, height_part) = input.split_at(index);
    let width: u32 = width_part.parse().ok()?;
    let height: u32 = height_part[1..].parse().ok()?;
    Some((width, height))
}

/// Converts goban coordinate to string representation
fn coordinate_to_str(coordinate: (u8, u8)) -> String {
    fn to_char(c: u8) -> char {
        (c + if c < 27 { 96 } else { 38 }) as char
    }

    let x = to_char(coordinate.0);
    let y = to_char(coordinate.1);

    format!("{}{}", x, y)
}

fn compressed_coordinate_to_str(coordinates: Vec<(u8, u8)>) -> String {
    let to_char = |c: u8| -> char { (c + if c < 27 { 96 } else { 38 }) as char };

    match coordinates.len() {
        0 => "".to_string(),
        1 => {
            let x1 = to_char(coordinates[0].0);
            let y1 = to_char(coordinates[0].1);
            format!("{}{}", x1, y1)
        }
        _ => {
            let x1 = to_char(coordinates.first().unwrap().0);
            let y1 = to_char(coordinates.first().unwrap().1);
            let x2 = to_char(coordinates.last().unwrap().0);
            let y2 = to_char(coordinates.last().unwrap().1);
            format!("{}{}:{}{}", x1, y1, x2, y2)
        }
    }
}

/// If possible, splits a label text into coordinate and label pair
fn split_label_text(input: &str) -> Option<(&str, &str)> {
    if input.len() >= 4 {
        Some(input.split_at(2))
    } else {
        None
    }
}

fn parse_variation_display_str(input: &str) -> Result<(DisplayNodes, bool), SgfError> {
    match input.parse::<u8>() {
        Ok(0) => Ok((DisplayNodes::Children, true)),
        Ok(1) => Ok((DisplayNodes::Siblings, true)),
        Ok(2) => Ok((DisplayNodes::Children, false)),
        Ok(3) => Ok((DisplayNodes::Siblings, false)),
        _ => Err(SgfError::from(SgfErrorKind::ParseError)),
    }
}

fn parse_application_str(input: &str) -> Result<(String, String), SgfError> {
    let index = input
        .find(':')
        .ok_or_else(|| SgfError::from(SgfErrorKind::ParseError))?;
    let (name, version) = input.split_at(index);
    Ok((name.to_string(), version[1..].to_string()))
}

/// Provides the result of the game. It is MANDATORY to use the
/// following format:
/// "0" (zero) or "Draw" for a draw (jigo),
/// "B+" ["score"] for a black win and
/// "W+" ["score"] for a white win
/// Score is optional (some games don't have a score e.g. chess).
/// If the score is given it has to be given as a real value,
/// e.g. "B+0.5", "W+64", "B+12.5"
/// Use "B+R" or "B+Resign" and "W+R" or "W+Resign" for a win by
/// resignation. Applications must not write "Black resigns".
/// Use "B+T" or "B+Time" and "W+T" or "W+Time" for a win on time,
/// "B+F" or "B+Forfeit" and "W+F" or "W+Forfeit" for a win by
/// forfeit,
/// "Void" for no result or suspended play and
fn parse_outcome_str(s: &str) -> Result<Outcome, SgfError> {
    if s.is_empty() || s == "Void" {
        return Err(SgfError::from(SgfErrorKind::ParseError));
    }
    if s == "Draw" || s == "D" {
        return Ok(Draw);
    }

    let winner_option: Vec<&str> = s.split('+').collect();
    if winner_option.len() != 2 {
        return Ok(Outcome::Unknown(s.to_string()));
    }

    let winner: Color = match &winner_option[0] as &str {
        "B" => Black,
        "W" => White,
        _ => return Ok(Outcome::Unknown(s.to_string())),
    };

    match &winner_option[1] as &str {
        "F" | "Forfeit" => Ok(WinnerByForfeit(winner)),
        "R" | "Resign" => Ok(WinnerByResign(winner)),
        "T" | "Time" => Ok(WinnerByTime(winner)),
        points => {
            if let Ok(outcome) = points
                .parse::<f32>()
                .map(|score| WinnerByPoints(winner, score))
            {
                Ok(outcome)
            } else {
                Ok(Outcome::Unknown(s.to_string()))
            }
        }
    }
}

fn move_str_to_coord(input: &str) -> Result<Action, SgfError> {
    if input.is_empty() {
        Ok(Pass)
    } else {
        match str_to_coordinates(input) {
            Ok(coordinates) => {
                if coordinates.0 == 20 && coordinates.1 == 20 {
                    return Ok(Pass);
                } else {
                    Ok(Move(coordinates.0, coordinates.1))
                }
            }
            Err(e) => Err(e),
        }
    }
}

/// Converts a string describing goban coordinates to numeric coordinates
fn str_to_coordinates(input: &str) -> Result<(u8, u8), SgfError> {
    if input.len() != 2 {
        Err(SgfErrorKind::ParseError.into())
    } else {
        let coords = input
            .as_bytes()
            .iter()
            .map(|c| convert_u8_to_coordinate(*c))
            .collect::<Vec<_>>();
        Ok((coords[0], coords[1]))
    }
}

/// Converts a u8 char to numeric coordinates
///
#[inline]
fn convert_u8_to_coordinate(c: u8) -> u8 {
    if c > 96 {
        c - 96
    } else {
        c - 38
    }
}
