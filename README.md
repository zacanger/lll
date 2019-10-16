# lll

Simple terminal file manager

WIP, extremely rough. I would not recommend using this until it's 1.0.0.

## Goals

Almost no features. Think more like `noice` than `ranger`.

* Current keybinds:
  * `hjkl`: navigation
  * `q` quit
  * `G` jump to bottom
  * `g` jump to top

* None of these are planned features:
  * Themes
  * Built-in shell execution
  * Config files
  * Multiple panes (use screen or tmux)
  * System information
  * Windows support

* But these are:
  * Handle files in their default apps (xdg-open)
  * Make scrolling work (currently doesn't re-draw when you go below or above the current screen)
  * Planned keybinds (mostly not implemented yet):
    * `l` should open a file, if not a directory (XDG_OPEN, spawn new terminal?)
    * `d` cut a file/directory
    * `y` yank (copy) a file/directory
    * `p` put
    * `x` delete
    * `/` search (smartcase) (should filter results, not navigate between matches)
    * `.` toggle hidden files visibility (true by default)
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
* `marcos` (Rust) is rough, but I'm using some code from it (MIT, Hitesh Paul).

[LICENSE](./LICENSE.md)
