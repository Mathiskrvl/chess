enum Color {
    White,
    Black
}

enum Piece {
    King(Color),
    Queen(Color),
    Cavalier(Color),
    Fou(Color),
    Tour(Color),
    Pion(Color)
}

enum MoveType {
    Normal,
    DoublePush,
    EnPassant,
    Rook,
    Promotion,
}

struct Chess {
    board : [[Option<Piece>; 8]; 8],
    pit : Vec<Piece>
}

impl Chess {
    fn new() -> Self {
        Self {
            board : [
            [Piece::Tour(Color::Black), Piece::Cavalier(Color::Black), Piece::Fou(Color::Black), Piece::Queen(Color::Black), Piece::King(Color::Black), Piece::Fou(Color::Black), Piece::Cavalier(Color::Black), Piece::Tour(Color::Black)],
            [Piece::Pion(Color::Black), Piece::Pion(Color::Black), Piece::Pion(Color::Black), Piece::Pion(Color::Black), Piece::Pion(Color::Black), Piece::Pion(Color::Black), Piece::Pion(Color::Black), Piece::Pion(Color::Black)],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [Piece::Pion(Color::White), Piece::Pion(Color::White), Piece::Pion(Color::White), Piece::Pion(Color::White), Piece::Pion(Color::White), Piece::Pion(Color::White), Piece::Pion(Color::White), Piece::Pion(Color::White)],
            [Piece::Tour(Color::White), Piece::Cavalier(Color::White), Piece::Fou(Color::White), Piece::Queen(Color::White), Piece::King(Color::White), Piece::Fou(Color::White), Piece::Cavalier(Color::White), Piece::Tour(Color::White)]
            ],
            pit :vec![]
        }
    }
    fn action(&mut self, from: (u8, u8), to: (u8, u8)) {
        if let some(dead) = self.board[to.0][to.1] {
            self.pit.push(dead);
        }
        self.board[to.0][to.1] = self.board[from.0][from.1];
        self.board[from.0][from.1] = None;
    }


}

