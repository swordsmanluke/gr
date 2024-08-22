use gr_reviews::{MERGE_USAGE, REVIEW_USAGE};
use crate::gr::init;
use crate::gr::log;
use crate::gr::r#move::MOVE_USAGE;

const USAGE: &str = "Usage: gq <command> [<args>]

gq is a command line interface for managing stacked commits and code reviews.
type 'gq help <command>' for detailed help with a specific command.

General Commands:
    init            Configure (or reconfigure) gq
    help            Display this help message
    log             Display the commit log

Branch Commands:
    create, bc     Create a new branch
    switch, bco    Switch to a branch
    commit, cc     Commit changes
    submit         Submit current branch (and parents) for code review
    sync           Sync from remote (recursive pull and rebase)

Review Commands:
    reviews, rv    List open reviews
    merge          Merge approved reviews

Stack Commands:
    top, bt        Move to the top of the stack
    bottom, bb     Move to the bottom of the stack
    up, bu         Move up in the stack
    down, bd       Move down in the stack

Git Commands:
    Any keywords not listed above will be passed directly to git.
";

const BRANCH_USAGE: [&str; 5] = [
// Create
"gq <create|bc> [branch name]

Creates a new branch. If no name is provided, you will be prompted for one.",

// Switch
"gq <switch|bco> [branch name]

Switch to the specified branch. If no name is provided, you will be prompted to select one.",

// Commit
"gq <commit|cc>

Commit changes. Follows 'git commit' syntax.
e.g. 'gq cc -m \"My commit message\"' or 'gq cc --amend' work.",

// Submit
"gq <submit>

Submit the current branch (and parents) for code review.
Syncs the current stack with remote before submitting.

This will force-update the remote if there are any conflicts.",

// Sync
"gq <sync>

Sync from remote (recursive pull and rebase).

Sync recursively pulls the latest changes from remote and local branches in order
to ensure that all branches in the current stack are up-to-date."
];

pub fn show_usage() {
    println!("{}", USAGE);
}

pub fn show_help(cmd: &str) {
    match cmd {
        // General
        "help" => println!("You already got it, chief."),
        "init" => println!("{}", init::USAGE),
        "log" => println!("{}", log::USAGE),

        // Branch
        "create" | "bc" => println!("{}", BRANCH_USAGE[0]),
        "switch" | "bco" => println!("{}", BRANCH_USAGE[1]),
        "commit" | "cc" => println!("{}", BRANCH_USAGE[2]),
        "submit" => println!("{}", BRANCH_USAGE[3]),
        "sync" => println!("{}", BRANCH_USAGE[4]),

        // Review
        "reviews" | "rv" => println!("{}", REVIEW_USAGE),
        "merge" => println!("{}", MERGE_USAGE),

        // Stack
        "up"     | "bu" => println!("{}", MOVE_USAGE[0]),
        "down"   | "bd" => println!("{}", MOVE_USAGE[1]),
        "bottom" | "bb" => println!("{}", MOVE_USAGE[2]),
        "top"    | "bt" => println!("{}", MOVE_USAGE[3]),

        // Git
        _ => println!("TODO"),
    }
}
