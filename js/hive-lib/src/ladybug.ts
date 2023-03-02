import { uniqWith } from 'lodash';
import {
  eachClimbDirection,
  eachDropDirection,
  eachSlideDirection,
  moveTile
} from './board';
import { hexesEqual, includesHex } from './hex';
import { getBugLetter, getTileBug, getTileColor } from './tile';
import { ColorKey, GameBoard, HexCoordinate, TileId } from './types';

/**
 * Get all coordinates that are valid moves for the given tile id at the given
 * coordinate acting as a ladybug. The ladybug rules state that the ladybug
 * moves three spaces: two on top of the hive, then one down.
 *
 * @param board A game board.
 * @param tileId The id of the tile being moved.
 * @param coordinate The hex coordinate where the given tile is located.
 * @return An array of hex coordinates.
 */
export function getValidLadybugMoveCoordinates(
  board: GameBoard,
  tileId: TileId,
  coordinate: HexCoordinate
): HexCoordinate[] {
  const valid: HexCoordinate[] = [];
  const walk = (board: GameBoard, path: HexCoordinate[]) => {
    const current = path[path.length - 1];
    if (path.length === 1) {
      eachClimbDirection(board, current, (neighbor) => {
        walk(moveTile(board, tileId, current, neighbor), [...path, neighbor]);
      });
    } else if (path.length < 3) {
      eachSlideDirection(board, current, (neighbor) => {
        if (!includesHex(path, neighbor)) {
          walk(moveTile(board, tileId, current, neighbor), [...path, neighbor]);
        }
      });
      eachClimbDirection(board, current, (neighbor) => {
        if (!includesHex(path, neighbor)) {
          walk(moveTile(board, tileId, current, neighbor), [...path, neighbor]);
        }
      });
      eachDropDirection(board, current, (neighbor, neighborStack) => {
        if (neighborStack.length > 0 && !includesHex(path, neighbor)) {
          walk(moveTile(board, tileId, current, neighbor), [...path, neighbor]);
        }
      });
    } else if (path.length === 3) {
      eachDropDirection(board, current, (neighbor, neighborStack) => {
        if (neighborStack.length === 0 && !includesHex(path, neighbor))
          valid.push(neighbor);
      });
    }
  };

  walk(board, [coordinate]);
  return uniqWith(valid, hexesEqual);
}

/**
 * Determine if a tile is a ladybug, optionally of a specific color.
 *
 * @param tileId A tile id.
 * @param color A tile color.
 * @return true if the tile is a ladybug (of the specified color if provided), false otherwise.
 */
export function isLadybug(tileId: TileId, color?: ColorKey): boolean {
  return (
    getBugLetter(getTileBug(tileId)) === 'L' &&
    (color ? getTileColor(tileId) === color : true)
  );
}
