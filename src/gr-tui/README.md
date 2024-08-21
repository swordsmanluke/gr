# GR-TUI

A text-UI widgets/rendering library for GR, built atop Ratatui.

## Design


### Frames
The "frame" is where our UI gets rendered. Frames have an X,Y position and a W,H size.

We want our UI to appear to be in-line with other console output, even as it remains
interactive. (GR does not expect to have its output piped to other programs - interactive 
mode only.) 

To this end, we 
1. determine the console size X,Y
2. scroll down N lines
3. render our frame X by N at the bottom of the console

### Widgets
GR has multiple widgets, divided into a few general classes:

1. Input requests
   * Things like selections or line editing are in here. Any time we need 
     interactive input from the User, we want one of these. Compared to
     other widget types, Input request widgets take over the frame, then
     disappear entirely once the input has been received, restoring the 
     previous state. They run in essentially a mock alt-terminal mode.
2. "Live" reporting
    * These widgets contain things like Spinners, or otherwise live-updating
      values. They are expected to change their contents until some task is
      terminated, at which point they become "Static" widgets.
3. "Static" reporting
    * Finally, "Static" widgets are simple, fire and forget output. These are
      essentially creating a mock "println!" for displaying output to the User
      which is going to be unchanging. The contents of Static reporting widgets
      will persist and be scrolled up the screen, persisting after GR exits.

### Input Widgets

**Select**

This is a general "choose X of Y" style of widget. It prompts the User to select 
zero or more options from a list of decisions. 

- prompt: a message to display to the User
- options: 1 or more options for the User to decide between
- multiselect: true = allow the user to choose more than one option
- allow_none: true = allow the user _not_ to select an option

**Confirm**

A Select that requires the user to choose between "Yes"/"No"

**ChooseOne**

A Select that provides a choice of one of a number of options

**Prompt**

A line editor - request a line of plain input from the User and accept on "Enter"

### Live widgets

Live widgets consist of UI and an async Channel Receiver.
The Task associated with the UI accepts the Channel's Sender and uses it to
send status updates to the UI object.

**Submit**

Display the state of an ongoing stack submission.
Remote branch creation; final push state; etc.

```
branch 3 - :checkmark: 
    PR: http://example.com/pr/1234
branch 2 - :checkmark:
    PR: http://example.com/pr/4321
branch 1 - :red_x:
    Conflict with main 
main - :yellow_bang: 
    Behind origin/main by 2

Conflicts with origin - run 'gq sync' 
```

**Merge**

Display the state of an ongoing stack merge. Merge state, etc

```
branch 4 - :red_x:
    :red_x: PR not approved
branch 3 - :clock: 
    :clock: Waiting for 'branch 2'
branch 2 - :spinner:
    :checkmark: branch 1 merged
    :checkmark: Rebased onto main
    :spinner:   Running checks
branch 1 - :checkmark:
    :checkmark: Merged
```

