use tcod::colors::*;
use tcod::console::*;



// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum

// size of map
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

// COLORS for BUILDING
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };

struct Tcod {
    root: Root,
    con: Offscreen,
}

fn handle_keys(tcod: &mut Tcod, game: &Game, player: &mut Object) -> bool {
    //if you import in a function, only this function can use it
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = tcod.root.wait_for_keypress(true);

    match key {
        //make fullscreen
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }

        //exit game
        Key { code: Escape, .. } => return true,

        //keys for movement
        Key { code: Up, .. } => player.move_by(0, -1, game),
        Key { code: Down, .. } => player.move_by(0, 1, game),
        Key { code: Left, .. } => player.move_by(-1, 0, game),
        Key { code: Right, .. } => player.move_by(1, 0, game),

        _ => {}
    }
    false
}


// generic Object, can be reused for ANY object
#[derive(Debug)]
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

    //move by given
    pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game) {
        if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    //set color and draw character than represents this object at its position
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

/// A tile of the map and it properties
/// The #[derive(…​)] bit automatically implements certain behaviours (Rust calls them traits, other languages use interfaces) you list there
/// Debug is to let us print the Tile’s contents and
/// Clone and Copy will let us copy the values on assignment or function call instead of moving them. So they’ll behave like e.g. integers in this respect.
#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }
    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}
/// the map is 2D arr of tiles
/// created shortcut so that we just write Map and not the full statement
type Map = Vec<Vec<Tile>>;

struct Game {
    map: Map,
}

/// define fn to make the map
/// use of vec! macro :)
fn make_map() -> Map {
    // fill map with "unblocked" tiles
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    // place two pillars to test the map
    map[30][22] = Tile::wall();
    map[50][22] = Tile::wall();

    map
}

// draw/render map to window
fn render_all(tcod: &mut Tcod, game: &Game, objects: &[Object]) {
    // go thru all tiles, set the BG and draw
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;
            if wall {
                tcod.con.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                tcod.con.set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }

    // draw all objects in the list
    for object in objects {
        object.draw(&mut tcod.con);
    }


     // blit contents of con to root console and present it
     blit(
        &tcod.con,
        (0,0),
        (MAP_WIDTH, MAP_HEIGHT),
        &mut tcod.root,
        (0,0),
        1.0,
        1.0,
    );
}


fn main() {
    tcod::system::set_fps(LIMIT_FPS);

    // init the window and game
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    let con = Offscreen::new(MAP_WIDTH,MAP_HEIGHT);
    let mut tcod = Tcod { root, con };

    //create player object
    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);

    //create NPC object
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2 - 5, '@', RED);

    //list of objects
    let mut objects = [player, npc];

    // create game, render
    let game = Game {
        // generate (not draw) map
        map: make_map(),
    };

    /*****************************************************************************************************/
    // main game loop
    while !tcod.root.window_closed() {

        //clear previous frame
        tcod.con.clear();

        for object in &objects {
            object.draw(&mut tcod.con);
        }

        //render screen
        render_all(&mut tcod, &game, &objects);
        tcod.root.flush();

        let player = &mut objects[0];

        // key handler
        let exit = handle_keys(&mut tcod, &game, player);
        if exit { break ; }
    }
    // end main game Loop
    /*****************************************************************************************************/



}

