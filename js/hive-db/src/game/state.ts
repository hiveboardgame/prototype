import { getGameMoves, ColorKey } from 'hive-lib';

export interface GameState {
  // the game notation, defining the current state of the game board
  notation: string;
  // the color of the player whose turn it is
  turn: ColorKey;
  // the number of moves that have been played
  moveCount: number;
}

/**
 * Create a new game state object, optionally providing a game notation string.
 *
 * @param notation A game notation string.
 * @return A new GameState object.
 */
export const newGameState = (notation?: string): GameState => {
  if (!notation) notation = '';
  const moves = getGameMoves(notation);
  return {
    notation: notation,
    turn: moves.length % 2 === 0 ? 'w' : 'b',
    moveCount: moves.length
  };
};

/**
 * Get the game notation string from a game state object.
 *
 * @param state A game state object.
 * @return A game notation string.
 */
export const getGameNotation = (state: GameState) => state.notation;

/**
 * Get the color of the player whose turn it is from a game state object.
 *
 * @param state A game state object.
 * @return A player color.
 */
export const getColorTurn = (state: GameState) => state.turn;

/**
 * Get the total number of moves that have been played in the game defined by
 * the game notation string in a game state object.
 *
 * @param state A game state object.
 * @return The total number of moves that have been played
 */
export const getMoveCount = (state: GameState) => state.moveCount;
