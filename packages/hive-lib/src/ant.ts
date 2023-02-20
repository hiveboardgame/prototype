import { eachSlideDirection, getTile, moveTile } from './board';
import { hexCoordinateKey } from './hex';
import { GameBoard, HexCoordinate } from './types';

/**
 * Get all coordinates that are valid moves for the tile at the given coordinate
 * acting as an ant. The ant rules state that an ant can move to any other
 * position around the hive as long as it can slide to that location.
 *
 * @param board A game board.
 * @param coordinate The hex coordinate of the tile acting as an ant.
 * @return An array of hex coordinates.
 */
export function getValidAntMoveCoordinates(
  board: GameBoard,
  coordinate: HexCoordinate
): HexCoordinate[] {
  // An array that stores the valid coordinates
  const valid: HexCoordinate[] = [];

  // A set to track coordinates the ant has visited, starting with its current location
  const visitedCoords = new Set<string>([hexCoordinateKey(coordinate)]);

  // A function that moves the ant along the outside of the hive
  const walk = (board: GameBoard, coordinate: HexCoordinate) => {
    eachSlideDirection(board, coordinate, (neighbor) => {
      const key = hexCoordinateKey(neighbor);
      if (!visitedCoords.has(key)) {
        const tileId = getTile(board, coordinate);
        if (!tileId) throw Error('This should never happen');
        visitedCoords.add(key);
        valid.push(neighbor);
        walk(moveTile(board, tileId, coordinate, neighbor), neighbor);
      }
    });
  };

  // Walk the hive
  walk(board, coordinate);

  return valid;
}
