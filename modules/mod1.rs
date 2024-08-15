mod player {
    // private function
    fn focus() {
        println!("called player::focus");
    }

    // public function
    fn shift() {
        println!("called player::shift");
    }

    // public function
    pub fn jump() {
        // call private function focus and shift inside the module
        focus();
        shift();
        println!("called player::jump");
    }
}

// nested module
pub mod play {
    pub mod sprite {
        pub fn create() {
            println!("called player::sprite::create");
        }
    }
}

use play::sprite::create;

fn main() {
    // call public function jump from player module
    player::jump();

    play::sprite::create();

    create();
}