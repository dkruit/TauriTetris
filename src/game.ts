export class Tetromino {
  public positions: [];
  public colorCode: string;

  constructor(positions, colorCode) {
    this.positions = positions;
    this.colorCode = colorCode;
  }
}

export class Board {
  readonly n_rows: number;
  readonly n_cols: number;
  private board_without_tetromino: [];
  private tetromino: Tetromino;
  public board: [];

    constructor(n_rows, n_columns) {
      this.n_rows = n_rows;
      this.n_cols = n_columns;
      this.board_without_tetromino = this.makeEmptyBoard()
      this.tetromino = new Tetromino([], "_")
      this.board = this.copyBoard(this.board_without_tetromino)
    }

    makeEmptyBoard(){
      let board = []
      for (let i = 0; i < this.n_rows; i++) {
        const row = [];
        for (let j = 0; j < this.n_cols; j++) {
          row.push("_");
        }
      board.push(row);
      }
      return board
    }

    setBoard(board) {
      this.board_without_tetromino = board
      this.drawBoard()
    }

    private copyBoard(board) {
      return board.map(row => row.slice());
    }

    public setTetromino(tetromino: Tetromino) {
      this.tetromino = tetromino
      this.drawBoard()
    }

    private drawBoard() {
      let board = this.copyBoard(this.board_without_tetromino)
      for (let pos of this.tetromino.positions) {
        board[pos[0]][pos[1]] = this.tetromino.colorCode
      }
      this.board = board
    }
}
