#[cfg(test)]
mod token_tests {
    use sgf_parser::Action::Move;
    use sgf_parser::*;

    #[test]
    fn can_check_token_type() {
        let normal_token = SgfToken::from_pair("B", "aa");
        assert!(!normal_token.is_game_info_token());
        assert!(!normal_token.is_setup_token());
        assert!(!normal_token.is_root_token());

        let root_token = SgfToken::from_pair("CA", "UTF-8");
        assert!(!root_token.is_game_info_token());
        assert!(!root_token.is_setup_token());
        assert!(root_token.is_root_token());

        let setup_token = SgfToken::from_pair("AB", "cd");
        assert!(!setup_token.is_game_info_token());
        assert!(setup_token.is_setup_token());
        assert!(!setup_token.is_root_token());

        let setup_token = SgfToken::from_pair("AW", "cd");
        assert!(!setup_token.is_game_info_token());
        assert!(setup_token.is_setup_token());
        assert!(!setup_token.is_root_token());

        let setup_token = SgfToken::from_pair("AE", "cd");
        assert!(!setup_token.is_game_info_token());
        assert!(setup_token.is_setup_token());
        assert!(!setup_token.is_root_token());

        let game_info_token = SgfToken::from_pair("RE", "W+T");
        assert!(game_info_token.is_game_info_token());
        assert!(!game_info_token.is_setup_token());
        assert!(!game_info_token.is_root_token());
    }

    #[test]
    fn can_parse_move_tokens() {
        let token = SgfToken::from_pair("B", "aa");
        assert_eq!(
            token,
            SgfToken::Move {
                color: Color::Black,
                action: Move(1, 1),
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "B[aa]");

        let token = SgfToken::from_pair("W", "kk");
        assert_eq!(
            token,
            SgfToken::Move {
                color: Color::White,
                action: Move(11, 11),
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "W[kk]");

        let token = SgfToken::from_pair("W", "tt");
        assert_eq!(
            token,
            SgfToken::Move {
                color: Color::White,
                action: Action::Pass
            }
        );
    }

    #[test]
    fn can_parse_uppercase_move_tokens() {
        let token = SgfToken::from_pair("B", "AA");
        assert_eq!(
            token,
            SgfToken::Move {
                color: Color::Black,
                action: Move(27, 27),
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "B[AA]");

        let token = SgfToken::from_pair("W", "KK");
        assert_eq!(
            token,
            SgfToken::Move {
                color: Color::White,
                action: Move(37, 37),
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "W[KK]");
    }

    #[test]
    fn can_parse_result_tokens() {
        assert_eq!(
            SgfToken::from_pair("RE", "B+R"),
            SgfToken::Result(Outcome::WinnerByResign(Color::Black))
        );
        assert_eq!(
            SgfToken::from_pair("RE", "B+Resign"),
            SgfToken::Result(Outcome::WinnerByResign(Color::Black))
        );
        assert_eq!(
            SgfToken::from_pair("RE", "B+35.0"),
            SgfToken::Result(Outcome::WinnerByPoints(Color::Black, 35.0))
        );
        assert_eq!(
            SgfToken::from_pair("RE", "W+R"),
            SgfToken::Result(Outcome::WinnerByResign(Color::White))
        );
        assert_eq!(
            SgfToken::from_pair("RE", "W+55.5"),
            SgfToken::Result(Outcome::WinnerByPoints(Color::White, 55.5))
        );
        assert_eq!(
            SgfToken::from_pair("RE", "W+T"),
            SgfToken::Result(Outcome::WinnerByTime(Color::White))
        );
        assert_eq!(
            SgfToken::from_pair("RE", "W+Time"),
            SgfToken::Result(Outcome::WinnerByTime(Color::White))
        );
        assert_eq!(
            SgfToken::from_pair("RE", "Draw"),
            SgfToken::Result(Outcome::Draw)
        );
        assert_eq!(
            SgfToken::from_pair("RE", "W+F"),
            SgfToken::Result(Outcome::WinnerByForfeit(Color::White))
        );
        assert_eq!(
            SgfToken::from_pair("RE", "B+Forfeit"),
            SgfToken::Result(Outcome::WinnerByForfeit(Color::Black))
        );
        assert_eq!(
            SgfToken::from_pair("RE", "unknown"),
            SgfToken::Result(Outcome::Unknown("unknown".to_string()))
        );
    }

    #[test]
    fn can_parse_ru_token() {
        assert_eq!(
            SgfToken::from_pair("RU", "Japanese"),
            SgfToken::Rule(RuleSet::Japanese)
        );
        assert_eq!(
            SgfToken::from_pair("RU", "AGA"),
            SgfToken::Rule(RuleSet::AGA)
        );
        assert_eq!(
            SgfToken::from_pair("RU", "Chinese"),
            SgfToken::Rule(RuleSet::Chinese)
        );
        assert_eq!(SgfToken::from_pair("RU", "NZ"), SgfToken::Rule(RuleSet::NZ));
        assert_eq!(
            SgfToken::from_pair("RU", "TEST"),
            SgfToken::Rule(RuleSet::Unknown("TEST".to_owned()))
        );
        assert_eq!(
            SgfToken::from_pair("RU", "GOE"),
            SgfToken::Rule(RuleSet::GOE)
        );
    }

    #[test]
    fn can_parse_time_tokens() {
        let token = SgfToken::from_pair("BL", "1234");
        assert_eq!(
            token,
            SgfToken::Time {
                color: Color::Black,
                time: 1234,
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "BL[1234]");

        let token = SgfToken::from_pair("WL", "34");
        assert_eq!(
            token,
            SgfToken::Time {
                color: Color::White,
                time: 34,
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "WL[34]");
    }

    #[test]
    fn can_parse_name_tokens() {
        let token = SgfToken::from_pair("PB", "Honinbo Shusai");
        assert_eq!(
            token,
            SgfToken::PlayerName {
                color: Color::Black,
                name: "Honinbo Shusai".to_string(),
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "PB[Honinbo Shusai]");

        let token = SgfToken::from_pair("PW", "Cho Chikun");
        assert_eq!(
            token,
            SgfToken::PlayerName {
                color: Color::White,
                name: "Cho Chikun".to_string(),
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "PW[Cho Chikun]");
    }

    #[test]
    fn can_parse_rank_tokens() {
        let token = SgfToken::from_pair("BR", "3p");
        assert_eq!(
            token,
            SgfToken::PlayerRank {
                color: Color::Black,
                rank: "3p".to_string(),
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "BR[3p]");

        let token = SgfToken::from_pair("WR", "5 kyu");
        assert_eq!(
            token,
            SgfToken::PlayerRank {
                color: Color::White,
                rank: "5 kyu".to_string(),
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "WR[5 kyu]");
    }

    #[test]
    fn can_parse_komi_tokens() {
        let token = SgfToken::from_pair("KM", "4.5");
        assert_eq!(token, SgfToken::Komi(4.5));
        let string_token: String = token.into();
        assert_eq!(string_token, "KM[4.5]");
    }

    #[test]
    fn can_parse_size_tokens() {
        let token = SgfToken::from_pair("SZ", "19");
        assert_eq!(token, SgfToken::Size(19, 19));
        let string_token: String = token.into();
        assert_eq!(string_token, "SZ[19]");
    }

    #[test]
    fn can_parse_size_token_with_two_values() {
        let token = SgfToken::from_pair("SZ", "15:17");
        assert_eq!(token, SgfToken::Size(15, 17));
        let string_token: String = token.into();
        assert_eq!(string_token, "SZ[15:17]");
    }

    #[test]
    fn can_parse_time_limit_tokens() {
        let token = SgfToken::from_pair("TM", "1234");
        assert_eq!(token, SgfToken::TimeLimit(1234));
        let string_token: String = token.into();
        assert_eq!(string_token, "TM[1234]");
    }

    #[test]
    fn can_parse_event_tokens() {
        let token = SgfToken::from_pair("EV", "event");
        assert_eq!(token, SgfToken::Event("event".to_string()));
        let string_token: String = token.into();
        assert_eq!(string_token, "EV[event]");
    }

    #[test]
    fn can_parse_comment_tokens() {
        let token = SgfToken::from_pair("C", "comment");
        assert_eq!(token, SgfToken::Comment("comment".to_string()));
        let string_token: String = token.into();
        assert_eq!(string_token, "C[comment]");
    }

    #[test]
    fn can_parse_comment_token_with_escpaed_chars() {
        let token = SgfToken::from_pair("C", "a [wrapped\\] comment");
        assert_eq!(
            token,
            SgfToken::Comment("a [wrapped\\] comment".to_string())
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "C[a [wrapped\\] comment]");
    }

    #[test]
    fn can_parse_game_name_tokens() {
        let token = SgfToken::from_pair("GN", "game name");
        assert_eq!(token, SgfToken::GameName("game name".to_string()));
        let string_token: String = token.into();
        assert_eq!(string_token, "GN[game name]");
    }

    #[test]
    fn can_parse_node_name_tokens() {
        let token = SgfToken::from_pair("N", "node name");
        assert_eq!(token, SgfToken::NodeName("node name".to_string()));
        let string_token: String = token.into();
        assert_eq!(string_token, "N[node name]");
    }

    #[test]
    fn can_parse_copyright_tokens() {
        let token = SgfToken::from_pair("CP", "copyright");
        assert_eq!(token, SgfToken::Copyright("copyright".to_string()));
        let string_token: String = token.into();
        assert_eq!(string_token, "CP[copyright]");
    }

    #[test]
    fn can_parse_date_tokens() {
        let token = SgfToken::from_pair("DT", "2019-02-02");
        assert_eq!(token, SgfToken::Date("2019-02-02".to_string()));
        let string_token: String = token.into();
        assert_eq!(string_token, "DT[2019-02-02]");
    }

    #[test]
    fn can_parse_place_tokens() {
        let token = SgfToken::from_pair("PC", "place");
        assert_eq!(token, SgfToken::Place("place".to_string()));
        let string_token: String = token.into();
        assert_eq!(string_token, "PC[place]");
    }

    #[test]
    fn can_parse_mark_x_tokens() {
        let token = SgfToken::from_pair("MA", "aa");
        assert_eq!(
            token,
            SgfToken::Cross {
                coordinates: vec![(1, 1)]
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "MA[aa]");

        // Add test for multiple coordinates
        let token = SgfToken::from_pair("MA", "fh:fj");
        assert_eq!(
            token,
            SgfToken::Cross {
                coordinates: vec![(6, 8), (6, 9), (6, 10)]
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "MA[fh:fj]");
    }

    #[test]
    fn can_parse_mark_circle_tokens() {
        let token = SgfToken::from_pair("CR", "aa");
        assert_eq!(
            token,
            SgfToken::Circle {
                coordinates: vec![(1, 1)]
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "CR[aa]");

        // Add test for multiple coordinates
        let token = SgfToken::from_pair("CR", "pd:pf");
        assert_eq!(
            token,
            SgfToken::Circle {
                coordinates: vec![(16, 4), (16, 5), (16, 6)]
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "CR[pd:pf]");
    }

    #[test]
    fn can_parse_mark_triangle_tokens() {
        let token = SgfToken::from_pair("TR", "aa");
        assert_eq!(
            token,
            SgfToken::Triangle {
                coordinates: vec![(1, 1)]
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "TR[aa]");

        let token = SgfToken::from_pair("TR", "pd:pf");
        assert_eq!(
            token,
            SgfToken::Triangle {
                coordinates: vec![(16, 4), (16, 5), (16, 6)]
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "TR[pd:pf]");
    }

    #[test]
    fn can_parse_mark_square_tokens() {
        let token = SgfToken::from_pair("SQ", "aa");
        assert_eq!(
            token,
            SgfToken::Square {
                coordinates: vec![(1, 1)]
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "SQ[aa]");

        let token = SgfToken::from_pair("SQ", "pd:pf");
        assert_eq!(
            token,
            SgfToken::Square {
                coordinates: vec![(16, 4), (16, 5), (16, 6)]
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "SQ[pd:pf]");
    }

    #[test]
    fn can_parse_selected_tokens() {
        let token = SgfToken::from_pair("SL", "aa");
        assert_eq!(
            token,
            SgfToken::Selected {
                coordinates: vec![(1, 1)]
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "SL[aa]");

        let token = SgfToken::from_pair("SL", "pd:pf");
        assert_eq!(
            token,
            SgfToken::Selected {
                coordinates: vec![(16, 4), (16, 5), (16, 6)]
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "SL[pd:pf]");
    }

    #[test]
    fn can_parse_territory_black_tokens() {
        let token = SgfToken::from_pair("TB", "aa");
        assert_eq!(
            token,
            SgfToken::TerritoryBlack {
                coordinates: vec![(1, 1)]
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "TB[aa]");

        let token = SgfToken::from_pair("TB", "pd:pf");
        assert_eq!(
            token,
            SgfToken::TerritoryBlack {
                coordinates: vec![(16, 4), (16, 5), (16, 6)]
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "TB[pd:pf]");
    }

    #[test]
    fn can_parse_territory_white_tokens() {
        let token = SgfToken::from_pair("TW", "aa");
        assert_eq!(
            token,
            SgfToken::TerritoryWhite {
                coordinates: vec![(1, 1)]
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "TW[aa]");

        let token = SgfToken::from_pair("TW", "pd:pf");
        assert_eq!(
            token,
            SgfToken::TerritoryWhite {
                coordinates: vec![(16, 4), (16, 5), (16, 6)]
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "TW[pd:pf]");
    }

    #[test]
    fn can_parse_dimpoints_tokens() {
        let token = SgfToken::from_pair("DD", "aa");
        assert_eq!(
            token,
            SgfToken::DimPoints {
                coordinates: vec![(1, 1)]
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "DD[aa]");

        let token = SgfToken::from_pair("DD", "pd:pf");
        assert_eq!(
            token,
            SgfToken::DimPoints {
                coordinates: vec![(16, 4), (16, 5), (16, 6)]
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "DD[pd:pf]");
    }

    #[test]
    fn can_parse_mark_label_tokens() {
        let token = SgfToken::from_pair("LB", "kk:foo");
        assert_eq!(
            token,
            SgfToken::Label {
                label: "foo".to_string(),
                coordinate: (11, 11),
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "LB[kk:foo]");
    }

    #[test]
    fn can_parse_game_token() {
        assert_eq!(SgfToken::from_pair("GM", "1"), SgfToken::Game(Game::Go));
        assert_eq!(
            SgfToken::from_pair("GM", "2"),
            SgfToken::Game(Game::Other(2))
        );
        assert_eq!(
            SgfToken::from_pair("GM", "error"),
            SgfToken::Invalid(("GM".to_string(), "error".to_string()))
        );
        let token = SgfToken::from_pair("GM", "1");
        let string_token: String = token.into();
        assert_eq!(string_token, "GM[1]");
    }

    #[test]
    fn can_parse_handicap_token() {
        assert_eq!(SgfToken::from_pair("HA", "3"), SgfToken::Handicap(3));
        assert_eq!(SgfToken::from_pair("HA", "0"), SgfToken::Handicap(0));
        assert_eq!(SgfToken::from_pair("HA", "999"), SgfToken::Handicap(999))
    }

    #[test]
    fn can_parse_add_tokens() {
        let token = SgfToken::from_pair("AB", "aa");
        assert_eq!(
            token,
            SgfToken::Add {
                color: Color::Black,
                coordinates: vec![(1, 1)],
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "AB[aa]");

        let token = SgfToken::from_pair("AW", "kk");
        assert_eq!(
            token,
            SgfToken::Add {
                color: Color::White,
                coordinates: vec![(11, 11)],
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "AW[kk]");
    }

    #[test]
    fn can_parse_add_compressed_tokens() {
        let token = SgfToken::from_pair("AB", "do:gq");
        assert_eq!(
            token,
            SgfToken::Add {
                color: Color::Black,
                coordinates: vec![
                    (4, 15),
                    (4, 16),
                    (4, 17),
                    (5, 15),
                    (5, 16),
                    (5, 17),
                    (6, 15),
                    (6, 16),
                    (6, 17),
                    (7, 15),
                    (7, 16),
                    (7, 17)
                ],
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "AB[do:gq]");

        let token = SgfToken::from_pair("AW", "kn:lq");
        assert_eq!(
            token,
            SgfToken::Add {
                color: Color::White,
                coordinates: vec![
                    (11, 14),
                    (11, 15),
                    (11, 16),
                    (11, 17),
                    (12, 14),
                    (12, 15),
                    (12, 16),
                    (12, 17)
                ],
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "AW[kn:lq]");
    }

    #[test]
    fn can_parse_add_empty_tokens() {
        let token = SgfToken::from_pair("AE", "pn:pq");
        assert_eq!(
            token,
            SgfToken::AddEmpty {
                coordinates: vec![(16, 14), (16, 15), (16, 16), (16, 17)]
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "AE[pn:pq]");

        let token = SgfToken::from_pair("AE", "aa");
        assert_eq!(
            token,
            SgfToken::AddEmpty {
                coordinates: vec![(1, 1)]
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "AE[aa]");
    }

    #[test]
    fn can_parse_move_number_tokens() {
        let token = SgfToken::from_pair("MN", "5");
        assert_eq!(token, SgfToken::MoveNumber(5));
        let string_token: String = token.into();
        assert_eq!(string_token, "MN[5]");
    }

    #[test]
    fn can_parse_user_tokens() {
        let token = SgfToken::from_pair("US", "test_user");
        assert_eq!(token, SgfToken::User(String::from("test_user")));
        let string_token: String = token.into();
        assert_eq!(string_token, "US[test_user]");
    }

    #[test]
    fn can_parse_source_tokens() {
        let token = SgfToken::from_pair("SO", "from somewhere");
        assert_eq!(token, SgfToken::Source(String::from("from somewhere")));
        let string_token: String = token.into();
        assert_eq!(string_token, "SO[from somewhere]");
    }

    #[test]
    fn can_parse_game_comment_tokens() {
        let token = SgfToken::from_pair("GC", "game comments");
        assert_eq!(token, SgfToken::GameComment(String::from("game comments")));
        let string_token: String = token.into();
        assert_eq!(string_token, "GC[game comments]");
    }

    #[test]
    fn can_parse_charset_token() {
        assert_eq!(
            SgfToken::from_pair("CA", "UTF-8"),
            SgfToken::Charset(Encoding::UTF8)
        );
        assert_eq!(
            SgfToken::from_pair("CA", "ISO-8859-1"),
            SgfToken::Charset(Encoding::Other("ISO-8859-1".to_string()))
        );
        let token = SgfToken::from_pair("CA", "UTF-8");
        let string_token: String = token.into();
        assert_eq!(string_token, "CA[UTF-8]");
    }

    #[test]
    fn can_parse_overtime_move_tokens() {
        let token_black = SgfToken::from_pair("OB", "5");
        assert_eq!(
            token_black,
            SgfToken::MovesRemaining {
                color: Color::Black,
                moves: 5
            }
        );
        let string_black: String = token_black.into();
        assert_eq!(string_black, "OB[5]");

        let token_white = SgfToken::from_pair("OW", "23");
        assert_eq!(
            token_white,
            SgfToken::MovesRemaining {
                color: Color::White,
                moves: 23
            }
        );
        let string_white: String = token_white.into();
        assert_eq!(string_white, "OW[23]");
    }

    #[test]
    fn can_parse_application_token() {
        let token = SgfToken::from_pair("AP", "CGoban:1.6.2");
        assert_eq!(
            token,
            SgfToken::Application {
                name: "CGoban".to_string(),
                version: "1.6.2".to_string(),
            }
        );
        let string_token: String = token.into();
        assert_eq!(string_token, "AP[CGoban:1.6.2]");
    }

    #[test]
    fn can_parse_overtime_token() {
        let token = SgfToken::from_pair("OT", "15/300 Canadian");
        assert_eq!(token, SgfToken::Overtime("15/300 Canadian".to_string()));
        let string_token: String = token.into();
        assert_eq!(string_token, "OT[15/300 Canadian]");
    }

    #[test]
    fn can_parse_variation_display_token() {
        let token_3 = SgfToken::from_pair("ST", "3");
        assert_eq!(
            token_3,
            SgfToken::VariationDisplay {
                nodes: DisplayNodes::Siblings,
                on_board_display: false
            }
        );
        let string_token_3: String = token_3.into();
        assert_eq!(string_token_3, "ST[3]");

        let token_2 = SgfToken::from_pair("ST", "2");
        assert_eq!(
            token_2,
            SgfToken::VariationDisplay {
                nodes: DisplayNodes::Children,
                on_board_display: false
            }
        );
        let string_token_2: String = token_2.into();
        assert_eq!(string_token_2, "ST[2]");

        let token_1 = SgfToken::from_pair("ST", "1");
        assert_eq!(
            token_1,
            SgfToken::VariationDisplay {
                nodes: DisplayNodes::Siblings,
                on_board_display: true
            }
        );
        let string_token_1: String = token_1.into();
        assert_eq!(string_token_1, "ST[1]");

        let token_0 = SgfToken::from_pair("ST", "0");
        assert_eq!(
            token_0,
            SgfToken::VariationDisplay {
                nodes: DisplayNodes::Children,
                on_board_display: true
            }
        );
        let string_token_0: String = token_0.into();
        assert_eq!(string_token_0, "ST[0]");
    }

    #[test]
    fn can_parse_fileformat_token() {
        let token = SgfToken::from_pair("FF", "3");
        assert_eq!(token, SgfToken::FileFormat(3));
        let string_token: String = token.into();
        assert_eq!(string_token, "FF[3]");

        let token = SgfToken::from_pair("FF", "5");
        assert_eq!(
            token,
            SgfToken::Invalid(("FF".to_string(), "5".to_string()))
        );
    }
}
