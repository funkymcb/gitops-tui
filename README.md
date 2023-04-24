# gitops-tui
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
    - [ ] file picker based on stages and picked commits
    - [ ] diff based on picked files
- [ ] quality of life:
    - [ ] filter commits by branch/author/date
