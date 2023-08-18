#[derive(Clone, Copy, Debug)]
struct Piece(u8);

enum PieceType {
    NoPiece = 0,
    Pawn = 1,
    Knight = 2,
    Rook = 3,
    Bishop = 4,
    Queen = 5,
    King = 6,
}

enum PieceColor {
    White,
    Dark,
}

trait ChessPiece {
    fn get_type(&self) -> PieceType;
    fn get_color(&self) -> PieceColor;
    fn set_color(&self, color: PieceColor) -> Piece;
    fn to_char(&self) -> char;
}

impl Piece {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Piece(PieceType::NoPiece as u8).set_color(PieceColor::White),
            '.' => Piece(PieceType::NoPiece as u8).set_color(PieceColor::Dark),

            'r' => Piece(PieceType::Rook as u8).set_color(PieceColor::Dark),
            'n' => Piece(PieceType::Knight as u8).set_color(PieceColor::Dark),
            'p' => Piece(PieceType::Pawn as u8).set_color(PieceColor::Dark),
            'b' => Piece(PieceType::Bishop as u8).set_color(PieceColor::Dark),
            'q' => Piece(PieceType::Queen as u8).set_color(PieceColor::Dark),
            'k' => Piece(PieceType::King as u8).set_color(PieceColor::Dark),

            'R' => Piece(PieceType::King as u8).set_color(PieceColor::White),
            'N' => Piece(PieceType::Knight as u8).set_color(PieceColor::White),
            'P' => Piece(PieceType::Pawn as u8).set_color(PieceColor::White),
            'B' => Piece(PieceType::Bishop as u8).set_color(PieceColor::White),
            'Q' => Piece(PieceType::Queen as u8).set_color(PieceColor::White),
            'K' => Piece(PieceType::King as u8).set_color(PieceColor::White),

            _ => panic!("Unknown piece character: {}", c),
        }
    }
}

impl ChessPiece for Piece {
    fn get_color(&self) -> PieceColor {
        let color_bit = 1 << 7;
        let Piece(n) = self;

        if n & color_bit != 0 {
            PieceColor::White
        } else {
            PieceColor::Dark
        }
    }

    fn set_color(&self, color: PieceColor) -> Piece {
        let color = if let PieceColor::White = color { 1 << 7 } else { 0 };
        let Piece(n) = self;
        Piece(*n | color)
    }

    fn get_type(&self) -> PieceType {
        let Piece(n) = self;

        // discard the color bit
        let n = n & 0b01111111;

        match n {
            0 => PieceType::NoPiece,
            1 => PieceType::Pawn,
            2 => PieceType::Knight,
            3 => PieceType::Rook,
            4 => PieceType::Bishop,
            5 => PieceType::Queen,
            6 => PieceType::King,
            _ => panic!("Unknown piece type: {:?}", self),
        }
    }

    fn to_char(&self) -> char {
        let color = self.get_color();
        let typ = self.get_type();

        match (typ, color) {
            // TODO: squares should not have to store their color, we calculate these on the fly
            (PieceType::NoPiece, PieceColor::White) => '#',
            (PieceType::NoPiece, PieceColor::Dark) => '.',
            (PieceType::Rook, PieceColor::Dark) => 'r',
            (PieceType::Rook, PieceColor::White) => 'R',
            (PieceType::Knight, PieceColor::Dark) => 'n',
            (PieceType::Knight, PieceColor::White) => 'N',
            (PieceType::Pawn, PieceColor::Dark) => 'p',
            (PieceType::Pawn, PieceColor::White) => 'P',
            (PieceType::Bishop, PieceColor::Dark) => 'b',
            (PieceType::Bishop, PieceColor::White) => 'B',
            (PieceType::Queen, PieceColor::Dark) => 'q',
            (PieceType::Queen, PieceColor::White) => 'Q',
            (PieceType::King, PieceColor::Dark) => 'k',
            (PieceType::King, PieceColor::White) => 'K',
        }
    }
}

#[derive(Debug)]
struct Board([Piece; 8 * 8]);

impl Board {
    fn from_string(string: &str) -> Board {
        let mut board: [Piece; 8 * 8] = [Piece(0); 8 * 8];

        let rows = string
            .split("\n")
            .filter(|c| !c.is_empty())
            .collect::<Vec<_>>();

        assert!(rows.len() == 8);

        let mut position = 0;

        for row in rows {
            // start from the top row (a8)
            assert!(row.len() == 8);

            for square_char in row.chars() {
                board[position] = Piece::from_char(square_char);
                position += 1;
            }
        }

        Board(board)
    }

    fn to_string(board: Board) -> String {
        let mut board_string = String::new();

        let Board(pieces) = board;

        let mut position = 1;

        for piece in pieces {
            board_string.push(piece.to_char());

            if position % 8 == 0 {
                board_string.push('\n');
            }

            position += 1;
        }

        board_string
    }

    fn square_color() {
        // TODO: given a coordinate index, return square color (7th bit on the left)
    }
}

// TODO: write tests
fn main() {
    let board_str = "
rnbkqbnr
pppppppp
.#.#.#.#
#.#.#.#.
.#.#.#.#
#.#.#.#.
PPPPPPPP
RNBKQBNR
";

    let board = Board::from_string(board_str);
    let serialized_board = Board::to_string(board);
    println!("{}", serialized_board);
}
