use std::cmp::Ordering;

#[derive(Debug)]  // allows compiler to use debug trait, allows printing struct
struct Rectangle {
    width: i32,
    height:i32
}
impl Rectangle {
    fn area(&self) -> i64 {
        (self.width * self.height).into()
    }
    fn compare_area(&self, rect : &Rectangle) -> i32 {
        match self.area().cmp(&rect.area()) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1
        }
    }
    // associated functions (static method)
    fn square(len : i32) -> Rectangle {
        let rect = Rectangle {width:len, height:len};
        return rect;
    }
}

fn main() {
    let rect = Rectangle {
        width : 10,
        height: 20
    };
    let rect2 = Rectangle {
        width : 9,
        height: 19
    };
    println!("rect : {:#?}", rect);

    // uses function 
    let a = area(&rect);

    // uses struct method
    let b = rect.area();

    println!("area is : {}", a);
    println!("area is via method : {}", b);

    // in rust, . works as . and -> in c

    // try compare_area, prints 1 because rect bigger than rect2
    println!("{}", rect.compare_area(&rect2));

    let rect3 = Rectangle::square(3);
    println!("square : {:#?}", rect3);
}

fn area(rect : &Rectangle) -> i64 {
    (rect.width * rect.height).into()
}