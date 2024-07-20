export class Block {
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
  private board_without_block: [];
  private block: Block;
  public board: [];

    constructor(n_rows, n_columns) {
      this.n_rows = n_rows;
      this.n_cols = n_columns;
      this.board_without_block = this.makeEmptyBoard()
      this.block = new Block([], "_")
      this.board = this.copyBoard(this.board_without_block)
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
      this.board_without_block = board
      this.drawBoard()
    }

    private copyBoard(board) {
      return board.map(row => row.slice());
    }

    public setBlock(block: Block) {
      this.block = block
      this.drawBoard()
    }

    private drawBoard() {
      let board = this.copyBoard(this.board_without_block)
      for (let pos of this.block.positions) {
        board[pos[0]][pos[1]] = this.block.colorCode
      }
      this.board = board
    }
}
