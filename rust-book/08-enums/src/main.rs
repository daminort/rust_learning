#[derive(Debug)]
enum IpType {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn send(&self) {
        println!("Sending message: {:?}", self);
    }
}

fn main() {
    let home = IpType::V4(String::from("127.0.0.1"));
    let loopback = IpType::V6(String::from("::1"));

    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);

    let m_write = Message::Write(String::from("Hello!"));
    let m_move = Message::Move { x: 10, y: 20 };
    let m_color = Message::ChangeColor(255, 255, 255);
    let m_quit = Message::Quit;

    m_write.send();
    m_move.send();
    m_color.send();
    m_quit.send();
}
