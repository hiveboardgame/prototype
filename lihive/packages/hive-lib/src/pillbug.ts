import {
  eachDropDirection,
  eachSlideDirection,
  getStackHeight,
  isGated,
  moveTile
} from './board';
import { hexesEqual, relativeHexDirection } from './hex';
import { getBugLetter, getTileBug, getTileColor } from './tile';
import { ColorKey, GameBoard, HexCoordinate, TileId } from './types';

/**
 * Get all coordinates that are valid moves for the given tile id at the given
 * coordinate acting as a pillbug. The pillbug rules state that the pillbug can
 * only move one space per turn.
 *
 * @param board A game board.
 * @param coordinate The hex coordinate of the tile acting as a pillbug.
 * @return An array of hex coordinates.
 */
export function getValidPillbugMoveCoordinates(
  board: GameBoard,
  coordinate: HexCoordinate
): HexCoordinate[] {
  const valid: HexCoordinate[] = [];
  eachSlideDirection(board, coordinate, (coordinate) => {
    valid.push(coordinate);
  });
  return valid;
}

/**
 * Get all coordinates that are valid moves for the given tile id at the given
 * coordinate to be moved by an adjacent pillbug.
 *
 * @param board A game board.
 * @param tileId The id of the tile being moved.
 * @param coordinate The hex coordinate where the tile to be moved is located.
 * @param pillbug The adjacent hex coordinate where the pillbug is located.
 * @return An array of hex coordinates.
 */
export function getValidPillbugPushCoordinates(
  board: GameBoard,
  tileId: TileId,
  coordinate: HexCoordinate,
  pillbug: HexCoordinate
): HexCoordinate[] {
  const valid: HexCoordinate[] = [];
  const pickupDirection = relativeHexDirection(coordinate, pillbug);

  // The pillbug cannot pick up a tile from on top of a stack and it cannot pass
  // a tile through a gate to pick up that tile.
  if (
    getStackHeight(board, coordinate) > 1 ||
    isGated(board, coordinate, pickupDirection)
  )
    return [];

  // board = removeTile(board, tileId, coordinate);
  board = moveTile(board, tileId, coordinate, pillbug);

  eachDropDirection(board, pillbug, (neighbor, neighborStack) => {
    if (neighborStack.length === 0 && !hexesEqual(neighbor, coordinate))
      valid.push(neighbor);
  });

  return valid;
}

/**
 * Determine if a tile is a pillbug, optionally of a specific color.
 *
 * @param tileId A tile id.
 * @param color A tile color.
 * @return true if the tile is a pillbug (of the specified color if provided), false otherwise.
 */
export function isPillbug(tileId: TileId, color?: ColorKey): boolean {
  return (
    getBugLetter(getTileBug(tileId)) === 'P' &&
    (color ? getTileColor(tileId) === color : true)
  );
}
