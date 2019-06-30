# lll

Simple terminal file manager

WIP, extremely rough. So far, you can open a file from a list of files.

## Goals

Almost no features. Think more like `noice` than `ranger`.

Keybinds:

* up/down arrows: up/down selection
* enter: open file
* `q`: exit

* None of these are planned features:
  * Themes
  * Built-in shell execution
  * Config files
  * Multiple panes (use screen or tmux)
  * System information
  * Windows support

* But these are:
  * Handle files in their default apps (xdg-open)
  * Planned keybinds (not implemented yet):
    * `hjkl` navigation
    * `d` cut a file/directory
    * `y` yank (copy) a file/directory
    * `p` put
    * `x` delete
    * `/` search (smartcase) (should filter results, not navigate between matches)
    * `space` select multiple items
    * `s` drop to a shell (on `exit`, should return to `lll`)

## Other options

* `ranger` (Python) is the gold standard. It's got every feature under the sun,
  and is not always fast.
* `hunter` and `joshuto` are `ranger` clones in Rust. `joshuto` is pretty good!
  I haven't tried `hunter` because it requires more packages than I want to
  install.
* `nnn`, `noice`, and `rover` are relatively small and written in C. I like
  them! But I don't like C.
* `fff` is also fun, but it's written in Bash, and I wanted something both
  fast and maintainable.
* `lf` (Go) is very good. Like `ranger` minus a bunch of stuff I never use. I
  should really get better at Go.

[LICENSE (MIT)](./LICENSE.md)
