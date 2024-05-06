use anchor_lang::error_code;

#[error_code]
pub enum TicTacToeError {
    GameAlreadyOver,
    NotPlayersTurn,
    GameAlreadyStarted,
    InvalidPlayer,
    GameNotInProgress,
    CellAlreadyTaken,
    RowOutOfBounds,
    ColumnOutOfBounds,
}
