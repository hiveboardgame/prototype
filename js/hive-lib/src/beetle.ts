import {
  eachClimbDirection,
  eachDropDirection,
  eachSlideDirection
} from './board';
import { GameBoard, HexCoordinate } from './types';

/**
 * Get all coordinates that are valid moves for the given tile id at the given
 * coordinate acting as a beetle. The beetle rules state that the beetle can
 * move only one space per turn but can also move on top of the hive.
 *
 * @param board A game board.
 * @param coordinate The hex coordinate of the tile acting as a beetle.
 * @return An array of hex coordinates.
 */
export function getValidBeetleMoveCoordinates(
  board: GameBoard,
  coordinate: HexCoordinate
): HexCoordinate[] {
  const valid: HexCoordinate[] = [];
  eachClimbDirection(board, coordinate, (neighbor) => {
    valid.push(neighbor);
  });
  eachSlideDirection(board, coordinate, (neighbor) => {
    valid.push(neighbor);
  });
  eachDropDirection(board, coordinate, (neighbor) => {
    valid.push(neighbor);
  });
  return valid;
}
