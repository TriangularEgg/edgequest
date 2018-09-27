//! 
//! Hold the `Game` struct and the `play()` function
//! 

// tcod
//
// This library helps us create a window to render to, gives us functions for pathing, FOV, and 
// generating dungeons (though we don't use their dungeon generators because I am much more interested in building my own.
// See the `Dungeon` module.)
extern crate tcod;
// We use tcod here to create the root console (to give it to the renderer) and to grab inputs from the player
use self::tcod::{console, Console};
use core::tcod::input;

// The game log
//
// Every roguelike needs a way of delivering messages to the player because - spoiler - ASCII isn't exactly expressive enough
// to alert the player as to their surroundings or the goings on of the game world they inhabit. The log system primarily is held
// together by equal parts eldritch rust knowledge and voodoo magic and each time I use it I am both simultaneously shocked at
// how hacky it is and how good rust is at handling it.
//
// Log is imported first so all other modules can get the macro
#[macro_use]
pub mod log;
use self::log::GlobalLog;

// The world handler
//
// The world is in charge of creating dungeons, populating them with creatures and items, and keeping track of the player.
// It basically ties together all game subsystems into a concrete package, hence, world.
//
// World is public so that docs are generated for it
pub mod world;
// Import world directly so we can make instances of it
use core::world::World;

// Game objects
//
// Objects are essentially what most game-related non-map entities can be boiled down to. Objects encompass
// the very building blocks of the game such as `Pos`itions, `Creature`s which are combinations of `Actor`s, `Entity`s,
// and `Action`s, and AI patterns that help drive monster descicion making.
//
// Object is public so that docs are generated for it
pub mod object;
// While normally this module most likely should not have access to objects, we need to see `Action`s as the player's
// choices changes the state of the game
use core::object::actions::Actions;

// Renderer
//
// The renderer is the interface by which game objects and constructs are made real through tcod interaction. The central
// idea is that if tcod is ever abandoned, minimal ammounts of code outside of the renderer should have to be changed.
// This hasn't *really* been the case as `World` depends on tcod lighting maps, the `Engine` holds a tcod root, and init creates
// said root.
//
// Renderer is public so that docs are generated for it
pub mod renderer;
// We import the renderer to create instances of it and RGB so we can color some log outputs
use self::renderer::Renderer;
use self::renderer::RGB;

// Initializer
// 
// All things must be created, and all configurations must be loaded at some point. The initializer module prepares tcod consoles,
// and loads the configs with serde
//
// Pretty sure you understand why this is public by now
pub mod init;

///
/// Enum representing the state of the game
/// 
/// "All programs are just one big finite automata" - Nowak, probably
/// 
/// All games essentially boil down to a number of finite states and the transitions between them. To help decide what state
/// the game transitions to, or to make other choices regarding the game, some notion of a state has to be preserved to base
/// such a judgement off of. Most of the time however, the player is the main driving factor of the state changes.
///
pub enum State {
  // Game just created
  New,
  // Player acted
  Act(Actions),
  // Key was pressed
  Keypress,
  // Debug command was triggered
  Debug
}

///
/// Engine struct to package the world with the renderer as well as any debug flags we may want to add
/// 
/// This engine isn't really a typical "game engine" in the way unreal or unity are but more or less what actually causes the game 
/// to even be playable in the first place, as world elements are wrapped together with a renderer and input. It essentially bridges the gap
/// between physical user and the game world via the renderer, and in that sense it's more of a steering wheel than an engine, but 
/// `SteeringWheel` makes pretty much no sense whatsoever and "engine" is already within the common vernacular of game developers
/// 
pub struct Engine {
  world: World,
  state: State,
  ren: Renderer,
  root: console::Root,

  // Debug
  noclip: bool

}

impl Engine {

  ///
  /// Capture keyboard input from tcod and update player state
  /// 
  fn process_keypress(&mut self, keypress: input::Key) {

    match keypress.code {
      
      // If the keycode isn't escape we continue checking for important keys
      input::KeyCode::Escape => panic!("Bye"),

      // This part of the code is for capturing the keypress not as an object, but as a character for easier parsing
      _ => { 
        
        // We only care if the key is printable, aka, has some symbol attached to it
        if keypress.printable != ' ' {
          
          // First, make an assumption that the player is affecting their movement as 90% of the game
          // is walking around. We *could* add it to every single one of the vim keypresses to save a trivial ammount of
          // time assigning this variable, but I dislike that.
          let oldpos = self.world.player.actor.pos.clone();

          // In addition, update the game state
          self.state = State::Keypress;

          // Begin to pattern match the char corresponding to the key pressed
          match keypress.printable {

            
            // Movement keys are bound to vim-like controls (hjklyubn)
            'h' => { 
              self.world.player.actor.move_cart(-1, 0);
              self.world.player.state = Actions::Move;
            },
            'j' => { 
              self.world.player.actor.move_cart(0, 1);
              self.world.player.state = Actions::Move;
            },
            'k' => {
              self.world.player.actor.move_cart(0, -1);
              self.world.player.state = Actions::Move;
            },
            'l' => {
              self.world.player.actor.move_cart(1, 0);
              self.world.player.state = Actions::Move;
            },
            'y' => {
              self.world.player.actor.move_cart(-1, -1);
              self.world.player.state = Actions::Move;
            },
            'u' => {
              self.world.player.actor.move_cart(1, -1);
              self.world.player.state = Actions::Move;
            },
            'b' => {
              self.world.player.actor.move_cart(-1, 1);
              self.world.player.state = Actions::Move;
            },
            'n' => { 
              self.world.player.actor.move_cart(1, 1);
              self.world.player.state = Actions::Move;
            },

            // "Regenerate" current level
            'w' => {
              self.world.test_traverse();
              self.state = State::Act(Actions::Unknown);
            },
            // Create an empty level for testing
            'q' => {
              self.world.test_empty();
              self.state = State::Act(Actions::Unknown);
            },
            // Wait
            '.' => { 
              self.world.player.state = Actions::Wait;
            },

            // Go downstars (if possible)
            '>' => { self.world.player.state = Actions::DownStair },
            // Go upstairs (if possible)
            '<' => { self.world.player.state = Actions::UpStair },

            // Debug keypresses

            // Toggle scent
            'r' => {
              self.ren.show_scent = !self.ren.show_scent;
              self.ren.draw_all(&mut self.root, &mut self.world);
              self.state = State::Debug;
            },

            // Toggle sound
            't' => {
              self.ren.show_sound = !self.ren.show_sound;
              self.ren.draw_all(&mut self.root, &mut self.world);
              self.state = State::Debug;
            },

            // Toggle FoV
            'f' => {
              self.ren.fov = !self.ren.fov;
              self.ren.draw_all(&mut self.root, &mut self.world);
              self.state = State::Debug;
            },

            // Toggle noclip
            'z' => {
              self.noclip = !self.noclip;
              self.state = State::Debug;
            },

            // Unbound key, so we just say we don't know what the player did
            _ => { self.world.player.state = Actions::Unknown }
            
          }

          // Now the game state needs to be properly re-oriented based on the (potential) player action.
          // In addition, we should also process the action of the player while we're here
          match self.state {

            // If state is Debug, don't override
            State::Debug => (),

            _ => {

              // Set game state to player state
              self.state = State::Act(self.world.player.state.clone());

              // Now let's process the player's action
              match self.world.player.state {

                Actions::Move => {

                  // Make sure player doesn't do anything dumb
                  if !self.world.is_valid_pos(self.world.player.actor.pos.x, self.world.player.actor.pos.y) && !self.noclip {
                    self.world.player.actor.pos = oldpos;
                    self.world.player.state = Actions::Unknown;
                  } else {
                    self.world.check_trap();
                  }

                }

                _ => ()

              }

            }
          }

        } 
        
        // Prints keycode to console in case if you're trying to find a key that isn't intutive, or you're debugging
        /* 
        else {
          println!("{:?}", keypress.code);
        }
        */

      }

    }
    
  }

  ///
  /// Return a new `Engine`
  /// 
  pub fn new() -> Engine {

    // Get map height
    let map_dim = init::map_dimensions();

    // Get root console
    let root = init::root();

    Engine {
      world: World::new(map_dim),
      state: State::New,
      ren: Renderer::new(
        map_dim, 
        (root.width() as isize, root.height() as isize), 
        init::console_height(),
        init::panel_width()
      ),
      root: root,

      // Debug 
      noclip: false
    }
    
  }

  ///
  /// Update the game state, then update the world depending on the new state
  ///
  fn update(&mut self) {

    match self.state {
      // Moving or waiting prompts a world update
      State::Act(Actions::Move) | State::Act(Actions::Wait) => self.world.update(),

      // Trying to go up and downstairs prompts the respective response from world
      State::Act(Actions::DownStair) => {
        self.world.go_down();
      },

      State::Act(Actions::UpStair) => {
        self.world.go_up();
      }
      
      // All other variants are dropped
      _ => ()

    }
    
  }

  ///
  /// Title screen test
  ///
  fn title_screen(&mut self) {

    // First part of this pretty much just fills the screen with black

    let w = self.root.width().clone();
    let h = self.root.height().clone();

    for x in 0..w {
      for y in 0..h {
        self.root.put_char_ex(
          x as i32,
          y as i32,
          ' ',
          RGB(0, 0, 0).to_tcod(),
          RGB(0, 0, 0).to_tcod()
        );
      }
    }

    // Second part displays stuff

    let title = "Edgequest";
    let subtitle = "Press any key to start.";

    self.root.set_default_foreground(RGB(255, 255, 255).to_tcod());
    // i32 conversion is a pain since I'd rather store stuff as isize and the tcod lib wants i32 since it's
    // pretty much just a C++ interface which is annoying
    self.root.print((w / 2 - (title.len() / 2) as i32) as i32, (h / 2 - 1) as i32, title);
    self.root.print((w / 2 - (subtitle.len() / 2) as i32) as i32, (h / 2 + 1) as i32, subtitle);

    self.root.flush();

    // Wait for keypress
    let _keypress = self.root.wait_for_keypress(true);

  }

  ///
  /// Play the game.
  /// 
  pub fn play(&mut self) {
    
    // Create the title screen
    self.title_screen();

    // Some starting messages, will be removed in later versions (hopefully)
    log!(("Welcome to Edgequest", RGB(255, 0, 255)));
    log!(("Move with vim keys", RGB(255, 255, 255)));
    log!(("esc to quit, w to regenerate", RGB(255, 255, 255)));
    log!(("r to toggle scent, t to toggle sound", RGB(255, 255, 255)));
    log!(("f to toggle FoV, z to toggle noclip", RGB(255, 255, 255)));

    // Initial update
    self.update();

    // Draw all and capture keypresses
    while !self.root.window_closed() {

      // Draw what the camera sees
      self.ren.draw_all(&mut self.root, &mut self.world);
      
      // Capture game keys (Keys that change the state of the player)
      // This is what gives it the turn based nature, i.e. waits for player input before
      // doing anything
      let keypress = self.root.wait_for_keypress(true);
      self.process_keypress(keypress);

      // Update game
      self.update();

    } 

  }

}