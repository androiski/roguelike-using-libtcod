use tcod::colors::*;
use tcod::console::*;



// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20; // 20 frames-per-second maximum

struct Tcod {
    root: Root,
    con: Offscreen,
}

//generic Object, can be reused for ANY object
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
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    //set color and draw character than represents this object at its position
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

fn handle_keys(tcod: &mut Tcod, player: &mut Object) -> bool {
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

    // init the window and game
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    let con = Offscreen::new(SCREEN_WIDTH,SCREEN_HEIGHT);
    let mut tcod = Tcod { root, con };

    //create player object
    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);

    //create NPC object
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2 - 5, '@', RED);

    //list of objects
    let mut objects = [player, npc];

    /*****************************************************************************************************/
    // main game loop
    while !tcod.root.window_closed() {

        //clear previous frame
        tcod.con.clear();

        for object in &objects {
            object.draw(&mut tcod.con);
        }

        //blit contents of con to root console and present it
        blit(
            &tcod.con,
            (0,0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut tcod.root,
            (0,0),
            1.0,
            1.0,
        );
        tcod.root.flush();

        let player = &mut objects[0];

        // key handler
        let exit = handle_keys(&mut tcod, player);
        if exit { break ; }
    }
    // end main game Loop
    /*****************************************************************************************************/



}

