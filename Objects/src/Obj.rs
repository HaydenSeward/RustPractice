struct Obj {

    x: u32,
    y: String,

} 

impl Obj {

    fn hello(&self, a: &str) {

        println!("{one}, {two}, {three}",
                one = self.x,
                two = self.y,
                three = &a);

    }
    fn new(&str

}

fn main() {

    let o = Obj{x: 8, y: "Hello".to_string()};
    o.hello("c");

}
