// 🧩 Challenge: Grocery Inventory (with impl)
// 🪄 Requirements
// Create a struct called GroceryItem with:
// name: String
// quantity: i32
// id: i32
// Use an impl block to define:
// A constructor (new()) that returns a new GroceryItem
// A method display() that prints all details in this format:
// Item: Apple | Quantity: 10 | ID: 1
// A method restock(&mut self, amount: i32) that adds the given amount to the quantity
// A method sell(&mut self, amount: i32) that subtracts the amount (but not below 0)
// In main():
// Create a grocery item using GroceryItem::new()
// Display it
// Restock by 15
// Display again
// Sell 8
// Display one last time
#[allow(dead_code)]
struct GroceryItem{
    name: String,
    quantity: i32,
    id: i32
}
impl GroceryItem{
    fn new(name: String, quantity: i32, id: i32) -> Self{
        Self{
            name,
            quantity,
            id
        }
    }

    fn display(&self){
        println!("Displaying Item...");
        println!("Item: {} | Quantity: {} | ID: {}", self.name, self.quantity, self.id);
    }

    fn restock(&mut self, num: i32){
        println!("Restocking Item...");
        self.quantity += num;
    }

    fn sell(&mut self, num: i32){
        let message = if self.quantity == 0 {
            "Item is currently out of stock. Please restock...".to_string()
        } else if num > self.quantity{
            "You don't have enough stock for this sales. Please restock...".to_string()
        } else {
            self.quantity -= num;
            format!("You just sold {} {}", num, self.name)
        };
        println!("{}", message)
    }
}
fn main(){
    let mut apple = GroceryItem::new("Apple".to_string(), 15, 99);
    apple.display();
    apple.restock(10);
    apple.display();
    apple.sell(8);
    apple.display();
}