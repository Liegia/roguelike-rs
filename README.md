# Learning rust with a quick roguelike tutorial

### ** link to the tutorial <https://tomassedovic.github.io/roguelike-tutorial/part-1-graphics.html>**

## Notes

To use any external librarys in Rust, you need to add it to the Cargo.toml under [dependencies].
**Rust libraries** are called **crates**. To add some imports use the **_use_** statements.

" use tcod::colors::_ "
" use tcod::console::_ "

Like Python, constants are written using uppercase.
in functions, defenitions and constants we need to explicitly mention what type it is.
i32 is a 32-bit signed integer.

'let' is used to create a new variable.

    let mut tcod = Tcod { root };

creates a mutable variaable called tcod.  
Variable are immutable by default so we need to use **_mut_** keyword to be able to write to it later.  
We don't have to specify the type of variable, Rust will figure it out.

Like Python x,y 0,0 is the top left of the screen.
