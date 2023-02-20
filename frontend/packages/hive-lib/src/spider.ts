import { uniqWith } from 'lodash';
import { eachSlideDirection, moveTile } from './board';
import { hexesEqual, includesHex } from './hex';
import { GameBoard, HexCoordinate, TileId } from './types';

/**
 * Get all coordinates that are valid moves for the given tile id at the given
 * coordinate acting as a spider. The spider rules state that the spider moves
 * exactly three spaces per turn, it cannot backtrack on itself, and that it may
 * only move around tile it is in direct contact with on each step of its move.
 *
 * @param board A game board.
 * @param tileId The id of the tile being moved.
 * @param coordinate The hex coordinate where the given tile is located.
 * @return An array of hex coordinates.
 */
export function getValidSpiderMoveCoordinates(
  board: GameBoard,
  tileId: TileId,
  coordinate: HexCoordinate
): HexCoordinate[] {
  // An array that stores valid hex coordinates
  const valid: HexCoordinate[] = [];

  const walk = (board: GameBoard, path: HexCoordinate[]) => {
    const current = path[path.length - 1];
    eachSlideDirection(board, current, (neighbor) => {
      if (!includesHex(path, neighbor)) {
        path.length === 3
          ? valid.push(neighbor)
          : walk(moveTile(board, tileId, current, neighbor), [
              ...path,
              neighbor
            ]);
      }
    });
  };

  walk(board, [coordinate]);
  return uniqWith(valid, hexesEqual);
}
