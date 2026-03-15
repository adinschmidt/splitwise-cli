use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
    Yaml,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum)]
pub enum GroupType {
    Home,
    Trip,
    Couple,
    Other,
    Apartment,
    House,
}

impl GroupType {
    pub fn as_api_value(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::Trip => "trip",
            Self::Couple => "couple",
            Self::Other => "other",
            Self::Apartment => "apartment",
            Self::House => "house",
        }
    }
}

#[derive(Debug, Parser)]
#[command(
    name = "splitwise",
    version,
    about = "CLI for the Splitwise API",
    after_help = "Examples:\n  splitwise users me\n  splitwise expenses list --group-id 123\n  splitwise friends create --email ada@example.com --first-name Ada --last-name Lovelace\n  splitwise expenses create --cost 42.50 --description Dinner --group-id 123\n  splitwise expenses create --body @expense.json --json"
)]
pub struct Cli {
    #[arg(long, global = true, value_enum)]
    pub output: Option<OutputFormat>,

    #[arg(long, global = true, conflicts_with = "yaml")]
    pub json: bool,

    #[arg(long, global = true, conflicts_with = "json")]
    pub yaml: bool,

    #[arg(long, global = true)]
    pub base_url: Option<String>,

    #[arg(long, global = true)]
    pub token: Option<String>,

    #[arg(long, global = true, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[arg(long, global = true, short = 'v')]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: CommandGroup,
}

#[derive(Debug, Subcommand)]
pub enum CommandGroup {
    Users(UsersCommand),
    Groups(GroupsCommand),
    Friends(FriendsCommand),
    Expenses(ExpensesCommand),
    Comments(CommentsCommand),
    Notifications(NotificationsCommand),
    Categories(CategoriesCommand),
    Currencies(CurrenciesCommand),
}

#[derive(Debug, Args)]
pub struct UsersCommand {
    #[command(subcommand)]
    pub command: UsersSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UsersSubcommand {
    Me,
    Get {
        id: i64,
    },
    Update {
        id: i64,
        #[command(flatten)]
        fields: UserUpdateArgs,
        #[arg(long)]
        body: Option<String>,
    },
}

#[derive(Debug, Args, Default)]
pub struct UserUpdateArgs {
    #[arg(long)]
    pub first_name: Option<String>,
    #[arg(long)]
    pub last_name: Option<String>,
    #[arg(long)]
    pub email: Option<String>,
    #[arg(long)]
    pub password: Option<String>,
    #[arg(long)]
    pub locale: Option<String>,
    #[arg(long)]
    pub default_currency: Option<String>,
}

#[derive(Debug, Args)]
pub struct GroupsCommand {
    #[command(subcommand)]
    pub command: GroupsSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum GroupsSubcommand {
    List,
    Get {
        id: i64,
    },
    Create {
        #[command(flatten)]
        fields: GroupCreateArgs,
        #[arg(long)]
        body: Option<String>,
    },
    Delete {
        id: i64,
    },
    Undelete {
        id: i64,
    },
    AddUser {
        #[command(flatten)]
        fields: GroupAddUserArgs,
        #[arg(long)]
        body: Option<String>,
    },
    RemoveUser {
        #[command(flatten)]
        fields: GroupRemoveUserArgs,
        #[arg(long)]
        body: Option<String>,
    },
}

#[derive(Debug, Args, Default)]
pub struct GroupCreateArgs {
    #[arg(long)]
    pub name: Option<String>,
    #[arg(long, value_enum)]
    pub group_type: Option<GroupType>,
    #[arg(long)]
    pub simplify_by_default: bool,
    #[arg(long = "member", value_name = "FIELD=VALUE[,FIELD=VALUE...]")]
    pub members: Vec<String>,
}

#[derive(Debug, Args, Default)]
pub struct GroupAddUserArgs {
    #[arg(long)]
    pub group_id: Option<i64>,
    #[arg(long)]
    pub user_id: Option<i64>,
    #[arg(long)]
    pub first_name: Option<String>,
    #[arg(long)]
    pub last_name: Option<String>,
    #[arg(long)]
    pub email: Option<String>,
}

#[derive(Debug, Args, Default)]
pub struct GroupRemoveUserArgs {
    #[arg(long)]
    pub group_id: Option<i64>,
    #[arg(long)]
    pub user_id: Option<i64>,
}

#[derive(Debug, Args)]
pub struct FriendsCommand {
    #[command(subcommand)]
    pub command: FriendsSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum FriendsSubcommand {
    List,
    Get {
        id: i64,
    },
    Create {
        #[command(flatten)]
        fields: FriendCreateArgs,
        #[arg(long)]
        body: Option<String>,
    },
    CreateMany {
        #[arg(long = "friend", value_name = "FIELD=VALUE[,FIELD=VALUE...]")]
        friends: Vec<String>,
        #[arg(long)]
        body: Option<String>,
    },
    Delete {
        id: i64,
    },
}

#[derive(Debug, Args, Default)]
pub struct FriendCreateArgs {
    #[arg(long)]
    pub email: Option<String>,
    #[arg(long)]
    pub first_name: Option<String>,
    #[arg(long)]
    pub last_name: Option<String>,
}

#[derive(Debug, Args)]
pub struct ExpensesCommand {
    #[command(subcommand)]
    pub command: ExpensesSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ExpensesSubcommand {
    List {
        #[arg(long)]
        group_id: Option<i64>,
        #[arg(long)]
        friend_id: Option<i64>,
        #[arg(long)]
        dated_after: Option<String>,
        #[arg(long)]
        dated_before: Option<String>,
        #[arg(long)]
        updated_after: Option<String>,
        #[arg(long)]
        updated_before: Option<String>,
        #[arg(long)]
        limit: Option<i64>,
        #[arg(long)]
        offset: Option<i64>,
    },
    Get {
        id: i64,
    },
    Create {
        #[command(flatten)]
        fields: ExpenseWriteArgs,
        #[arg(long)]
        body: Option<String>,
    },
    Update {
        id: i64,
        #[command(flatten)]
        fields: ExpenseWriteArgs,
        #[arg(long)]
        body: Option<String>,
    },
    Delete {
        id: i64,
    },
    Undelete {
        id: i64,
    },
}

#[derive(Debug, Args, Default)]
pub struct ExpenseWriteArgs {
    #[arg(long)]
    pub cost: Option<String>,
    #[arg(long)]
    pub description: Option<String>,
    #[arg(long)]
    pub details: Option<String>,
    #[arg(long)]
    pub group_id: Option<i64>,
    #[arg(long)]
    pub currency_code: Option<String>,
    #[arg(long)]
    pub date: Option<String>,
    #[arg(long)]
    pub category_id: Option<i64>,
    #[arg(long)]
    pub payment: bool,
    #[arg(long = "share", value_name = "FIELD=VALUE[,FIELD=VALUE...]")]
    pub shares: Vec<String>,
}

#[derive(Debug, Args)]
pub struct CommentsCommand {
    #[command(subcommand)]
    pub command: CommentsSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CommentsSubcommand {
    List {
        #[arg(long)]
        expense_id: i64,
    },
    Create {
        #[command(flatten)]
        fields: CommentCreateArgs,
        #[arg(long)]
        body: Option<String>,
    },
    Delete {
        id: i64,
    },
}

#[derive(Debug, Args, Default)]
pub struct CommentCreateArgs {
    #[arg(long)]
    pub expense_id: Option<i64>,
    #[arg(long)]
    pub content: Option<String>,
}

#[derive(Debug, Args)]
pub struct NotificationsCommand {
    #[command(subcommand)]
    pub command: NotificationsSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum NotificationsSubcommand {
    List {
        #[arg(long)]
        limit: Option<i64>,
        #[arg(long)]
        updated_after: Option<String>,
    },
}

#[derive(Debug, Args)]
pub struct CategoriesCommand {
    #[command(subcommand)]
    pub command: ReferenceSubcommand,
}

#[derive(Debug, Args)]
pub struct CurrenciesCommand {
    #[command(subcommand)]
    pub command: ReferenceSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ReferenceSubcommand {
    List,
}
