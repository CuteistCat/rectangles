/*
   Copyright [yyyy] [name of copyright owner]

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/
use std::io;

fn main() {
    println!("Width:");
    let mut width = String::new();    //
    io::stdin().read_line(&mut width)
    	.expect("failed to read line");
    let width:u32 = width.trim().parse()
        .expect("Not a valid number");
    println!("height:");
    let mut height = String::new();    //
        io::stdin().read_line(&mut height)
    	.expect("failed to read line");
    let height:u32 = height.trim().parse()
        .expect("Not a valid number");
    
    println!(
        "the area of the rectangle is {} sqare pixels",
        area(width, height)
    );
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}
