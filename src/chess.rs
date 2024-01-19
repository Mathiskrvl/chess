#[derive(Copy, Clone, PartialEq)]
enum Color {
    White,
    Black,
    None
}

#[derive(Copy, Clone)]
enum Piece {
    King(Color, bool),
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
    // pit : Vec<Piece>
}

impl Chess {
    fn new() -> Self {
        Self {
            board: [
            [Some(Piece::Tour(Color::White)), Some(Piece::Cavalier(Color::White)), Some(Piece::Fou(Color::White)), Some(Piece::Queen(Color::White)), Some(Piece::King(Color::White, true)), Some(Piece::Fou(Color::White)), Some(Piece::Cavalier(Color::White)), Some(Piece::Tour(Color::White))],
            [Some(Piece::Pion(Color::White)), Some(Piece::Pion(Color::White)), Some(Piece::Pion(Color::White)), Some(Piece::Pion(Color::White)), Some(Piece::Pion(Color::White)), Some(Piece::Pion(Color::White)), Some(Piece::Pion(Color::White)), Some(Piece::Pion(Color::White))],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [Some(Piece::Pion(Color::Black)), Some(Piece::Pion(Color::Black)), Some(Piece::Pion(Color::Black)), Some(Piece::Pion(Color::Black)), Some(Piece::Pion(Color::Black)), Some(Piece::Pion(Color::Black)), Some(Piece::Pion(Color::Black)), Some(Piece::Pion(Color::Black))],
            [Some(Piece::Tour(Color::Black)), Some(Piece::Cavalier(Color::Black)), Some(Piece::Fou(Color::Black)), Some(Piece::Queen(Color::Black)), Some(Piece::King(Color::Black, true)), Some(Piece::Fou(Color::Black)), Some(Piece::Cavalier(Color::Black)), Some(Piece::Tour(Color::Black))]
            ],
            // pit: Vec::new()
        }
    }
    fn get_color(&self, from: (usize, usize)) -> Option<Color> {
        match self.board[from.0][from.1] {
            Some(Piece::King(color, _)) => Some(color),
            Some(Piece::Queen(color)) => Some(color),
            Some(Piece::Cavalier(color)) => Some(color),
            Some(Piece::Fou(color)) => Some(color),
            Some(Piece::Tour(color)) => Some(color),
            Some(Piece::Pion(color)) => Some(color),
            None => None
        }
    }
    fn action(&mut self, from: (usize, usize), to: (usize, usize)) {
        // if let Some(dead) = self.board[to.0][to.1] {
        //     self.pit.push(dead);
        // }
        self.board[to.0][to.1] = self.board[from.0][from.1];
        self.board[from.0][from.1] = None;
    }

    fn available_move(&self, from: (usize, usize)) -> Vec<(MoveType, usize, usize)> {
        match self.board[from.0][from.1] {
            Some(Piece::Tour(color)) => self.tour_available_move(color, from),
            Some(Piece::Fou(color)) => self.fou_available_move(color, from),
            Some(Piece::Cavalier(color)) => self.cavalier_available_move(color, from),
            Some(Piece::King(color, first_move)) => self.king_available_move(color, first_move, from),
            Some(Piece::Queen(color)) => self.queen_available_move(color, from),
            Some(Piece::Pion(color)) => self.pion_available_move(color, from),
            None => todo!()
        }
    }
    fn cavalier_available_move(&self, color: Color, from: (usize, usize)) -> Vec<(MoveType, usize, usize)> {
        let mut available_move = Vec::new();
        //let (x, y) = (from.0 as i8, from.1 as i8);
        for &direction in [(2i8,1i8), (1i8,2i8), (2i8,-1i8), (1i8,-2i8), (-1i8,-2i8), (-2i8,-1i8), (-1i8, 2i8), (-2i8, 1i8)].iter() {
            let (x, y) = (from.0 as i8 + direction.0, from.1 as i8 + direction.1);
            if x >= 0 && x <= 7 && y >= 0 && y <= 7 {
                if let Some(piece_color) = self.get_color((x as usize, y as usize)) {
                    if piece_color != color {
                        available_move.push((MoveType::Normal, x as usize, y as usize))
                    }
                }
                else {
                    available_move.push((MoveType::Normal, x as usize, y as usize))
                }
            }
        }
        available_move
    }
    fn king_available_move(&self, color: Color, first_move: bool, from: (usize, usize)) -> Vec<(MoveType, usize, usize)> {
        let mut available_move = Vec::new();
        //let (x, y) = (from.0 as i8, from.1 as i8);
        for &direction in [(-1i8,0i8), (-1i8,1i8), (0i8,1i8), (1i8,1i8), (1i8,0i8), (1i8,-1i8), (0i8, -1i8), (-1i8, -1i8)].iter() {
            let (x, y) = (from.0 as i8 + direction.0, from.1 as i8 + direction.1);
            if x >= 0 && x <= 7 && y >= 0 && y <= 7 {
                if let Some(piece_color) = self.get_color((x as usize, y as usize)) {
                    if piece_color != color {
                        available_move.push((MoveType::Normal, x as usize, y as usize))
                    }
                }
                else {
                    available_move.push((MoveType::Normal, x as usize, y as usize))
                }
            }
        } 
        if first_move {
            // todo rook moves
        }
        available_move
    }

    fn tour_available_move(&self, color: Color, from: (usize, usize)) -> Vec<(MoveType, usize, usize)> {
        let mut available_move = Vec::new();
        // haut
        for x in 0..from.0 {
            if let Some(piece_color) = self.get_color((x, from.1)) {
                if piece_color != color {
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
            if let Some(piece_color) = self.get_color((x, from.1)) {
                if piece_color != color {
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
            if let Some(piece_color) = self.get_color((from.0, y)) {
                if piece_color != color {
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
            if let Some(piece_color) = self.get_color((from.0, y)) {
                if piece_color != color {
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
    fn fou_available_move(&self, color: Color, from: (usize, usize)) -> Vec<(MoveType, usize, usize)> {
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
                if let Some(piece_color) = self.get_color((x, y)) {
                    if piece_color != color {
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
                if let Some(piece_color) = self.get_color((x, y)) {
                    if piece_color != color {
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
                if let Some(piece_color) = self.get_color((x, y)) {
                    if piece_color != color {
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
                if let Some(piece_color) = self.get_color((x, y)) {
                    if piece_color != color {
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
    fn queen_available_move(&self, color: Color, from: (usize, usize)) -> Vec<(MoveType, usize, usize)> {
        let mut available_move = self.tour_available_move(color, from);
        available_move.extend(self.fou_available_move(color, from));
        available_move
    }
    fn pion_available_move(&self, color: Color, from: (usize, usize)) -> Vec<(MoveType, usize, usize)> {
        todo!()
    }
}

