fn char_at(x: usize, y: usize, width: usize, height: usize) -> char {
    if y == 0 || y == height - 1 || x == 0 || x == width - 1 || x == y || x == width - 1 - y {
        '*'
    } else {    // (: 
        ' '
    }
}
fn main() {
    const WIDTH: usize = 50;
const HEIGHT: usize = 50;

let mut output = String::new();

for y in 0..HEIGHT {
for x in 0..WIDTH {
 output.push(char_at(x, y, WIDTH, HEIGHT));
 }
output.push('\n');
 }
 print!("{}", output);
}
