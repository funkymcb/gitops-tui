# WIP: gitops-tui
Tui to help synchronise different gitops cluster repos

# TODOs
- [x] config file handling
- [x] read repos from config and open
    - [x] poc for a single repo
    - [x] loop over all repos
- [ ] list pretty printed commits (like: `git log --pretty=format:"%<(16)%ah: %Cgreen%h%Creset %an: %B"`)
    - [x] extend revwalk (commits) with author and date
    - [x] pretty print author
    - [x] pretty print date
- [ ] build tui layout:
    - [x] picker for commits
        - [x] list commits
        - [x] make commits selectable
        - [x] add checkbox before string
    - [x] refactor and seperate into modules
    - [ ] file picker based on stages and picked commits
        - [x] implement tui block
        - [x] add diff on toggled commit
        - [ ] toggle logic (remove diff on untoggle)
    - [ ] diff based on picked files
- [ ] quality of life:
    - [ ] only show commits of master branch

# raw ideas
only if commit selected we show diffs

# layout ideas
x |----------------|
x | commits        | 25%
x |--------|-------|
x |diff-   |file-  |
x |tree    |prev   | 50%
x |        |       |
x |--------|-------|
x | ? menu ?       | 25%
x |----------------|
