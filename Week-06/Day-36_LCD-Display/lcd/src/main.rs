mod lcd;
use lcd::{LCD};

fn main() {
    LCD::new(12345).show(2);
    LCD::new(67890).show(3);
}