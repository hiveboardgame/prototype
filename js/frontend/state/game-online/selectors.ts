import { GameState } from './store';
import {
  buildBoard,
  findTileCoordinate,
  GameBoard,
  getGameMoves,
  getStacks,
  getStacksInHand,
  HexCoordinate,
  isMovePass,
  Move,
  relativeHexCoordinate,
  TileId
} from 'hive-lib';
import { createSelector } from '@reduxjs/toolkit';
import {
  Game,
  getBlackUserData,
  getColorTurn,
  getGameNotation,
  getGameOptions,
  getGameResult,
  getPlayerUsername,
  getWhiteUserData,
  UserData
} from 'hive-db';

/**
 * Get the last time the user requested the game board to be centered
 */
export const selectBoardCentered = (state: GameState): string =>
  state.boardCentered;

/**
 * Get the Game object from the store.
 */
export const selectGame = (state: GameState): Game | null => state.game;

/**
 * Get the set of valid next moves
 */
export const selectValidMoves = (state: GameState): Move[] | null => state.validNextMoves;

/**
 * Get the id of the currently selected tile.
 */
export const selectSelectedTileId = (state: GameState): TileId | null =>
  state.selectedTileId;

/**
 * Get the index of the move up to which the user wants to view. A value of -1
 * indicates that the user wants to view the latest move.
 */
export const selectUpTo = (state: GameState): number => state.upTo;

/**
 * Get flag indicating if the game data has been updated but the user has not
 * yet seen the updated game data.
 */
export const selectHasNewMovesToView = (state: GameState): boolean =>
  state.newMovesToView;

/**
 * Get the move proposed by the player.
 */
export const selectProposedMove = (state: GameState): Move | null =>
  state.proposedMove;

/**
 * Get the hex coordinate of the move proposed by the player.
 */
export const selectProposedMoveCoordinate = (
  state: GameState
): HexCoordinate | null => state.proposedMoveCoordinate;

/**
 * Get the user data for the user playing as black.
 */
export const selectBlackPlayer = createSelector(
  [selectGame],
  (game): UserData | null => (game ? getBlackUserData(game) : null)
);

/**
 * Get the user data for the user playing as white.
 */
export const selectWhitePlayer = createSelector(
  [selectGame],
  (game): UserData | null => (game ? getWhiteUserData(game) : null)
);

/**
 * Get the game notation string from the store.
 */
export const selectGameNotation = createSelector([selectGame], (game): string =>
  game ? getGameNotation(game) : ''
);

/**
 * Get the color whose turn it is from the store. Returns null if there is no
 * game data.
 */
export const selectColorTurn = createSelector([selectGame], (game) =>
  game ? getColorTurn(game) : null
);

/**
 * Get the array of game moves from the game notation.
 */
export const selectMoves = createSelector(
  [selectGameNotation],
  (notation): Move[] => getGameMoves(notation)
);

/**
 * Get the array of game moves, including the proposed move.
 */
export const selectDisplayMoves = createSelector(
  [selectMoves, selectProposedMove],
  (moves, proposed): Move[] => (proposed ? [...moves, proposed] : moves)
);

/**
 * Get the index of the move up to which the user wants to view.
 */
export const selectDisplayUpTo = createSelector(
  [selectUpTo, selectDisplayMoves],
  (upTo, moves) => (upTo === -1 ? moves.length : upTo)
);

/**
 * Get the game board at the most recently played move.
 */
export const selectGameBoard = createSelector(
  [selectMoves],
  (moves): GameBoard => buildBoard(moves)
);

/**
 * Get the game board at the move index that the user wants to view, including
 * the proposed move if it exists.
 */
export const selectDisplayGameBoard = createSelector(
  [selectDisplayMoves, selectDisplayUpTo],
  (moves, upTo): GameBoard => buildBoard(moves, upTo)
);

/**
 * Get an array of all tile stacks and their locations on the board.
 */
export const selectDisplayBoardStacks = createSelector(
  [selectDisplayGameBoard],
  getStacks
);

/**
 * Get the stacks of tiles in black's hand at the index of the move up to which
 * the user wants to view.
 */
export const selectDisplayBlackHand = createSelector(
  [selectGame, selectDisplayGameBoard],
  (game, board): TileId[][] =>
    game ? getStacksInHand(board, 'b', getGameOptions(game)) : []
);

/**
 * Get the stacks of tiles in white's hand at the index of the move up to which
 * the user wants to view.
 */
export const selectDisplayWhiteHand = createSelector(
  [selectGame, selectDisplayGameBoard],
  (game, board): TileId[][] =>
    game ? getStacksInHand(board, 'w', getGameOptions(game)) : []
);

/**
 * Determine if user is viewing previous moves.
 */
export const selectIsViewingHistory = createSelector(
  [selectDisplayMoves, selectDisplayUpTo],
  (moves, upTo) => moves.length !== upTo
);

/**
 * Get the last tile that was played by either player. If the first tile has not
 * been played or the last move was a pass, returns null.
 */
export const selectLastTilePlayed = createSelector(
  [selectMoves],
  (moves): TileId | null => {
    if (!moves.length) return null;
    const last = moves[moves.length - 1];
    if (isMovePass(last)) return null;
    return last.tileId;
  }
);

/**
 * Get an array of coordinates where the current player could move the currently
 * selected tile.
 */
export const selectValidMovesForTile = createSelector(
  [
    selectValidMoves,
    selectGame,
    selectGameBoard,
    selectIsViewingHistory,
    selectColorTurn,
    selectSelectedTileId,
  ],
  (
    moves,
    game,
    board,
    isHistory,
    player,
    selected,
  ): HexCoordinate[] => {
    if (!game || isHistory || !selected || !player) return [];
    return moves.filter(move => move.tileId === selected)
      .map(move => {
        if (move.refId === 'x') {
          return { r: 0, q: 0 };
        } else {
          return relativeHexCoordinate(findTileCoordinate(board, move.refId), move.dir)
        }
      });
  }
);

/**
 * Get the end result of the game. Returns the string "draw" if the game ended
 * in a draw, "tie" if the game ended in a tie, the uid of the winning player,
 * or the empty string if the game is not over.
 */
export const selectGameResult = createSelector([selectGame], (game) =>
  game ? getGameResult(game) : ''
);

/**
 * Get a message to display to the user based on the game state.
 */
export const selectDisplayMessage = createSelector(
  [
    selectGame,
    selectColorTurn,
    selectProposedMove,
    selectGameResult,
    selectDisplayUpTo
  ],
  (game, colorTurn, proposed, result, upTo) => {
    if (!game) return '';
    if (result !== '') {
      switch (result) {
        case 'tie':
          return 'Tie game!';
        case 'draw':
          return "It's a draw!";
        default:
          return getPlayerUsername(game, result) + ' wins!';
      }
    }
    if (proposed) return 'Click again to play move';
    if (upTo === -1) {
      if (colorTurn === 'b') return 'Black to move';
      if (colorTurn === 'w') return 'White to move';
    } else {
      return upTo % 2 === 0 ? 'White to move' : 'Black to move';
    }
  }
);
