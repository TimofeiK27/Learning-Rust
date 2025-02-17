fn main() {
    let x = 5;
    println!("val is {}", x);
    const ARR : i32 = 100;
    println!("const {}", ARR);
    // shadowing, first x is shadowed by second

    let x = "HI";
    println!("shadowed val {} is ",x);

    let over : u8 = 255;
    println!("overflowed {}", over);

    let tup = ("AWH", 100);
    println!("{}", tup.0);

    let arr = [22; 8];
    println!("{}", arr[7]);

    let x = 10;
    let y = 12;
    let sum = my_func(x,y);
    println!("sum is: {}", sum);

    let mut count = 0;
    let result = loop {
        count += 1;
        if count > 10 {
            break count;
        }
    };
    println!("loop interated {} times", result);


    let mut cur = 3;
    while cur >= 0 {
        println!("cur is {}", cur);
        cur -= 1;
    };

    // showing how to do range()
    for num in 1..4 {
        println!("{}", num);
    }
}

fn my_func(x:i32, y:i32) -> i32{
    println!("x is {} and y is {}", x, y);
    x + y
}
