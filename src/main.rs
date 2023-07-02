fn main() {
    println!("Width:")
    let mut width = String::new();    //
    io::stdin().read_line(&mut width)
    	.expect("failed to read line");
    let width:u32 = width.trim().parse()
        .expect("Not a valid number");
    println!("height")
    let mut height = String::new();    //
        io::stdin().read_line(&mut height)
    	.expect("failed to read line");
    let width:u32 = width.trim().parse()
        .expect("Not a valid number");
    
    println!(
        "the area of the rectangle is {} sqare pixels",
        area(WIDTH1, HEIGHT1)
    );
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}
