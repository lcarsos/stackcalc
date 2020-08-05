use stackcalc::stack::Stack;

fn main() {
    let mut s: Stack<f32>;
    s.push(3.0);
    s.push(2.0);
    println!("end of stack: {}", s.peek().unwrap());
}
