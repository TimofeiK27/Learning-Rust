enum IpAddrType {
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),

}

impl Message {
    fn prr(self) {
        println!("AAA");
    }
}
struct IpAddr {
    kind : IpAddrType,
    data : i32,
}

fn main() {
    let localhost = IpAddrType::V4(1,0,0,127);
    let message = Message::Write(String::from("HELLO"));
    let message2 = Message::Quit;

    let addr = IpAddr {kind : localhost, data : 100};
    message.prr();
    message2.prr();

    let x : i8 = 5;
    let y : Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0);
    println!("{}", sum);

    let z: Option<i8> = None;
    let sum = x + z.unwrap_or(2);
    println!("{}", sum);

    println!("y + 1: {:?}", plus_one(y.map(|x| x as i32))); // maps each value in y to be i32
    println!("y + 1: {:?}", plus_one(z.map(|x| x as i32)));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}