use std::collections::HashMap;

// Arrays of ASCII symbols of chess pieces
// Sequence: King, Queen, Rook, Bishop, Knight, Pawn
const BLACK_PIECES: [&str; 6] = ["♔", "♕", "♖", "♗", "♘", "♙"];
const WHITE_PIECES: [&str; 6] = ["♚", "♛", "♜", "♝", "♞", "♟"];

// Coordinate struct
#[derive(Eq, Hash, Clone, Copy, PartialEq, Debug)]
pub struct Coordinate {
	pub row: i8,
	pub col: i8,
}

impl Coordinate {
	pub fn to_string(&self) -> String {
		// Convert coordinate to string
		let mut result = String::new();
		// Use cols to convert to letters
		const COLS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
		result.push(COLS[self.col as usize]);
		result.push_str(&(8 - self.row).to_string());
		return result;
	}
}

// Chess Pieces
#[derive(Eq, Hash, Clone, Copy, PartialEq)]
#[derive(Debug)]
pub enum Pieces {
	King,
	Queen,
	Rook,
	Bishop,
	Knight,
	Pawn,
	Empty,
}

#[derive(Hash, Eq, Clone, Copy, PartialEq)]
#[derive(Debug)]
pub enum Color {
	White,
	Black,
}

#[derive(Hash, Eq, Clone, Copy, PartialEq)]
#[derive(Debug)]
pub struct Piece {
	pub breed: Pieces,
	pub color: Color,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Board {
	pub board: [[Piece; 8]; 8], // 2D array of Pieces
	pub turn: Color,
	pub castling_black_king_side: bool,
	pub castling_black_queen_side: bool,
	pub castling_white_king_side: bool,
	pub castling_white_queen_side: bool,
	pub white_pieces: Box<HashMap<Coordinate, Piece>>,
	pub black_pieces: Box<HashMap<Coordinate, Piece>>,
	pub last_2_moves_pawn: Option<Coordinate>,
	halfmove_clock: i8,
	fullmove_number: i8,
}

#[allow(dead_code)]
impl Board {
	pub fn new() -> Self {
		Board {
			board: [[Piece {
				breed: Pieces::Empty,
				color: Color::White,
			}; 8]; 8],
			turn: Color::White,
			castling_black_king_side: true,
			castling_black_queen_side: true,
			castling_white_king_side: true,
			castling_white_queen_side: true,
			white_pieces: Box::new(HashMap::new()),
			black_pieces: Box::new(HashMap::new()),
			last_2_moves_pawn: None,
			halfmove_clock: 0,
			fullmove_number: 1,
		}
	}
	
	pub fn clear(&mut self) {
		for i in 0..8 {
			for j in 0..8 {
				self.board[i][j] = Piece {
					breed: Pieces::Empty,
					color: Color::White,
				};
			}
		}
		self.turn = Color::White;
		self.castling_black_king_side = false;
		self.castling_black_queen_side = false;
		self.castling_white_king_side = false;
		self.castling_white_queen_side = false;
		self.white_pieces = Box::new(HashMap::new());
		self.black_pieces = Box::new(HashMap::new());
		self.last_2_moves_pawn = None;
		self.halfmove_clock = 0;
		self.fullmove_number = 1;
	}
	
	pub fn load_fen(&mut self, fen: &str) {
		// function to parse fen string
		// source: https://en.wikipedia.org/wiki/forsyth%e2%80%93edwards_notation
		
		let mut fen_array = fen.split(' ');
		let fen_board = fen_array.next().unwrap();
		let fen_turn = fen_array.next().unwrap();
		let _fen_castling = fen_array.next().unwrap(); // todo
		let _fen_en_passant = fen_array.next().unwrap();
		let _fen_half_move = fen_array.next().unwrap(); // todo
		let _fen_full_move = fen_array.next().unwrap(); // todo
		
		self.clear();
		
		// change the turn
		if fen_turn == "w" {
			self.turn = Color::White;
		} else if fen_turn == "b" {
			self.turn = Color::Black;
		} else {
			panic!("invalid turn");
		}
		
		// parse the Board
		let mut row: i8 = 0;
		let mut col: i8 = 0;
		let mut piece: Option<Piece> = None;
		
		for c in fen_board.chars() {
			if c == '/' {
				row += 1;
				col = 0; // because at the end of the loop, col will be incremented
			} else if c.is_digit(10) {
				col += c.to_digit(10).unwrap() as i8;
			} else {
				piece = match c {
					'K' => Some(Piece {
						breed: Pieces::King,
						color: Color::White,
					}),
					'Q' => Some(Piece {
						breed: Pieces::Queen,
						color: Color::White,
					}),
					'R' => Some(Piece {
						breed: Pieces::Rook,
						color: Color::White,
					}),
					'B' => Some(Piece {
						breed: Pieces::Bishop,
						color: Color::White,
					}),
					'N' => Some(Piece {
						breed: Pieces::Knight,
						color: Color::White,
					}),
					'P' => Some(Piece {
						breed: Pieces::Pawn,
						color: Color::White,
					}),
					'k' => Some(Piece {
						breed: Pieces::King,
						color: Color::Black,
					}),
					'q' => Some(Piece {
						breed: Pieces::Queen,
						color: Color::Black,
					}),
					'r' => Some(Piece {
						breed: Pieces::Rook,
						color: Color::Black,
					}),
					'b' => Some(Piece {
						breed: Pieces::Bishop,
						color: Color::Black,
					}),
					'n' => Some(Piece {
						breed: Pieces::Knight,
						color: Color::Black,
					}),
					'p' => Some(Piece {
						breed: Pieces::Pawn,
						color: Color::Black,
					}),
					_ => Some(Piece {
						breed: Pieces::Empty,
						color: Color::White,
					}),
				};
			};
			
			// if Piece is not None, then insert the Piece
			if piece != None {
				self.place_piece(piece.unwrap(), row as usize, col as usize);
				piece = None;
				col += 1;
			}
		}
	}
	
	pub fn get_fen(&self) -> String {
		// function to convert the Board to FEN
		// source: https://en.wikipedia.org/wiki/Forsyth%e2%80%93Edwards_notation
		
		let mut fen_board: String = String::new();
		let mut empty_count: u8 = 0;
		
		for i in 0..8 {
			for j in 0..8 {
				if self.board[i][j].breed == Pieces::Empty {
					empty_count += 1;
				} else {
					if empty_count > 0 {
						fen_board.push_str(&empty_count.to_string());
						empty_count = 0;
					}
					
					match self.board[i][j].breed {
						Pieces::King => {
							if self.board[i][j].color == Color::White {
								fen_board.push_str("K");
							} else {
								fen_board.push_str("k");
							}
						}
						Pieces::Queen => {
							if self.board[i][j].color == Color::White {
								fen_board.push_str("Q");
							} else {
								fen_board.push_str("q");
							}
						}
						Pieces::Rook => {
							if self.board[i][j].color == Color::White {
								fen_board.push_str("R");
							} else {
								fen_board.push_str("r");
							}
						}
						Pieces::Bishop => {
							if self.board[i][j].color == Color::White {
								fen_board.push_str("B");
							} else {
								fen_board.push_str("b");
							}
						}
						Pieces::Knight => {
							if self.board[i][j].color == Color::White {
								fen_board.push_str("N");
							} else {
								fen_board.push_str("n");
							}
						}
						Pieces::Pawn => {
							if self.board[i][j].color == Color::White {
								fen_board.push_str("P");
							} else {
								fen_board.push_str("p");
							}
						}
						_ => panic!("Invalid piece"),
					}
				}
			}
			
			if empty_count > 0 {
				fen_board.push_str(&empty_count.to_string());
				empty_count = 0;
			}
			
			if i != 7 {
				fen_board.push_str("/");
			}
		}
		
		// Separated by spaces add info about the turn, castling, en passant, and halfmove clock
		fen_board.push_str(" ");
		if self.turn == Color::White {
			fen_board.push_str("w");
		} else {
			fen_board.push_str("b");
		}
		
		// TODO: check if castling is possible
		fen_board.push_str(" ");
		if self.castling_white_king_side {
			fen_board.push_str("K");
		}
		if self.castling_white_queen_side {
			fen_board.push_str("Q");
		}
		if self.castling_black_king_side {
			fen_board.push_str("k");
		}
		if self.castling_black_queen_side {
			fen_board.push_str("q");
		}
		
		fen_board.push_str(" ");
		if self.last_2_moves_pawn == None {
			fen_board.push_str("-");
		} else {
			fen_board.push_str(&self.last_2_moves_pawn.unwrap().to_string());
		}
		
		fen_board.push_str(" ");
		fen_board.push_str(&self.halfmove_clock.to_string());
		
		fen_board.push_str(" ");
		fen_board.push_str(&self.fullmove_number.to_string());
		
		return fen_board;
	}
	
	pub fn default() -> Self {
		let mut result = Board::new();
		result.load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
		return result;
	}
	
	pub fn place_piece(&mut self, piece: Piece, row: usize, col: usize) {
		// check bounds
		if row > 7 || col > 7 {
			panic!("invalid Coordinates {} {}", row, col);
		}
		
		self.board[row][col] = piece;

		// Add to the piece map
		let coord = Coordinate { row: row as i8, col: col as i8 };
    if piece.color == Color::White {
      self.white_pieces.insert(coord, piece);
    } else {
      self.black_pieces.insert(coord, piece);
    }
	}
	
	pub fn draw(&self) {
		// function to draw the Board
		/*
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		. . . . . . . .
		*/
		
		let mut array_of_chars_for_pieces: &[&str; 6];
		
		for row in 0..8 {
			for col in 0..8 {
				let piece: Piece = self.board[row][col];
				
				if piece.color == Color::White {
					array_of_chars_for_pieces = &WHITE_PIECES;
				} else {
					array_of_chars_for_pieces = &BLACK_PIECES;
				}
				
				// match the Piece to the correct character
				match piece.breed {
					Pieces::King => {
						print!("{} ", array_of_chars_for_pieces[0]);
					}
					
					Pieces::Queen => {
						print!("{} ", array_of_chars_for_pieces[1]);
					}
					
					Pieces::Rook => {
						print!("{} ", array_of_chars_for_pieces[2]);
					}
					
					Pieces::Bishop => {
						print!("{} ", array_of_chars_for_pieces[3]);
					}
					
					Pieces::Knight => {
						print!("{} ", array_of_chars_for_pieces[4]);
					}
					
					Pieces::Pawn => {
						print!("{} ", array_of_chars_for_pieces[5]);
					}
					
					Pieces::Empty => {
						print!("\x1b[39;49m.\x1b[0m ");
					}
				}
			}
			println!();
		}
	}
	
	pub fn diagonal_moves(&self, row: i8, col: i8, color: Color) -> Vec<Coordinate> {
		// function to get all diagonal moves
		let mut result: Vec<Coordinate> = Vec::new();
		let mut piece: Piece;
		let (mut new_row, mut new_col): (i8, i8);
		let (mut left_up, mut left_down, mut right_up, mut right_down) = (true, true, true, true);
		
		for delta in 1..8 {
			// left up
			if left_up {
				new_row = row + delta;
				new_col = col - delta;
				
				if (new_row >= 0 && new_col >= 0) && (new_row < 8 && new_col < 8) {
					piece = self.board[new_row as usize][new_col as usize];
					
					if piece.breed != Pieces::Empty {
						if piece.color != color {
							result.push(Coordinate {
								row: new_row,
								col: new_col,
							});
							left_up = false;
						} else if piece.color == color {
							left_up = false;
						}
					} else {
						result.push(Coordinate {
							row: new_row,
							col: new_col,
						});
					}
				} else {
					left_up = false;
				}
			}
			
			// left down
			if left_down {
				new_row = row - delta;
				new_col = col - delta;
				
				if (new_row >= 0 && new_col >= 0) && (new_row < 8 && new_col < 8) {
					piece = self.board[new_row as usize][new_col as usize];
					
					if piece.breed != Pieces::Empty {
						if piece.color != color {
							result.push(Coordinate {
								row: new_row,
								col: new_col,
							});
							left_down = false;
						} else if piece.color == color {
							left_down = false;
						}
					} else {
						result.push(Coordinate {
							row: new_row,
							col: new_col,
						});
					}
				} else {
					left_down = false;
				}
			}
			
			// right up
			if right_up {
				new_row = row + delta;
				new_col = col + delta;
				
				if (new_row >= 0 && new_col >= 0) && (new_row < 8 && new_col < 8) {
					piece = self.board[new_row as usize][new_col as usize];
					
					if piece.breed != Pieces::Empty {
						if piece.color != color {
							result.push(Coordinate {
								row: new_row,
								col: new_col,
							});
							right_up = false;
						} else if piece.color == color {
							right_up = false;
						}
					} else {
						result.push(Coordinate {
							row: new_row,
							col: new_col,
						});
					}
				} else {
					right_up = false;
				}
			}
			
			// right down
			if right_down {
				new_row = row - delta;
				new_col = col + delta;
				
				if (new_row >= 0 && new_col >= 0) && (new_row < 8 && new_col < 8) {
					piece = self.board[new_row as usize][new_col as usize];
					
					if piece.breed != Pieces::Empty {
						if piece.color != color {
							result.push(Coordinate {
								row: new_row,
								col: new_col,
							});
							right_down = false;
						} else if piece.color == color {
							right_down = false;
						}
					} else {
						result.push(Coordinate {
							row: new_row,
							col: new_col,
						});
					}
				} else {
					right_down = false;
				}
			}
		}
		
		return result;
	}
	
	pub fn linear_moves(&self, row: i8, col: i8, color: Color) -> Vec<Coordinate> {
		// function to get all linear moves
		let mut result: Vec<Coordinate> = Vec::new();
		let mut piece: Piece;
		let (mut new_row, mut new_col): (i8, i8);
		let (mut up, mut down, mut left, mut right) = (true, true, true, true);
		
		for delta in 1..8 {
			// up
			if up {
				new_row = row + delta;
				new_col = col;
				
				if (new_row >= 0 && new_col >= 0) && (new_row < 8 && new_col < 8) {
					piece = self.board[new_row as usize][new_col as usize];
					
					if piece.breed != Pieces::Empty {
						if piece.color != color {
							result.push(Coordinate {
								row: new_row,
								col: new_col,
							});
							up = false;
						} else if piece.color == color {
							up = false;
						}
					} else {
						result.push(Coordinate {
							row: new_row,
							col: new_col,
						});
					}
				} else {
					up = false;
				}
			}
			
			// down
			if down {
				new_row = row - delta;
				new_col = col;
				
				if (new_row >= 0 && new_col >= 0) && (new_row < 8 && new_col < 8) {
					piece = self.board[new_row as usize][new_col as usize];
					
					if piece.breed != Pieces::Empty {
						if piece.color != color {
							result.push(Coordinate {
								row: new_row,
								col: new_col,
							});
							down = false;
						} else if piece.color == color {
							down = false;
						}
					} else {
						result.push(Coordinate {
							row: new_row,
							col: new_col,
						});
					}
				} else {
					down = false;
				}
			}
			
			// left
			if left {
				new_row = row;
				new_col = col - delta;
				
				if (new_row >= 0 && new_col >= 0) && (new_row < 8 && new_col < 8) {
					piece = self.board[new_row as usize][new_col as usize];
					
					if piece.breed != Pieces::Empty {
						if piece.color != color {
							result.push(Coordinate {
								row: new_row,
								col: new_col,
							});
							left = false;
						} else if piece.color == color {
							left = false;
						}
					} else {
						result.push(Coordinate {
							row: new_row,
							col: new_col,
						});
					}
				} else {
					left = false;
				}
			}
			
			// right
			if right {
				new_row = row;
				new_col = col + delta;
				
				if (new_row >= 0 && new_col >= 0) && (new_row < 8 && new_col < 8) {
					piece = self.board[new_row as usize][new_col as usize];
					
					if piece.breed != Pieces::Empty {
						if piece.color != color {
							result.push(Coordinate {
								row: new_row,
								col: new_col,
							});
							right = false;
						} else if piece.color == color {
							right = false;
						}
					} else {
						result.push(Coordinate {
							row: new_row,
							col: new_col,
						});
					}
				} else {
					right = false;
				}
			}
		}
		
		return result;
	}
	
	pub fn apply_move(&mut self, starting: Coordinate, ending: Coordinate) -> Option<Piece> {
		// Returns the piece that was captured
		// get the Piece at the starting Coordinate
		let piece: Piece = self.board[starting.row as usize][starting.col as usize];
		let captured_piece: Piece = self.board[ending.row as usize][ending.col as usize];
		
		self.board[starting.row as usize][starting.col as usize] = Piece {
			breed: Pieces::Empty,
			color: Color::White,
		};

		self.board[ending.row as usize][ending.col as usize] = piece;

		if piece.breed == Pieces::Pawn && (ending.row - starting.row).abs() == 2 {
			self.last_2_moves_pawn = Some(ending);
		}
		
		// Modify the map of pieces
		if piece.color == Color::White {
			self.white_pieces.insert(ending, piece);
			self.white_pieces.remove(&starting);

      if captured_piece.breed != Pieces::Empty {
        self.black_pieces.remove(&ending);
      }
		} else {
			self.black_pieces.insert(ending, piece);
			self.black_pieces.remove(&starting);

      if captured_piece.breed != Pieces::Empty {
        self.white_pieces.remove(&ending);
      }
		}

		// Invert a turn
		if self.turn == Color::White {
			self.turn = Color::Black;
		} else {
			self.turn = Color::White;
		}
	
		if captured_piece.breed != Pieces::Empty {
			return Some(captured_piece);
		} else {
			return None;
		}
	}
	
	pub fn get_king_coord(&self, color: Color) -> Option<Coordinate> {
		let pieces_map: &HashMap<Coordinate, Piece> = if color == Color::White {
			&self.white_pieces
		} else {
			&self.black_pieces
		};
		
		for (coord, piece) in pieces_map.into_iter() {
			if piece.breed == Pieces::King {
				return Some(*coord);
			}
		}
		
		return None;
	}
	
	fn filter_check_moves(
		&self,
		piece_coord: Coordinate,
		moves: Vec<Coordinate>,
	) -> Vec<Coordinate> {
		let mut clone_board = Board::new();
		clone_board.load_fen(&self.get_fen()[..]);
		let piece = self.board[piece_coord.row as usize][piece_coord.col as usize];
		let mut result: Vec<Coordinate> = Vec::new();
		
		for move_coord in moves {
			let captured = clone_board.apply_move(piece_coord, move_coord);
			if !clone_board.is_in_check(piece.color) {
				result.push(move_coord);
			}
			// Undo the move by swapping the pieces back
			clone_board.apply_move(move_coord, piece_coord);
      if captured != None {
        clone_board.place_piece(captured.unwrap(), move_coord.row as usize, move_coord.col as usize);
      }
		}
		
		return result;
	}
	
	pub fn get_moves(&self, row: i8, col: i8) -> Vec<Coordinate> {
		let mut result: Vec<Coordinate> = Vec::new();
		let piece: Piece = self.board[row as usize][col as usize];
		
		match piece.breed {
			Pieces::King => {
				// * * * (-1 +1) (0 +1) (+1 +1)
				// * K * (-1 0) (0 0) (+1 0)
				// * * * (-1 -1) (0 -1) (+1 -1)
				
				result.push(Coordinate {
					row: row - 1,
					col: col + 1,
				});
				result.push(Coordinate {
					row,
					col: col + 1,
				});
				result.push(Coordinate {
					row: row + 1,
					col: col + 1,
				});
				
				result.push(Coordinate {
					row: row - 1,
					col,
				});
				result.push(Coordinate {
					row: row + 1,
					col,
				});
				
				result.push(Coordinate {
					row: row - 1,
					col: col - 1,
				});
				result.push(Coordinate {
					row,
					col: col - 1,
				});
				result.push(Coordinate {
					row: row + 1,
					col: col - 1,
				});
			}
			
			Pieces::Queen => {
				// * . * . *
				// . * * * .
				// * * Q * *
				// . * * * .
				// * . * . *
				
				// Appending diagonal and linear moves
				result.append(&mut self.diagonal_moves(row, col, piece.color));
				result.append(&mut self.linear_moves(row, col, piece.color));
			}
			
			Pieces::Rook => {
				result = self.linear_moves(row, col, piece.color);
			}
			
			Pieces::Bishop => {
				result = self.diagonal_moves(row, col, piece.color);
			}
			
			Pieces::Knight => {
				/*
				. . * . * . .
				. * . . . * .
				. . . K . . .
				. * . . . * .
				. . * . * . .
				*/
				
				result.push(Coordinate {
					row: row - 2,
					col: col - 1,
				});
				result.push(Coordinate {
					row: row - 2,
					col: col + 1,
				});
				result.push(Coordinate {
					row: row - 1,
					col: col - 2,
				});
				result.push(Coordinate {
					row: row - 1,
					col: col + 2,
				});
				result.push(Coordinate {
					row: row + 1,
					col: col - 2,
				});
				result.push(Coordinate {
					row: row + 1,
					col: col + 2,
				});
				result.push(Coordinate {
					row: row + 2,
					col: col - 1,
				});
				result.push(Coordinate {
					row: row + 2,
					col: col + 1,
				});
			}
			
			Pieces::Pawn => {
				/*
				. . .
				. . .
				. P .
				*/
				
				// Cases of 2 space moves
				if piece.color == Color::White && row == 6 {
					// If the way is not occupied, add the two spaces
					if self.board[(row - 1) as usize][col as usize].breed == Pieces::Empty {
						result.push(Coordinate {
							row: row - 1,
							col,
						});
						if self.board[(row - 2) as usize][col as usize].breed == Pieces::Empty {
							result.push(Coordinate {
								row: row - 2,
								col,
							});
						}
					}
				} else if piece.color == Color::Black && row == 1 {
					// If the way is not occupied, add the two spaces
					if self.board[(row + 1) as usize][col as usize].breed == Pieces::Empty {
						result.push(Coordinate {
							row: row + 1,
							col,
						});
						if self.board[(row + 2) as usize][col as usize].breed == Pieces::Empty {
							result.push(Coordinate {
								row: row + 2,
								col,
							});
						}
					}
				}
				
				// Cases of 1 space moves
				if piece.color == Color::White {
					// If the way is not occupied, add the one space
					if row > 0 && self.board[(row - 1) as usize][col as usize].breed == Pieces::Empty {
						result.push(Coordinate {
							row: row - 1,
							col,
						});
					}
				} else if piece.color == Color::Black {
					// If the way is not occupied, add the one space
					if row < 7 && self.board[(row + 1) as usize][col as usize].breed == Pieces::Empty {
						result.push(Coordinate {
							row: row + 1,
							col,
						});
					}
				}
				
				// Cases of attacKing moves
				// If diagonal is occupied and the Color is opposite, add it
				let mut diag_piece: Piece;
				if piece.color == Color::White {
          if row >= 1 && col >= 1 {
            // Diag left
            diag_piece = self.board[(row - 1) as usize][(col - 1) as usize];
            if diag_piece.breed != Pieces::Empty && diag_piece.color != piece.color {
              result.push(Coordinate {
                row: row - 1,
                col: col - 1,
              });
            }
          }
					
          if row >= 1 && col < 7 {
            // Diag right
            diag_piece = self.board[(row - 1) as usize][(col + 1) as usize];
            if diag_piece.breed != Pieces::Empty && diag_piece.color != piece.color {
              result.push(Coordinate {
                row: row - 1,
                col: col + 1,
              });
            }
          }
				} else if piece.color == Color::Black {
          if row < 7 && col >= 1 {
            // Diag left
            diag_piece = self.board[(row + 1) as usize][(col - 1) as usize];
            if diag_piece.breed != Pieces::Empty && diag_piece.color != piece.color {
              result.push(Coordinate {
                row: row + 1,
                col: col - 1,
              });
            }
          }
					
          if row < 7 && col < 7 {
            // Diag right
            diag_piece = self.board[(row + 1) as usize][(col + 1) as usize];
            if diag_piece.breed != Pieces::Empty && diag_piece.color != piece.color {
              result.push(Coordinate {
                row: row + 1,
                col: col + 1,
              });
            }
          }
				}
				
				if self.last_2_moves_pawn != None {
					/*
					. . .
					. . *
					. p P
					*/
					// Get a Piece on the right and on the left
					let (left_row, right_coords): (i8, i8) = (col - 1, col + 1);
					let new_row: i8 = if piece.color == Color::White {
						row - 1
					} else {
						row + 1
					};
					// Check bounds
					// if left is valid
					if left_row >= 0 && left_row < 8 {
						let left_piece: Piece = self.board[row as usize][left_row as usize];
						// New row equals row - 1 if color is White, row + 1 if color is Black
						if self.last_2_moves_pawn.unwrap() == (Coordinate { row, col: left_row }) {
							if left_piece.breed == Pieces::Pawn && left_piece.color != piece.color {
								result.push(Coordinate {
									row: new_row,
									col: left_row,
								});
							}
						}
					}
					
					// if right is valid
					if right_coords >= 0 && right_coords < 8 {
						let right_piece: Piece = self.board[row as usize][right_coords as usize];
						// New row equals row - 1 if color is White, row + 1 if color is Black
						if self.last_2_moves_pawn.unwrap()
						== (Coordinate {
							row,
							col: right_coords,
						})
						{
							if right_piece.breed == Pieces::Pawn && right_piece.color != piece.color
							{
								result.push(Coordinate {
									row: new_row,
									col: right_coords,
								});
							}
						}
					}
				}
			}
			
			Pieces::Empty => {}
		}
		
		// Filter out the Coordinates that are out of bounds
		result = result
		.into_iter()
		.filter(|coord| coord.row >= 0 && coord.row < 8 && coord.col >= 0 && coord.col < 8)
		.collect();

		// Filter out the Coordinates that hit a friendly piece
		result = result
		.into_iter()
		.filter(|coord| {
			let on_way_piece: Piece = self.board[coord.row as usize][coord.col as usize];
			on_way_piece.breed == Pieces::Empty || on_way_piece.color != piece.color
		}).collect();

		// Filter out duplicates
		result = result
		.into_iter()
		.fold(vec![], |mut acc, coord| {
			if !acc.contains(&coord) {
				acc.push(coord);
			}
			acc
		});
		
		// Filter out moves that lead to check
		return self.filter_check_moves(Coordinate { row, col }, result);
		// result
	}
	
	pub fn is_in_check(&self, color: Color) -> bool {
		let king_coord: Option<Coordinate> = self.get_king_coord(color);
		if king_coord.is_none() {
			return false;
		}
		
		for (coord, _) in match color {
        Color::White => self.black_pieces.iter(),
        Color::Black => self.white_pieces.iter()
      } {
			if self
			.get_moves(coord.row, coord.col)
			.contains(&king_coord.unwrap())
			{
				return true;
			}
		}
		
		return false;
	}
	
	pub fn is_in_checkmate(&mut self, color: Color) -> bool {
    let king_coord = self.get_king_coord(color);

    if king_coord == None {
      return false;
    }

    let friendly_pieces = if color == Color::White {
      &self.white_pieces
    } else {
      &self.black_pieces
    };

    // If friendly piece can avoid check
    for (coord, _) in friendly_pieces.iter() {
      let moves = self.get_moves(coord.row, coord.col);
      if moves.len() != 0 {
        return false;
      }
    }

		return true;
	}
}