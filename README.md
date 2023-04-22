# gitops-tui
Tui to help synchronise different gitops cluster repos

# TODOs
- [ ] read repos from config and clone
- [ ] list pretty printed commits (like: `git log --pretty=format:"%<(16)%ah: %Cgreen%h%Creset %an: %B"`)
- [ ] build tui layout:
    - [ ] picker for commits
    - [ ] file picker based on stages and picked commits
    - [ ] diff based on picked files
