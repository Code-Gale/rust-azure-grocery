// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
// Add a new enum called BoxState with three variants:
// Open
// Closed
// Sealed
// Extend your existing ShippingBox struct by adding a new field:
// state: BoxState
// In your impl ShippingBox block:
// Update your new() method to set the initial state to Closed
// Add a method open(&mut self) → sets state to Open
// Add a method seal(&mut self) → sets state to Sealed
// Add a method display_state(&self) → prints the current state nicely ("The box is sealed", etc.)
// In your print_char() method, include the box’s state at the end.
// In your main():
// Create one box
// Display it
// Open it
// Display again
// Seal it
// Display one last time
enum Color {
    Black,
    White,
    Blue
}
enum BoxState{
    Opened,
    Closed,
    Sealed
}
struct ShippingBox {
    dimensions: (f64, f64, f64),
    weight: f64,
    color: Color,
    state: BoxState
}

impl ShippingBox{
    fn new(dimensions: (f64, f64, f64), weight: f64, color:Color) -> Self {
        Self{
            dimensions,
            weight,
            color,
            state: BoxState::Closed,
        }
    }

    fn print_char(&self){
        println!("Dimensions: {}cm, {}cm, {}cm | Weight: {}g | Color: {} | State: {}",
        self.dimensions.0,
        self.dimensions.1,
        self.dimensions.2,
        self.weight,
        self.get_color(),
        self.get_state()
    );
    }

    fn open_box(&mut self) {
        let message = if matches!(self.state, BoxState::Opened) {
            "This box has already been opened."
        } else if matches!(self.state, BoxState::Sealed) {
            "This box has been sealed and can't be opened."
        } else {
            self.state = BoxState::Opened;
            "Box opened successfully."
        };
        println!("{}", message);
    }

    fn seal_box(&mut self) {
        let message = if matches!(self.state, BoxState::Sealed){
            "This box has already been Sealed."
        } else {
            self.state = BoxState::Sealed;
            "Box sealed successfully."
        };
        println!("{}", message);
    }

    fn get_color(&self) -> &str {
        match self.color{
            Color::Black => "Black",
            Color::White => "White",
            Color::Blue => "Blue"
        }
    }

    fn get_state(&self) -> &str {
        match self.state{
            BoxState::Opened => "Opened",
            BoxState::Closed => "Closed",
            BoxState::Sealed => "Sealed"
        }
    }
}

fn main() {
    let mut box1 = ShippingBox::new((10.0, 7.5, 15.0), 25.0, Color::Black);
    box1.print_char();
    box1.open_box();
    box1.print_char();
    box1.seal_box();
    box1.print_char();
}
