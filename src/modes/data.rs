#[derive(PartialEq)]
enum Mode {
    Normal,
    Insert,
    Steno,
}

#[derive(PartialEq)]
enum Command {
    SwichMode(Mode),
    Exit,
}

struct BufferState {
    text: String,
    position: (u16, u16),
}
