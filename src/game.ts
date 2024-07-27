export class Tetromino {
  public positions: [number, number][];
  public colorCode: string;

  constructor(positions: [number, number][], colorCode: string) {
    this.positions = positions;
    this.colorCode = colorCode;
  }
}

export class Board {
  readonly n_rows: number;
  readonly n_cols: number;
  private board_without_tetromino: string[][];
  private tetromino: Tetromino;
  public board: string[][];

    constructor(n_rows: number, n_columns: number) {
      this.n_rows = n_rows;
      this.n_cols = n_columns;
      this.board_without_tetromino = this.makeEmptyBoard()
      this.tetromino = new Tetromino([], "_")
      this.board = this.copyBoard(this.board_without_tetromino)
    }

    makeEmptyBoard(): string[][]{
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

    setBoard(board: string[][]) {
      this.board_without_tetromino = board
      this.drawBoard()
    }

    private copyBoard(board: string[][]) {
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
