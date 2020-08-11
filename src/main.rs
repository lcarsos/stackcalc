use stackcalc::stack::Stack;

fn main() {
    let mut s: Stack<f32> = Stack::new();
    s.push(3.0);
    s.push(2.0);
    println!("end of stack: {}", s.peek().unwrap());
    s.pop();
    println!("new end of stack: {}", s.pop().unwrap());
}
