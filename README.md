# xinOS Rust Boilerplate
A boilerplate to help you start making xinOS games in Rust

## Dependencies
To use this boilerplate, all you need is **Xargo**

## Building
To build the application for use with xinOS, run `make`  
A library file (.a) will be generated in `target/i386-unknown-none`, place that in xinOS's `src/games` directory, and edit game_list.h to add your game

```c
int yourGame_main();

struct game games[] = {
    {.name = "yourGame", .author = "you", .init = yourGame_main}
}
```

## Notes
- There is no std, use core and no_std crates
- Some functions have safe wrappers, some are unsafe FFI functions, most are not defined at all
- Strongly avoid using mutable statics, static Mutexes and anything related to statics and mutation, unless you reinitialize them everytime the main function is entered