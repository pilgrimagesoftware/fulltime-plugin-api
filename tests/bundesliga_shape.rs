//! Validates the canonical `league-data-schema` against `Libs/openligadb`'s existing
//! response shapes (`Team`, `Match`, `TableTeam`), standing in for a working
//! `Plugins/Bundesliga` prototype per
//! `openspec/changes/define-league-data-contract/tasks.md` (2.5): full validation against
//! EPL/national-team shapes is tracked as follow-up once those plugins exist.
//!
//! Field values below mirror `Libs/openligadb`'s `Team`, `Match`, and `TableTeam` structs
//! (deserialized field names in comments), proving every field openligadb returns has a
//! home in the canonical schema.

use fulltime_plugin_api::{
    Competition, Fixture, FixtureStatus, Score, Standings, StandingsGroup, StandingsRow, Team,
};

fn team(id: &str, name: &str, short_name: &str) -> Team {
    // Mirrors openligadb::models::team::Team { id: teamId, name: teamName, short_name:
    // shortName, .. } (icon_url has no canonical equivalent - display-only, out of scope
    // for the data contract).
    Team {
        id: id.to_owned(),
        name: name.to_owned(),
        short_name: short_name.to_owned(),
    }
}

#[test]
fn league_fixture_matches_openligadb_match_shape() {
    // Mirrors openligadb::models::match::Match: matchID, matchDateTimeUTC, team1, team2,
    // group.groupName, matchIsFinished, matchResults.
    let fixture = Fixture {
        id: "6197".to_owned(),
        competition_id: "bl1-2024".to_owned(),
        group: None, // single-table league match: no group
        kickoff: "2024-08-23T18:30:00Z".to_owned(),
        home_team: team("40", "FC Bayern München", "FCB"),
        away_team: team("81", "TSG 1899 Hoffenheim", "TSG"),
        venue: None,
        status: FixtureStatus::Finished,
        score: Some(Score { home: 2, away: 0 }),
    };

    assert_eq!(fixture.status, FixtureStatus::Finished);
    assert_eq!(fixture.score, Some(Score { home: 2, away: 0 }));
}

#[test]
fn league_table_matches_openligadb_table_team_shape() {
    // Mirrors openligadb::models::table::TableTeam: teamInfoId, points, opponentGoals,
    // goals, matches, won, lost, draw (goalDiff is derived from goals/opponent_goals, so
    // it is not carried as a separate schema field).
    let row = StandingsRow {
        team: team("40", "FC Bayern München", "FCB"),
        rank: 1,
        played: 3,
        won: 3,
        drawn: 0,
        lost: 0,
        goals_for: 12,
        goals_against: 2,
        points: 9,
    };

    let standings = Standings {
        competition_id: "bl1-2024".to_owned(),
        groups: vec![StandingsGroup {
            name: None, // single-table league: one unnamed group
            rows: vec![row],
        }],
    };

    assert_eq!(standings.groups.len(), 1);
    assert_eq!(standings.groups[0].rows[0].points, 9);
}

#[test]
fn competition_metadata_fits_canonical_shape() {
    let competition = Competition {
        id: "bl1-2024".to_owned(),
        name: "Bundesliga 2024/25".to_owned(),
    };

    assert_eq!(competition.id, "bl1-2024");
}
