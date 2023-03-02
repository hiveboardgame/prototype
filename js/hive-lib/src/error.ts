import { HexCoordinate, Move, TileId } from './types';

/**
 * Generate an error indicating that two coordinates are not but should be adjacent.
 *
 * @param a The first coordinate.
 * @param b The second coordinate.
 */
export function coordinatesNotAdjacentError(
  a: HexCoordinate,
  b: HexCoordinate
): Error {
  return Error(`Tiles (${a.q},${a.r}) and (${b.q},${b.r}) are not adjacent.`);
}

/**
 * Generate an error for an invalid hex direction value.
 *
 * @param direction The invalid hex direction.
 */
export function invalidDirectionError(direction: number): Error {
  return Error(`${direction} is not a valid direction.`);
}

/**
 * Generate an error for a missing reference tile.
 *
 * @param move The move that contains the reference tile
 */
export function missingReferenceTileError(move: Move): Error {
  return Error(`Reference tile ${move.refId} is not on the game board.`);
}

/**
 * Generate an error for a coordinate that has no neighboring tiles to use as reference.
 *
 * @param coordinate The coordinate that has no neighbors.
 */
export function noReferenceTileError(coordinate: HexCoordinate): Error {
  return Error(
    `Unable to find a reference tile adjacent to (${coordinate.q},${coordinate.r}).`
  );
}

/**
 * Generate an error for a tile missing from the top of a stack.
 *
 * @param tileId The missing tile.
 * @param coordinate The coordinate where the tile should be located.
 */
export function tileNotOnTopError(
  tileId: TileId,
  coordinate: HexCoordinate
): Error {
  const { q, r } = coordinate;
  return Error(`Tile ${tileId} is not on top of the stack at (${q},${r})`);
}
