#[derive(Copy, Clone, PartialEq)]
enum Color {
    White,
    Black,
    // None
}

#[derive(Copy, Clone, PartialEq)]
enum Piece {
    King(Color, bool),
    Queen(Color),
    Cavalier(Color),
    Fou(Color),
    Tour(Color),
    Pion(Color),
    TemporaryPion(Color, usize),
}

#[derive(Debug)]
enum MoveType {
    Normal,
    DoublePush,
    EnPassant,
    Roque,
    Promotion,
}

struct Chess {
    board : [[Option<Piece>; 8]; 8],
    turn : Color,
    count_turn: usize,
    count_temporary_pion: usize
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
            turn : Color::White,
            count_turn: 0,
            count_temporary_pion: 0,
            // pit: Vec::new()
        }
    }
    fn finish_turn(&mut self) {
        if self.turn == Color::White {
            self.turn = Color::Black;
        } else {
            self.turn = Color::White
        }
        self.count_turn += 1;
        // can be improve
        if self.count_temporary_pion > 0 {
            for &i in [1usize, 7usize].iter() {
                for j in 0..8 {
                    if let Some(piece) = self.board[i][j] {
                        if piece == Piece::TemporaryPion(Color::Black, self.count_turn - 2) || piece == Piece::TemporaryPion(Color::White, self.count_turn - 2) {
                            self.board[i][j] = None;
                            self.count_temporary_pion -= 1;
                        }
                    }
                }
            }
        }
    }
    fn color_piece(&self, piece: Piece) -> Color {
        match piece {
            Piece::King(color, _) => color,
            Piece::Queen(color) => color,
            Piece::Cavalier(color) => color,
            Piece::Fou(color) => color,
            Piece::Tour(color) => color,
            Piece::Pion(color) => color,
            Piece::TemporaryPion(color, _) => color,
        }
    }
    fn get_color(&self, from: (usize, usize)) -> Option<Color> {
        if let Some(piece) = self.board[from.0][from.1] {
            if piece != Piece::TemporaryPion(Color::Black, self.count_turn - 1) && piece != Piece::TemporaryPion(Color::White, self.count_turn - 1) {
                return Some(self.color_piece(piece));
            }
        }
        None
    }
    // action en fonction des MoveType
    fn action(&mut self, from: (usize, usize), to: (usize, usize)) {
        let (available_move, move_type) = self.available_move(from);
        assert_eq!(available_move.len(), move_type.len(), "Les vecteur available_move : {:?} et move_type: {:?} n'ont pas la même longeur", available_move, move_type);
        if let Some(index) = available_move.iter().position(|&x| x == to) {
            match move_type[index] {
                MoveType::Normal => self.normal_move(from, to),
                MoveType::DoublePush => self.double_move(from, to),
                MoveType::Roque => self.roque_move(from, to),
                MoveType::Promotion => self.promotion_move(from, to),
                MoveType::EnPassant => self.passant_move(from, to)
            }
        }
    }
    fn normal_move(&mut self, from: (usize, usize), to: (usize, usize)) {
        self.board[to.0][to.1] = self.board[from.0][from.1];
        self.board[from.0][from.1] = None;
    }
    fn double_move(&mut self, from: (usize, usize), to: (usize, usize)) {
        if self.turn == Color::White {
            self.board[from.0 + 1][from.1] = Some(Piece::TemporaryPion(self.turn, self.count_turn));
        } else {
            self.board[from.0 - 1][from.1] = Some(Piece::TemporaryPion(self.turn, self.count_turn));
        }
        self.count_temporary_pion += 1;
        self.normal_move(from, to);
    }
    fn roque_move(&mut self, from: (usize, usize), to: (usize, usize)) {
        todo!()
    }
    fn promotion_move(&mut self, from: (usize, usize), to: (usize, usize)) {
        todo!()
    }
    fn passant_move(&mut self, from: (usize, usize), to: (usize, usize)) {
        self.board[to.0][to.1] = None;
        self.count_temporary_pion -= 1;
        self.board[from.0][to.1] = None;
        self.normal_move(from, to);
    }
    fn available_move(&self, from: (usize, usize)) -> (Vec<(usize, usize)>, Vec<MoveType>) {
        if let Some(from_color) = self.get_color(from) {
            if self.turn != from_color{
                (Vec::new(), Vec::new())
            } else {
                match self.board[from.0][from.1].unwrap() {
                    Piece::Tour(color) => self.tour_available_move(color, from),
                    Piece::Fou(color) => self.fou_available_move(color, from),
                    Piece::Cavalier(color) => self.cavalier_available_move(color, from),
                    Piece::King(color, first_move) => self.king_available_move(color, first_move, from),
                    Piece::Queen(color) => self.queen_available_move(color, from),
                    Piece::Pion(color) => self.pion_available_move(color, from),
                    Piece::TemporaryPion(_, _) => (Vec::new(), Vec::new()),
                }
            }
        } else {
            (Vec::new(), Vec::new())
        }
    }
    fn cavalier_available_move(&self, color: Color, from: (usize, usize)) -> (Vec<(usize, usize)>, Vec<MoveType>) {
        let mut available_move = Vec::new();
        let mut type_move = Vec::new();
        //let (x, y) = (from.0 as i8, from.1 as i8);
        for direction in [(2i8,1i8), (1i8,2i8), (2i8,-1i8), (1i8,-2i8), (-1i8,-2i8), (-2i8,-1i8), (-1i8, 2i8), (-2i8, 1i8)].into_iter() {
            let (x, y) = (from.0 as i8 + direction.0, from.1 as i8 + direction.1);
            if x >= 0 && x <= 7 && y >= 0 && y <= 7 {
                if let Some(piece_color) = self.get_color((x as usize, y as usize)) {
                    if piece_color != color {
                        available_move.push((x as usize, y as usize));
                        type_move.push(MoveType::Normal);
                    }
                }
                else {
                    available_move.push((x as usize, y as usize));
                    type_move.push(MoveType::Normal);
                }
            }
        }
        (available_move, type_move)
    }
    fn king_available_move(&self, color: Color, first_move: bool, from: (usize, usize)) -> (Vec<(usize, usize)>, Vec<MoveType>) {
        let mut available_move = Vec::new();
        let mut type_move = Vec::new();
        for &direction in [(-1i8,0i8), (-1i8,1i8), (0i8,1i8), (1i8,1i8), (1i8,0i8), (1i8,-1i8), (0i8, -1i8), (-1i8, -1i8)].iter() {
            let (x, y) = (from.0 as i8 + direction.0, from.1 as i8 + direction.1);
            if x >= 0 && x <= 7 && y >= 0 && y <= 7 {
                if let Some(piece_color) = self.get_color((x as usize, y as usize)) {
                    if piece_color != color {
                        available_move.push((x as usize, y as usize));
                        type_move.push(MoveType::Normal);
                    }
                }
                else {
                    available_move.push((x as usize, y as usize));
                    type_move.push(MoveType::Normal);
                }
            }
        } 
        if first_move {
            // Roque à gauche
            if let Some(color_tour) = self.get_color((from.0, from.1 - 4)) {
                if color == color_tour && self.get_color((from.0, from.1 - 1)) == None && self.get_color((from.0, from.1 - 2)) == None && self.get_color((from.0, from.1 - 3)) == None {
                    available_move.push((from.0, from.1 - 4));
                    type_move.push(MoveType::Roque);
                }
            }
            // Roque à droite
            if let Some(color_tour) = self.get_color((from.0, from.1 + 3)) {
                if color == color_tour && self.get_color((from.0, from.1 + 1)) == None && self.get_color((from.0, from.1 + 2)) == None {
                    available_move.push((from.0, from.1 + 3));
                    type_move.push(MoveType::Roque);
                }
            }
        }
        (available_move, type_move)
    }
    fn tour_available_move(&self, color: Color, from: (usize, usize)) -> (Vec<(usize, usize)>, Vec<MoveType>) {
        let mut available_move = Vec::new();
        let mut type_move = Vec::new();
        // haut
        for x in 0..from.0 {
            if let Some(piece_color) = self.get_color((x, from.1)) {
                if piece_color != color {
                    available_move.push((x, from.1));
                    type_move.push(MoveType::Normal);
                    break
                } else {
                    break
                }
            } else {
                available_move.push((x, from.1));
                type_move.push(MoveType::Normal);
            }
        }
        // bas
        for x in from.0..8 {
            if let Some(piece_color) = self.get_color((x, from.1)) {
                if piece_color != color {
                    available_move.push((x, from.1));
                    type_move.push(MoveType::Normal);
                    break
                } else {
                    break
                }
            } else {
                available_move.push((x, from.1));
                type_move.push(MoveType::Normal);
            }
        }
        // gauche
        for y in 0..from.1 {
            if let Some(piece_color) = self.get_color((from.0, y)) {
                if piece_color != color {
                    available_move.push((from.0, y));
                    type_move.push(MoveType::Normal);
                    break
                } else {
                    break
                }
            } else {
                available_move.push((from.0, y));
                type_move.push(MoveType::Normal);
            }
        }
        // droite
        for y in from.1..8 {
            if let Some(piece_color) = self.get_color((from.0, y)) {
                if piece_color != color {
                    available_move.push((from.0, y));
                    type_move.push(MoveType::Normal);
                    break
                } else {
                    break
                }
            } else {
                available_move.push((from.0, y));
                type_move.push(MoveType::Normal);
            }
        }
        // return
        (available_move, type_move)
    }
    fn fou_available_move(&self, color: Color, from: (usize, usize)) -> (Vec<(usize, usize)>, Vec<MoveType>) {
        let mut available_move = Vec::new();
        let mut type_move = Vec::new();
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
                        available_move.push((x, y));
                        type_move.push(MoveType::Normal);
                        bd = 0;
                    } else {
                        bd = 0;
                    }
                } else {
                    available_move.push((x, y));
                    type_move.push(MoveType::Normal);
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
                        available_move.push((x, y));
                        type_move.push(MoveType::Normal);
                        bg = 0;
                    } else {
                        bg = 0;
                    }
                } else {
                    available_move.push((x, y));
                    type_move.push(MoveType::Normal);
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
                        available_move.push((x, y));
                        type_move.push(MoveType::Normal);
                        hg = 0;
                    } else {
                        hg = 0;
                    }
                } else {
                    available_move.push((x, y));
                    type_move.push(MoveType::Normal);
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
                        available_move.push((x, y));
                        type_move.push(MoveType::Normal);
                        hd = 0;
                    } else {
                        hd = 0;
                    }
                } else {
                    available_move.push((x, y));
                    type_move.push(MoveType::Normal);
                }
            }
            distance += 1;
        }
        (available_move, type_move)
    }
    fn queen_available_move(&self, color: Color, from: (usize, usize)) -> (Vec<(usize, usize)>, Vec<MoveType>) {
        let (mut available_move,mut type_move) = self.tour_available_move(color, from);
        let (fou_available_move, fou_type_move) = self.fou_available_move(color, from);
        available_move.extend(fou_available_move);
        type_move.extend(fou_type_move);
        (available_move, type_move)
    }
    fn pion_available_move(&self, color: Color, from: (usize, usize)) -> (Vec<(usize, usize)>, Vec<MoveType>) {
        let mut available_move = Vec::new();
        let mut type_move = Vec::new();
        // pion blanc
        if color == Color::White {
            if self.get_color((from.0 + 1, from.1)) == None {
                if from.0 + 1 != 0 {
                    available_move.push((from.0 + 1, from.1));
                    type_move.push(MoveType::Normal);
                } else {
                    available_move.push((from.0 + 1, from.1));
                    type_move.push(MoveType::Promotion);
                }
            }
            if from.0 == 6 && self.get_color((from.0 + 2, from.1)) == None {
                available_move.push((from.0 + 2, from.1));
                type_move.push(MoveType::DoublePush);
            }
            if let Some(piece) = self.board[from.0 + 1][from.1 - 1] {
                if piece == Piece::TemporaryPion(Color::Black, self.count_turn - 1) {
                    available_move.push((from.0 + 1, from.1 - 1));
                    type_move.push(MoveType::EnPassant);
                }
                else if self.color_piece(piece) == Color::Black {
                    available_move.push((from.0 + 1, from.1 - 1));
                    type_move.push(MoveType::Normal);
                }
            }
            if let Some(piece) = self.board[from.0 + 1][from.1 + 1] {
                if piece == Piece::TemporaryPion(Color::Black, self.count_turn - 1) {
                    available_move.push((from.0 + 1, from.1 + 1));
                    type_move.push(MoveType::EnPassant);
                }
                else if self.color_piece(piece) == Color::Black {
                    available_move.push((from.0 + 1, from.1 + 1));
                    type_move.push(MoveType::Normal);
                }
            }
        }
        // pion noir
        if color == Color::Black {
            if self.get_color((from.0 - 1, from.1)) == None {
                if from.0 - 1 != 7 {
                    available_move.push((from.0 - 1, from.1));
                    type_move.push(MoveType::Normal);
                } else {
                    available_move.push((from.0 - 1, from.1));
                    type_move.push(MoveType::Promotion);
                }
            }
            if from.0 == 1 && self.get_color((from.0 - 2, from.1)) == None {
                available_move.push((from.0 - 2, from.1));
                type_move.push(MoveType::DoublePush);
            }
            if let Some(piece) = self.board[from.0 - 1][from.1 - 1] {
                if piece == Piece::TemporaryPion(Color::White, self.count_turn - 1) {
                    available_move.push((from.0 - 1, from.1 - 1));
                    type_move.push(MoveType::EnPassant);
                }
                else if self.color_piece(piece) == Color::White {
                    available_move.push((from.0 - 1, from.1 - 1));
                    type_move.push(MoveType::Normal);
                }
            }
            if let Some(piece) = self.board[from.0 - 1][from.1 + 1] {
                if piece == Piece::TemporaryPion(Color::White, self.count_turn - 1) {
                    available_move.push((from.0 - 1, from.1 + 1));
                    type_move.push(MoveType::EnPassant);
                }
                else if self.color_piece(piece) == Color::White {
                    available_move.push((from.0 - 1, from.1 + 1));
                    type_move.push(MoveType::Normal);
                }
            }
        }
        (available_move, type_move)
    }
}

