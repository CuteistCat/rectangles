fn main() {
    const WIDTH1: u32 = 30;
    const HEEIGHT1: u32 = 50;
    println!(
	"the area of the rectangle is {} sqare pixels",
	area(width1, height1)
	);
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}
