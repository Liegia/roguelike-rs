# Learning rust with a quick roguelike tutorial

### ** link to the tutorial <https://tomassedovic.github.io/roguelike-tutorial/part-1-graphics.html>**

## Notes

To use any external librarys in Rust, you need to add it to the Cargo.toml under [dependencies].
**Rust libraries** are called **crates**. To add some imports use the **_use_** statements.

" use tcod::colors::* "
" use tcod::console::* "

Like Python, constants are written using uppercase.
in functions, defenitions and constants we need to explicitly mention what type it is.
i32 is a 32-bit signed integer.

'let' is used to create a new variable.

    let mut tcod = Tcod { root };

creates a mutable variaable called tcod.  
Variable are immutable by default so we need to use **_mut_** keyword to be able to write to it later.  
We don't have to specify the type of variable, Rust will figure it out.

Like Python x, y / 0, 0 is the top left of the screen.

A function signature in Ruat is

    fn function_name(parameter: type, ...) -> return_type

We can use `&mut` before the type to borrow operators.  
<https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html>  
<https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html>

In **libtcod**, console is where we _draw stuff_.  
We can create _offscreen consoles_ and draw on them.  
This will let us add transparency effekts or render only portions of the console.  
It will let us stack GUI elements on top of the game window. Render the info panel, game map and so on separatly.
  
The `dyn` keyword in `&mut dyn Console` highlights that `Console` is a **_trait_** and not a concrete type, such as `struct` or `enum`.  
