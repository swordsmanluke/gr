# GR

A Rust-based tool for local, upstream (theoretically) agnostic [Stacked diffs](https://newsletter.pragmaticengineer.com/p/stacked-diffs) workflow.

## Quick Start:


### Configure GR for your repo:
```bash
$ cd my_git_repo
$ gr init
```

### Create a new stack on your current branch
```bash
$ gr bc my-new-branch
```

### Move up or down the stack of diffs
```bash
$ gr up
$ gr down
```

### View the current stack of diffs
```bash
$ gr log
```

### Submit the current stack for review
```bash
$ gr submit
```

### Merge the current stack to main
```bash
$ gr merge
```
