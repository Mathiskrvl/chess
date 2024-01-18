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

impl Piece {
    fn get_color(&self) -> Color {
        match self {
            Piece::King(color) => color,
            Piece::Queen(color) => color,
            Piece::Cavalier(color) => color,
            Piece::Fou(color) => color,
            Piece::Tour(color) => color,
            Piece::Pion(color) => color,
        }
    }
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
            board: [
            [Piece::Tour(Color::White), Piece::Cavalier(Color::White), Piece::Fou(Color::White), Piece::Queen(Color::White), Piece::King(Color::White), Piece::Fou(Color::White), Piece::Cavalier(Color::White), Piece::Tour(Color::White)],
            [Piece::Pion(Color::White), Piece::Pion(Color::White), Piece::Pion(Color::White), Piece::Pion(Color::White), Piece::Pion(Color::White), Piece::Pion(Color::White), Piece::Pion(Color::White), Piece::Pion(Color::White)],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [Piece::Pion(Color::Black), Piece::Pion(Color::Black), Piece::Pion(Color::Black), Piece::Pion(Color::Black), Piece::Pion(Color::Black), Piece::Pion(Color::Black), Piece::Pion(Color::Black), Piece::Pion(Color::Black)],
            [Piece::Tour(Color::Black), Piece::Cavalier(Color::Black), Piece::Fou(Color::Black), Piece::Queen(Color::Black), Piece::King(Color::Black), Piece::Fou(Color::Black), Piece::Cavalier(Color::Black), Piece::Tour(Color::Black)]
            ],
            pit: Vec::new()
        }
    }
    fn action(&mut self, from: (u8, u8), to: (u8, u8)) {
        if let Some(dead) = self.board[to.0][to.1] {
            self.pit.push(dead);
        }
        self.board[to.0][to.1] = self.board[from.0][from.1];
        self.board[from.0][from.1] = None;
    }

    fn available_move(&self, from: (u8, u8)) -> Vec((MoveType, u8, u8)) {
        match self.board[from.0][from.1] {
            Piece::Tour(color) => self.tour_available_move(color, from),
            Piece::Fou(color) => self.fou_available_move(color, from),
            Piece::Cavalier(color) => self.cavalier_available_move(color, from),
            Piece::King(color) => self.king_available_move(color, from),
            Piece::Queen(color) => self.queen_available_move(color, from),
            Piece::Pion(color) => self.pion_available_move(color, from),
        }
    }

    fn tour_available_move(&self, color: Color, from: (u8, u8)) -> Vec((MoveType, u8, u8)) {
        let mut available_move = Vec::new();
        // haut
        for x in 0..from.0 {
            if let Some(piece) = self.board[x][from.1] {
                if piece.get_color() != color {
                    available_move.push((MoveType::Normal, x, from.1));
                    break
                } else {
                    break
                }
            } else {
                available_move.push((MoveType::Normal, x, from.1));
            }
        }
        // bas
        for x in from.0..8 {
            if let Some(piece) = self.board[x][from.1] {
                if piece.get_color() != color {
                    available_move.push((MoveType::Normal, x, from.1));
                    break
                } else {
                    break
                }
            } else {
                available_move.push((MoveType::Normal, x, from.1));
            }
        }
        // gauche
        for y in 0..from.1 {
            if let Some(piece) = self.board[from.0][y] {
                if piece.get_color() != color {
                    available_move.push((MoveType::Normal, from.0, y));
                    break
                } else {
                    break
                }
            } else {
                available_move.push((MoveType::Normal, from.0, y));
            }
        }
        // droite
        for y in from.1..8 {
            if let Some(piece) = self.board[from.0][y] {
                if piece.get_color() != color {
                    available_move.push((MoveType::Normal, from.0, y));
                    break
                } else {
                    break
                }
            } else {
                available_move.push((MoveType::Normal, from.0, y));
            }
        }
        // return
        available_move
    }
    fn fou_available_move(&self, color: Color, from: (u8, u8)) -> Vec((MoveType, u8, u8)) {
        let mut available_move = Vec::new();
        let (mut bd, mut bg, mut hg, mut hd) = (1, 1, 1, 1);
        let mut distance = 1;
        while bd + bg + hg + hd != 0 {
            // bas-droite
            if bd == 1 {
                let (x, y) = (from.0 + distance, from.1 + distance);
                if x == 7 || y == 7 {
                    bd = 0;
                }
                if let Some(piece) = self.board[x][y] {
                    if piece.get_color() != color {
                        available_move.push((MoveType::Normal, x, y));
                        bd = 0;
                    } else {
                        bd = 0;
                    }
                } else {
                    available_move.push((MoveType::Normal, x, y));
                }
            }
            // bas-gauche
            if bg == 1 {
                let (x, y) = (from.0 + distance, from.1 - distance);
                if x == 7 || y == 0 {
                    bg = 0;
                }
                if let Some(piece) = self.board[x][y] {
                    if piece.get_color() != color {
                        available_move.push((MoveType::Normal, x, y));
                        bg = 0;
                    } else {
                        bg = 0;
                    }
                } else {
                    available_move.push((MoveType::Normal, x, y));
                }
            }
            // haut-gauche
            if hg == 1 {
                let (x, y) = (from.0 - distance, from.1 - distance);
                if x == 0 || y == 0 {
                    hg = 0;
                }
                if let Some(piece) = self.board[x][y] {
                    if piece.get_color() != color {
                        available_move.push((MoveType::Normal, x, y));
                        hg = 0;
                    } else {
                        hg = 0;
                    }
                } else {
                    available_move.push((MoveType::Normal, x, y));
                }
            }
            // haut-droite
            if hd == 1 {
                let (x, y) = (from.0 - distance, from.1 + distance);
                if x == 0 || y == 7 {
                    hd = 0;
                }
                if let Some(piece) = self.board[x][y] {
                    if piece.get_color() != color {
                        available_move.push((MoveType::Normal, x, y));
                        hd = 0;
                    } else {
                        hd = 0;
                    }
                } else {
                    available_move.push((MoveType::Normal, x, y));
                }
            }
            distance += 1;
        }
        available_move
    }
}

