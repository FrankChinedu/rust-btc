fn main() {}

enum Message {
    Quit,
    Move(Move),
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct Move {
    x: i32,
    y: i32,
}
