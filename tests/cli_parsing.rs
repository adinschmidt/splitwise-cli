use clap::{CommandFactory, Parser};
use insta::assert_snapshot;

use splitwise_cli::cli::{Cli, CommandGroup, ExpensesSubcommand, FriendsSubcommand};

#[test]
fn cli_builds() {
    Cli::command().debug_assert();
}

#[test]
fn root_help_matches_snapshot() {
    let mut command = Cli::command();
    let help = command.render_long_help().to_string();
    assert_snapshot!(help, @r###"
    CLI for the Splitwise API

    Usage: splitwise [OPTIONS] <COMMAND>

    Commands:
      users          
      groups         
      friends        
      expenses       
      comments       
      notifications  
      categories     
      currencies     
      help           Print this message or the help of the given subcommand(s)

    Options:
          --output <OUTPUT>
              [possible values: table, json, yaml]

          --json
              

          --yaml
              

          --base-url <BASE_URL>
              

          --token <TOKEN>
              

          --config <FILE>
              

      -v, --verbose
              

      -h, --help
              Print help

      -V, --version
              Print version

    Examples:
      splitwise users me
      splitwise expenses list --group-id 123
      splitwise friends create --email ada@example.com --first-name Ada --last-name Lovelace
      splitwise expenses create --cost 42.50 --description Dinner --group-id 123
      splitwise expenses create --body @expense.json --json
    "###);
}

#[test]
fn parses_expense_create_typed_flags() {
    let cli = Cli::parse_from([
        "splitwise",
        "expenses",
        "create",
        "--cost",
        "42.50",
        "--description",
        "Dinner",
        "--group-id",
        "77",
        "--share",
        "user_id=1,paid_share=42.50,owed_share=21.25",
    ]);

    let CommandGroup::Expenses(command) = cli.command else {
        panic!("expected expenses command");
    };

    let ExpensesSubcommand::Create { fields, body } = command.command else {
        panic!("expected expenses create");
    };

    assert_eq!(fields.cost.as_deref(), Some("42.50"));
    assert_eq!(fields.description.as_deref(), Some("Dinner"));
    assert_eq!(fields.group_id, Some(77));
    assert_eq!(
        fields.shares,
        vec!["user_id=1,paid_share=42.50,owed_share=21.25".to_string()]
    );
    assert_eq!(body, None);
}

#[test]
fn parses_friends_create_many_specs() {
    let cli = Cli::parse_from([
        "splitwise",
        "friends",
        "create-many",
        "--friend",
        "email=ada@example.com,first_name=Ada,last_name=Lovelace",
        "--friend",
        "email=grace@example.com,first_name=Grace,last_name=Hopper",
    ]);

    let CommandGroup::Friends(command) = cli.command else {
        panic!("expected friends command");
    };

    let FriendsSubcommand::CreateMany { friends, body } = command.command else {
        panic!("expected create-many");
    };

    assert_eq!(friends.len(), 2);
    assert!(body.is_none());
}
