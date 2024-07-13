# TODO
This is a list of features I need to add to GR to help 
me keep track of what needs doing.

The list is in alphabetical order, not order of importance
so features may get done in a seemingly incoherent manner.

## Port GQ Commands
GQ is the previous version of this application. These commands
were already implemented and need to be ported to Rust.

- [x] add_branch: Create a new `git` branch
- [x] bottom: Move to the bottom of the commit stack
- [x] checkout: Switch to a different branch
- [x] commit: Save your latest changes
- [ ] config: View the configuration file
- [x] down: Move down one branch in the stack
- [x] init: Initialize a new project
- [ ] log: View the commit stack
- [x] merge: Merge a stack of approved PRs 
  - [ ] validate req'd # of approvals met
  - [ ] validate required checks passed
  - [ ] allow optional checks to fail / be in progress
  - [x] recursively tell GH to merge PRs
- [x] review: View open PRs for a given stack
  - [ ] Reformat output, it's kinda ugly
- [x] submit: Recursively open PRs for the current stack
  - [ ] Edit commit messages
  - [x] Push to remote
  - [x] Create review
- [x] sync: Download remote contents, then recursively pull-rebase on parent branch(es)
  - [ ] Auto-remove merged branches
  - [ ] Reparent descendants after deleting merged branches
- [x] top: Move to the top of the stack
- [x] up: Move one branch up the stack

## Build out Git support
To enable many of the features up there, we will need to ask
`git` to help us do stuff. 

- [x] current branch name
- [ ] parent of given branch
  - [x] local
  - [ ] remote
- [ ] create a commit
- [ ] add files to commit
- [ ] push to remote
- [ ] pull from remote
- [ ] rebase
- [x] check if currently _in_ a git repo
- [x] list all local branches
- [x] call git with arbitrary arguments 
  - [x] ...and fork() to it 

## Connect with Github
To support opening/updating/merging PRs on behalf of the User,
we want to support connecting to github (others in future, maybe!)

- [ ] authenticate to github
- [ ] open a PR
- [ ] list all PRs
- [ ] retrieve a PR's information
- [ ] check a PR's mergeability 
  - [ ] no conflicts
  - [ ] required checks passed
  - [ ] required # of approvals met
- [ ] merge a PR

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