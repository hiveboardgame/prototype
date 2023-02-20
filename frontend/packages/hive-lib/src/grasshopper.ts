import { eachDirection, isSpaceOccupied } from './board';
import { hexesEqual, relativeHexCoordinate } from './hex';
import { GameBoard, HexCoordinate } from './types';

/**
 * Get all coordinates that are valid moves for a tile at the given coordinate
 * acting as a grasshopper. The grasshopper rules state that the grasshopper
 * jumps from its space over any number of pieces (at least one) to the next
 * unoccupied space along a straight row of joined pieces.
 *
 * @param board A game board.
 * @param coordinate The hex coordinate where the given tile is located.
 * @return An array of hex coordinates.
 */
export function getValidGrasshopperMoveCoordinates(
  board: GameBoard,
  coordinate: HexCoordinate
): HexCoordinate[] {
  const valid: HexCoordinate[] = [];
  eachDirection((direction) => {
    const neighbor = relativeHexCoordinate(coordinate, direction);
    let current = neighbor;
    while (isSpaceOccupied(board, current)) {
      current = relativeHexCoordinate(current, direction);
    }
    if (!hexesEqual(current, neighbor)) {
      valid.push(current);
    }
  });
  return valid;
}
