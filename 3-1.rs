
struct Nil;

struct Pair(i32, f64);

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}
//允许代码未被使用
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main(){
    let point: Point = Point { x: 0.3, y:0.4};

    println!("point coordinate： ({}, {})", point.x, point.y);

    let Point { x: my_x, y: my_y} = point;

    //my_x和my_y变量作为参数赋值给Point不会move overship
    let _rectangle = Rectangle {
        p1: Point { x: my_y, y:my_x},
        p2: point,
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    let Pair(integer, decimal) = pair;

    println!("my_x: {} , my_y: {}",my_x, my_y);
    println!("pair contains {:?} and {:?}", integer, decimal);
    println!("Rectangle is {:?}", _rectangle);
}
