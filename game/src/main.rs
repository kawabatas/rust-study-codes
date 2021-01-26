// @see
// https://github.com/a5huynh/defender-game

use piston::event_loop::{EventSettings, Events};
use piston::input::{PressEvent, ReleaseEvent, RenderEvent, UpdateEvent};

use game::config::GraphicsConfig;
use game::App;

fn main() {
    // Original Defenders had a resolution of 320x256
    let mut app = App::new(GraphicsConfig::new("Defender", 640, 512));

    // Poll for events from the window.
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut app.window.settings) {
        // Handle keyboard input
        if let Some(i) = e.press_args() {
            app.input(&i, true);
        }
        if let Some(i) = e.release_args() {
            app.input(&i, false);
        }
        // Handle rendering
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        // Handle any updates
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
