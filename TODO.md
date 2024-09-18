# TODO
This is a list of features I need to add to GR to help 
me keep track of what needs doing.

The list is in alphabetical order, not order of importance
so features may get done in a seemingly incoherent manner.

## BUGS
P1: Showstopper - the tool cannot be used
P2: Broken functionality - One or more functions do not succeed
P3: Breaks expectations - A feature does something unexpected

- [ ] **P2** Sync - doesn't remove merged branches with multiple commits
      Create branch w 2+ commits
      Squash remote branch to main
      Sync does not detect merged commit == branch commits

### Known Issues
- [ ] **P3** Init - Fails on _brand-new_ git repos.
  - Until the first 'Initial Commit' is committed, the branch list is empty, breaking GQ


## Next Steps
- [ ] Squash support
- [ ] Async widget support for status tracking
- [ ] Add CircleCI support
- [ ] Find slowdowns in GQ

## Port GQ Commands
GQ is the previous version of this application. These commands
were already implemented and need to be ported to Rust.

- [x] merge: Merge a stack of approved PRs 
  - [ ] Check Mergeability (see Connect with Github below)
  - [x] recursively tell GH to merge PRs
- [ ] move: Change the parent of the current branch and rebase
- [ ] squash: Merge N branches into a single branch


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
  - [ ] Color scheme
  - [ ] Unified "widgets" for each main type in GQ
    - [ ] Branch (w/w-out commits)
    - [ ] commit shortline
    - [ ] confirmation prompt
    - [ ] input prompt
    - [ ] editor launcher

## Internals
General improvements / refactors to consider

- [ ] Imp: Merge should delete merged branches
- [ ] Bug: Merge should rebase remote branches onto remote branch before merging
  - Github _can_ do this automatically, but it doesn't have to and the bottom-up merge order breaks if we don't do this.

- [ ] Merge order
  - [ ] Bottom Up (1 merge to main per PR)
  - [ ] Top Down (1 merge to main, squashing PRs down as we go)

- [ ] Generic "tool" system to accelerate adding new backends, etc
  - [ ] Supported tool types (vcs / code review / ci-cd)
  - [ ] Supported tools per type
    - [x] VCS: git
    - [x] CR:  github
    - [ ] CR:  gitea
    - [ ] CICD: CircleCI
    - [ ] CICD: Jenkins
  - [ ] Support "external script" types
    - [ ] script must implement specific CLI args and return JSON
        - e.g. a CR tool script wrapper must implement "create, merge, review"
            and return results in JSON format


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

