import { Game, getGameIsEnded, getTurnUid } from 'hive-db';
import { HexCoordinate, Move, TileId } from 'hive-lib';

export interface GameProps {
  // the uid of the player viewing the game
  uid: string | null;
  // a string indicating the last time the user requested the board to be centered
  boardCentered: string;
  // the game data
  game: Game | null;
  // a flag to indicate that the game data has been updated but updates have not been viewed
  newMovesToView: boolean;
  // a list of valid next moves
  validNextMoves: Move[],
  // the proposed move that the player is currently viewing
  proposedMove: Move | null;
  // the coordinate of the proposed move
  proposedMoveCoordinate: HexCoordinate | null;
  // the id of the tile that is currently selected
  selectedTileId: TileId | null;
  // the index of the move up to which the game is being viewed
  upTo: number;
}

export function initializeGame(): GameProps {
  return {
    uid: null,
    boardCentered: '',
    game: null,
    newMovesToView: false,
    validNextMoves: [],
    proposedMove: null,
    proposedMoveCoordinate: null,
    selectedTileId: null,
    upTo: -1
  };
}

/**
 * Determine if the user viewing the game is allowed to propose a move.
 *
 * @param state The game state.
 */
export function canProposeMove(state: GameProps): boolean {
  const { uid, game, selectedTileId } = state;
  return (
    notNull(game) &&
    notNull(uid) &&
    notNull(selectedTileId) &&
    !getGameIsEnded(game) &&
    isViewingLatest(state) &&
    getTurnUid(game) === uid
  );
}

/**
 * Determine if latest move is being displayed.
 * @param state The game state.
 */
export function isViewingLatest(state: GameProps): boolean {
  return state.upTo === -1;
}

/**
 * Determine if some value is not null.
 * @param value The value to test.
 */
export function notNull<T>(value: T | null): value is T {
  return value !== null;
}
