# TODO
This is a list of features I need to add to GR to help 
me keep track of what needs doing.

The list is in alphabetical order, not order of importance
so features may get done in a seemingly incoherent manner.

## Next Steps
- [ ] Add Help menu
- [ ] Async widget support for status tracking
- [ ] Find slowdowns in GQ
- [ ] Add CircleCI support

## Port GQ Commands
GQ is the previous version of this application. These commands
were already implemented and need to be ported to Rust.

- [x] add_branch: Create a new `git` branch
- [x] bottom: Move to the bottom of the commit stack
- [x] checkout: Switch to a different branch
- [x] commit: Save your latest changes
- [x] down: Move down one branch in the stack
- [x] init: Initialize a new project
- [x] log: View the commit stack
- [x] merge: Merge a stack of approved PRs 
  - [ ] Check Mergeability (see Connect with Github below)
  - [x] recursively tell GH to merge PRs
- [ ] move: Change the parent of the current branch and rebase
- [x] review: View open PRs for a given stack
  - [x] Reformat output, it's kinda ugly
- [ ] split: Divide commits in a branch into N branches
- [x] submit: Recursively open PRs for the current stack
  - [x] Edit commit title
  - [ ] Edit commit message
  - [x] Push to remote
  - [x] Create review
- [x] sync: Download remote contents, then recursively pull-rebase on parent branch(es)
  - [x] Auto-remove merged branches
  - [x] Reparent descendants after deleting merged branches
- [x] top: Move to the top of the stack
- [x] up: Move one branch up the stack


## Improve UI
The UI is inconsistent and bad right now. Let's polish this up a bit!

- [ ] Log
  - [ ] colors are wrong
  - [ ] stack order is inconsistent
  - [ ] stack moves right when it doesn't have to
       - [ ] "left"-only branches still indent instead of creating pipes
       - NOTE: since we have tries, not BSTs, we need to differentiate between "root", "fork", "pipe" and "leaf"
  - [ ] consider splitting output by "stack"

- [ ] General "Task progress" widget:  <name> - <spinner> -> <name> - <status>

- [ ] Submit
  - [ ] Gather User's desired operations, _then_ show 'submit' progress per branch

- [ ] Merge
  - [ ] Gather User's desired operations, _then_ show 'merge' progress per branch
  - [ ] Sync after merging

- [ ] Stack / Feature Support
  - [ ] feature tracking - link branches by prefix
  - [ ] prefix format: /<name>/<feature>/#-<branch name>
  - [ ] gq 'new' - creates a new branch prefix for a feature

- [ ] General
  - [x] Commit to Ratatui or try something else
  - [ ] Color scheme
  - [ ] Unified "widgets" for each main type in GQ
    - [ ] Branch (w/w-out commits)
    - [ ] commit shortline
    - [ ] confirmation prompt
    - [ ] input prompt
    - [ ] editor launcher

## Internals
General improvements / refactors to consider

- [x] Bug: Editing Commit Title (during merge) doesn't display a cursor

- [ ] Merge order
  - [ ] Bottom Up (1 merge to main per PR)
  - [ ] Top Down (1 merge to main, squashing PRs down as we go)

- [ ] Generic "tool" system to accelerate adding new backends, etc
  - [ ] Supported tool types (vcs / code review / ci-cd)
  - [ ] Supported tools per type
    - [x] VCS: git
    - [ ] CR:  github
    - [ ] CR:  gitlab
    - [ ] CICD: CircleCI
    - [ ] CICD: Jenkins
  - [ ] Support "external script" types
    - [ ] script must implement specific CLI args and return JSON
        - e.g. a CR tool script wrapper must implement "create, merge, review"
            and return results in JSON format


## Build out Git support
To enable many of the features up there, we will need to ask
`git` to help us do stuff. 

- [x] current branch name
- [x] parent of given branch
  - [x] local
  - [x] remote
- [x] create a commit
- [x] add files to commit
- [x] push to remote
- [x] pull from remote
- [x] rebase
- [x] check if currently _in_ a git repo
- [x] list all local branches
- [x] call git with arbitrary arguments 
  - [x] ...and fork() to it 

## Connect with Github
To support opening/updating/merging PRs on behalf of the User,
we want to support connecting to github (others in future, maybe!)

- [x] authenticate to github
- [x] open a PR
- [x] list all PRs
- [x] retrieve a PR's information
- [ ] check a PR's mergeability 
  - [x] wait to ensure there are no conflicts
  - [ ] required checks passed
  - [ ] required # of approvals met
- [x] merge a PR

## Connect with CircleCI
To support CI/CD approvals/tracking and suchlike, we want
to connect to CircleCI (others in future!) and track deploy status

- [ ] auth with Circle
- [ ] list pipelines for given branch
- [ ] check given pipeline status
- [ ] get check statuses
- [ ] send approval

## Other

- [ ] Redo the command verbs. Switch either to override git-cmds for familiarity... or break clean.
- [ ] Overhaul error message support
