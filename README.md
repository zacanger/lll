# lll

Simple terminal file manager

WIP, extremely rough

## Goals

Almost no features. Think more like `noice` than `ranger`, which is why I'm
writing my own rather than using Hunter.

* No preview
* Minimal keybinds (`hjkl`, `dd`/`yy`/`pp`, `xx` to delete)
* Handle files in their default apps (xdg-open)
* Drop to a shell for most operations

## Other options

* `ranger` (Python) is the gold standard. It's got every feature under the sun,
  and is not always fast.
* `hunter` and `joshuto` are `ranger` clones in Rust. `joshuto` is pretty good! I
  haven't tried `hunter` because it requires more packages than I want to
  install.
* `nnn`, `noice`, and `rover` are relatively small and written in C. I like
  them! But I don't like C.
* `fff` is also fun, but it's written in Bash, and I wanted something both
  fast and maintainable.
* `lf` (Go) is very good. Like `ranger` minus a bunch of stuff I never use. I
  should really get better at Go.

[LICENSE (MIT)](./LICENSE.md)
