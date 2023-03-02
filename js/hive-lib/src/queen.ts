import { eachSlideDirection } from './board';
import { getBugLetter, getTileBug, getTileColor } from './tile';
import { ColorKey, GameBoard, HexCoordinate, TileId } from './types';

/**
 * Get all coordinates that are valid moves for the given tile id at the given
 * coordinate acting as a queen. The queen rules state that the queen can only
 * move one space per turn.
 *
 * @param board A game board.
 * @param coordinate The hex coordinate of the tile acting as a queen.
 * @return An array of hex coordinates.
 */
export function getValidQueenMoveCoordinates(
  board: GameBoard,
  coordinate: HexCoordinate
): HexCoordinate[] {
  const valid: HexCoordinate[] = [];
  eachSlideDirection(board, coordinate, (neighbor) => {
    valid.push(neighbor);
  });
  return valid;
}

/**
 * Determine if a tile is a queen, optionally of a specific color.
 *
 * @param tileId A tile id.
 * @param color A tile color.
 * @return true if the tile is a queen (of the specified color if provided), false otherwise.
 */
export function isQueen(tileId: TileId, color?: ColorKey): boolean {
  return (
    getBugLetter(getTileBug(tileId)) === 'Q' &&
    (color ? getTileColor(tileId) === color : true)
  );
}
