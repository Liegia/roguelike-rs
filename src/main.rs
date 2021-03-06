///2021-02-21 K.Heidenborg
/// Roguelike tutorial by Tomasse Dovic. tomassedovic.github.io
use tcod::colors::*;
use tcod::console::*;

// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20; // max 20 fps

struct Tcod {
    root: Root,
    con: Offscreen,
}

/// Generic object: The Player, a monster, an item, the stairs...
/// It's always represented by a character on screen
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object { x, y, char, color}
    }
    
    /// Move by the given amount
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.y += dx;
        self.x += dy;
    }
    
    /// Set the color and then draw the character that represents this object at its position
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_background(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}


fn handle_keys(tcod: &mut Tcod, player_x: &mut i32, player_y: &mut i32) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            // Alt+Enter: Toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }

        Key { code: Escape, .. } => return true, // exit game

        // movement keys
        Key { code: Up, .. } => player.move_by(0, -1),
        Key { code: Down, .. } => player.move_by(0, 1),
        Key { code: Left, .. } => player.move_by(-1, 0),
        Key { code: Right, .. } => player.move_by(1, 0),

        _ => {}
    }

    false
}

fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tcod = Tcod { root, con };

    // create object representing the player
    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);
    
    // create npc
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', YELLOW);
    
    // the list of objects with those two
    let mut objects = [player, npc];

    // let mut player_x = SCREEN_WIDTH / 2; // Initialise to the center
    // let mut player_y = SCREEN_HEIGHT / 2; // of the screen

    while !tcod.root.window_closed() {
        // clear the screen
        tcod.con.clear()
        for object in &objects {
            object.draw(&mut tocd.con);
        }

        //tcod.con.set_default_foreground(WHITE);
        //tcod.con.clear();
        //tcod.con
        //    .put_char(player_x, player_y, '@', BackgroundFlag::None);
        //tcod.root.flush();
        //tcod.root.wait_for_keypress(true);

        // Blit the contetnts of "con" to the root console and present it
        blit(
            &tcod.con,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut tcod.root,
            (0, 0),
            1.0,
            1.0,
        );
        
        tcod.root.flush();

        // Handle keys and exit game if needed
        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, player);
        if exit {
            break;
        }
    }
}
