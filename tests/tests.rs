mod helpers;

use helpers as h;

#[test]
fn pipeline_helper() {
    let actual = h::pipeline(
        r#"
            open los_tres_amigos.txt
            | from-csv
            | get rusty_luck
            | str --to-int
            | sum
            | echo "$it"
        "#);

    assert_eq!(actual, r#"open los_tres_amigos.txt | from-csv | get rusty_luck | str --to-int | sum | echo "$it""#);
}

#[test]
fn external_num() {
    let actual = nu!(
        cwd: "tests/fixtures/formats",
        "open sgml_description.json | get glossary.GlossDiv.GlossList.GlossEntry.Height | echo $it"
    );

    assert_eq!(actual, "10");
}

#[test]
fn external_has_correct_quotes() {
    let actual = nu!(
        cwd: ".",
        r#"echo "hello world""#
    );

    let actual = h::normalize_string(&actual);

    assert_eq!(actual, r#""hello world""#);
}

#[test]
fn add_plugin() {
    let actual = nu!(
        cwd: "tests/fixtures/formats", h::pipeline(
        r#"
            open cargo_sample.toml
            | add dev-dependencies.newdep "1"
            | get dev-dependencies.newdep
            | echo $it
        "#
    ));

    assert_eq!(actual, "1");
}

#[test]
fn edit_plugin() {
    let actual = nu!(
        cwd: "tests/fixtures/formats", h::pipeline(
        r#"
            open cargo_sample.toml
            | edit dev-dependencies.pretty_assertions "7"
            | get dev-dependencies.pretty_assertions
            | echo $it
        "#
    ));

    assert_eq!(actual, "7");
}
