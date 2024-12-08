use log::debug;

fn main() {
    let rec1 = Rectangle{
        width:32,
        height:44,
    };
    dbg!(&rec1);
}

#[derive(Debug)]
struct Rectangle{
    width:i32,
    height:i32,
}

fn area(rectangle:&Rectangle)->i32{
    rectangle.height*rectangle.width
}
