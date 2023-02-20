import produce from 'immer';
import { forOwn, isEmpty } from 'lodash';
import { missingReferenceTileError, tileNotOnTopError } from './error';
import {
  hexCoordinateKey,
  includesHex,
  relativeHexCoordinate,
  toHexDirection
} from './hex';
import { isMovePass } from './move';
import { getTileColor, getTopTile } from './tile';
import {
  ColorKey,
  CoordFn,
  DirectionFn,
  GameBoard,
  HexCoordinate,
  Move,
  NeighborFn,
  SpaceFn,
  StackCoordinate,
  StackFn,
  TileId
} from './types';

/**
 * Build a game board from a sequence of moves, optionally up to but not
 * including a certain placement index.
 *
 * @param moves An array of game moves
 * @param upTo The move index up to which the board will be built
 */
export function buildBoard(moves: Move[], upTo?: number): GameBoard {
  return produce<GameBoard>({}, (draft) => {
    upTo = upTo ?? moves.length;
    moves.slice(0, upTo).forEach((move) => {
      // No tiles are placed or moved on a pass
      if (isMovePass(move)) return;

      // Extract the data we need from the move object
      const { tileId, refId, dir } = move;

      if (isEmpty(draft)) {
        // The first tile placed on the board is always at (0, 0)
        _placeTile(draft, tileId, { q: 0, r: 0 });
      } else {
        // Get the coordinates of the reference tile and the tile being placed/moved
        const refCoordinate = findTileCoordinate(draft, refId);
        const tileCoordinate = findTileCoordinate(draft, tileId);

        // Throw an error if the reference tile is not on the board
        if (!refCoordinate) throw missingReferenceTileError(move);

        // Get the coordinate where the tile will be placed
        const targetCoordinate = relativeHexCoordinate(refCoordinate, dir);

        // If the tile is already on the board, move it, otherwise place it
        if (!tileCoordinate) {
          _placeTile(draft, tileId, targetCoordinate);
        } else {
          _moveTile(draft, tileId, tileCoordinate, targetCoordinate);
        }
      }
    });
  });
}

/**
 * Iterate over all neighboring stacks onto which the tile at the given
 * coordinate could climb, calling iteratee for each. The following conditions
 * must be met for a tile to be able to climb onto a neighboring stack (and are
 * checked by this iterator):
 *  - The height of the destination stack must be equal to or greater than the
 *    height of the source stack.
 *  - There must not be a gate between the source and destination coordinates.
 *
 *  Iteratee functions may exit iteration early by explicitly returning false.
 *
 * @param board A game board.
 * @param coordinate The coordinate whose neighbors will be iterated over.
 * @param iteratee The function invoked per iteration.
 * @return false if the iteration ended early, true otherwise. If there is no
 * stack at the given coordinate, true is returned.
 */
export function eachClimbDirection(
  board: GameBoard,
  coordinate: HexCoordinate,
  iteratee: NeighborFn
): boolean {
  const stack = getStack(board, coordinate);
  if (!stack) return true;
  return eachDirection((direction) => {
    const neighbor = relativeHexCoordinate(coordinate, direction);
    const neighborStack = getStack(board, neighbor) || [];
    return neighborStack.length >= stack.length &&
      !isGated(board, coordinate, direction)
      ? iteratee(neighbor, neighborStack, direction)
      : true;
  });
}

/**
 * Iterate over all six hex direction values (1 through 6). The iteratee is
 * invoked with one argument, the direction value. Iteratee functions may exit
 * iteration early by explicitly returning false.
 *
 * @param iteratee The function invoked per iteration.
 * @return false if iteration ended early, true otherwise.
 */
export function eachDirection(iteratee: DirectionFn): boolean {
  for (let i = 1; i <= 6; ++i) {
    if (iteratee(i) === false) return false;
  }
  return true;
}

/**
 * Iterate over all neighboring coordinate into which the tile at the given
 * coordinate could drop, calling iteratee for each. The following conditions
 * must be met for a tile to be able to drop into a neighboring coordinate (and
 * are checked by this iterator):
 *  - The height of the source stack must be at least two greater than the
 *    height of the destination stack.
 *  - There must not be a gate between the source and destination coordinates.
 *
 *  Iteratee functions may exit iteration early by explicitly returning false.
 *
 * @param board A game board.
 * @param coordinate The coordinate whose neighbors will be iterated over.
 * @param iteratee The function invoked per iteration.
 * @return false if the iteration ended early, true otherwise. If there is no
 * stack at the given coordinate, true is returned.
 */
export function eachDropDirection(
  board: GameBoard,
  coordinate: HexCoordinate,
  iteratee: NeighborFn
): boolean {
  const stack = getStack(board, coordinate);
  if (!stack) return true;
  return eachDirection((direction) => {
    const neighbor = relativeHexCoordinate(coordinate, direction);
    const neighborStack = getStack(board, neighbor) || [];
    return stack.length - neighborStack.length >= 2
      ? iteratee(neighbor, neighborStack, direction)
      : true;
  });
}

/**
 * Iterate over all spaces surrounding a given coordinate. The iteratee is
 * invoked with two arguments: *(coordinate, stack)*. The *stack* argument will
 * be undefined if there is no stack at the corresponding *coordinate*. Iteratee
 * functions may exit iteration early by explicitly returning false.
 *
 * @param board The game board.
 * @param coordinate The coordinate whose neighboring spaces will be iterated over.
 * @param iteratee The function invoked per iteration.
 * @return false if iteration ended early, true otherwise.
 */
export function eachNeighboringSpace(
  board: GameBoard,
  coordinate: HexCoordinate,
  iteratee: SpaceFn
): boolean {
  return eachDirection((direction) => {
    const neighbor = relativeHexCoordinate(coordinate, direction);
    return iteratee(neighbor, getStack(board, neighbor)) !== false;
  });
}

/**
 * Iterate over all tile stacks surrounding the given coordinate. The iteratee
 * is invoked with two arguments: *(coordinate, stack)*. Iteratee functions may
 * exit iteration early by explicitly returning false.
 *
 * @param board The game board.
 * @param coordinate The coordinate whose neighboring stacks will be iterated over.
 * @param iteratee The function invoked per iteration.
 * @return false if iteration ended early, true otherwise.
 */
export function eachNeighboringStack(
  board: GameBoard,
  coordinate: HexCoordinate,
  iteratee: StackFn
): boolean {
  return eachDirection((direction) => {
    const neighbor = relativeHexCoordinate(coordinate, direction);
    const stack = getStack(board, neighbor);
    return stack ? iteratee(neighbor, stack) !== false : true;
  });
}

/**
 * Iterate over all neighboring coordinates into which the tile at the given
 * coordinate could slide, calling iteratee for each. The following conditions
 * must be met for a tile to be able to slide into a neighboring coordinate (and
 * are checked by this iterator):
 *  - The height of the source coordinate stack must be exactly one greater than
 *    the height of the destination coordinate stack.
 *  - There must not be a gate between the source and destination coordinates.
 *  - The source and destination coordinates must share an occupied neighbor, OR;
 *  - The source and destination coordinates must both have stack heights
 *    greater than one.
 *
 *  Iteratee functions may exit iteration early by explicitly returning false.
 *
 * @param board A game board.
 * @param coordinate The coordinate whose neighbors will be iterated over.
 * @param iteratee The function invoked per iteration.
 * @return false if iteration ended early, true otherwise. If there is no stack
 * at the given coordinate, true is returned.
 */
export function eachSlideDirection(
  board: GameBoard,
  coordinate: HexCoordinate,
  iteratee: NeighborFn
): boolean {
  const stack = getStack(board, coordinate);
  const neighbors = getOccupiedNeighbors(board, coordinate);
  const isOccupiedNeighbor = (coordinate: HexCoordinate) =>
    includesHex(neighbors, coordinate);
  if (!stack) return true;
  return eachDirection((direction) => {
    const neighbor = relativeHexCoordinate(coordinate, direction);
    const neighborStack = getStack(board, neighbor) || [];
    return stack.length - neighborStack.length === 1 &&
      !isGated(board, coordinate, direction) &&
      (stack.length > 1 ||
        someNeighboringSpace(board, neighbor, isOccupiedNeighbor))
      ? iteratee(neighbor, neighborStack, direction)
      : true;
  });
}

/**
 * Iterate over all occupied tile stacks on a game board. The iteratee is
 * invoked with a hex coordinate and the stack located at that coordinate.
 * Iteratee functions may exit iteration early by explicitly returning false.
 *
 * @param board The game board to iterate over.
 * @param iteratee The function invoked per iteration.
 * @return false if iteration ended early, true otherwise.
 */
export function eachStack(board: GameBoard, iteratee: StackFn): boolean {
  let iter: any = true;
  forOwn(board, (rs, q) => {
    forOwn(rs, (stack, r) => {
      iter = iteratee({ q: +q, r: +r }, stack);
      return iter !== false;
    });
    return iter !== false;
  });
  return iter !== false;
}

/**
 * Iterate over all unoccupied coordinates that are adjacent to occupied spaces
 * on a board. Each coordinate will only be visited once. Iteratee functions
 * may exit iteration early by explicitly returning false.
 *
 * @param board A game board.
 * @param iteratee The function invoked per iteration.
 * @return false if iteration ended early, true otherwise.
 */
export function eachUnoccupiedCoordinate(
  board: GameBoard,
  iteratee: CoordFn
): boolean {
  const visited = new Set<string>();
  return eachStack(board, (coordinate) => {
    return eachNeighboringSpace(board, coordinate, (neighbor, stack) => {
      const key = hexCoordinateKey(neighbor);
      if (!visited.has(key) && !stack) {
        visited.add(key);
        return iteratee(neighbor) !== false;
      }
    });
  });
}

/**
 * Determine if some predicate holds true for every space surrounding some coordinate.
 *
 * @param board A game board.
 * @param coordinate The coordinate whose neighbors will be tested.
 * @param predicate A predicate to test.
 * @return true if predicate evalutes to true for every neighbor, false otherwise.
 */
export function everyNeighbor(
  board: GameBoard,
  coordinate: HexCoordinate,
  predicate: SpaceFn
): boolean {
  return eachNeighboringSpace(
    board,
    coordinate,
    (neighbor, space) => predicate(neighbor, space) === true
  );
}

/**
 * Find the first neighboring hex coordinate for which a predicate holds true.
 *
 * @param board A game board.
 * @param coordinate The coordinate whose neighbors will be searched.
 * @param predicate The predicate to test on each neighbor.
 * @return the first hex coordinate for which predicate returns true.
 */
export function findNeighborCoordinate(
  board: GameBoard,
  coordinate: HexCoordinate,
  predicate: SpaceFn
): HexCoordinate | undefined {
  let result: HexCoordinate | undefined;
  eachNeighboringSpace(board, coordinate, (neighbor, stack) => {
    if (predicate(neighbor, stack)) {
      result = neighbor;
      return false;
    }
  });
  return result;
}

/**
 * Find the hex coordinate location of a tile on a game board. If the tile is
 * not on the board, returns null.
 *
 * Note that this function searches complete stacks, not just the tops of
 * stacks.
 *
 * @param board The game board to search.
 * @param tileId The tile id to search for.
 * @return The location of the tile, or null if the tile is not on the board.
 */
export function findTileCoordinate(
  board: GameBoard,
  tileId: TileId
): HexCoordinate | null {
  let coordinate: HexCoordinate | null = null;
  eachStack(board, (coord, stack) => {
    if (stack.includes(tileId)) {
      coordinate = coord;
      return false;
    }
  });
  return coordinate;
}

/**
 * Get the result of a game.
 *
 * @param board A game board.
 * @return The empty string if the game is not over, 'black' if black wins,
 * 'white' if white wins, and 'tie' if the game has ended in a tie.
 */
export function getGameResult(
  board: GameBoard
): 'black' | 'white' | 'tie' | '' {
  const blackSurrounded = isQueenSurrounded(board, 'b');
  const whiteSurrounded = isQueenSurrounded(board, 'w');
  if (blackSurrounded && whiteSurrounded) return 'tie';
  if (blackSurrounded) return 'white';
  if (whiteSurrounded) return 'black';
  return '';
}

/**
 * Get the total number of tiles on the board, optionally of a specified color.
 *
 * @param board A game board.
 * @param color The color tile to count.
 * @return The number of total tiles on the board if no color was provided,
 * otherwise the number of tiles of the given color.
 */
export function getNumTiles(board: GameBoard, color?: ColorKey): number {
  let count = 0;
  eachStack(board, (_, stack) => {
    if (!color) {
      count += stack.length;
    } else {
      stack.forEach((tileId) => {
        if (getTileColor(tileId) === color) count += 1;
      });
    }
  });
  return count;
}

/**
 * Get an array of all occupied hex coordinates on a game board.
 *
 * @param board A game board.
 * @return An array of coordinates that contain tile stacks.
 */
export function getOccupiedCoordinates(board: GameBoard): HexCoordinate[] {
  const coordinates: HexCoordinate[] = [];
  eachStack(board, (coordinate) => coordinates.push(coordinate));
  return coordinates;
}

/**
 * Get an array of all occupied hex coordinates surrounding a given coordinate.
 *
 * @param board A game board.
 * @param coordinate The coordinate whose neighbors will be searched.
 * @return An array of coordinate surrounding the given coordinate that are occupied.
 */
export function getOccupiedNeighbors(
  board: GameBoard,
  coordinate: HexCoordinate
): HexCoordinate[] {
  const coordinates: HexCoordinate[] = [];
  eachNeighboringStack(board, coordinate, (neighbor) => {
    coordinates.push(neighbor);
  });
  return coordinates;
}

/**
 * Get the stack of tiles located at the given hex coordinate.
 *
 * @param board The game board
 * @param coordinate The hex coordinate
 * @return The tile stack if there is at least one tile at the given coordinate, otherwise undefined.
 */
export function getStack(
  board: GameBoard,
  coordinate: HexCoordinate
): TileId[] | undefined {
  const { q, r } = coordinate;
  const rs = board[q];
  return rs ? rs[r] ?? undefined : undefined;
}

/**
 * Get an array of all stacks associated with their locations on a board.
 *
 * @param board A game board.
 * @return An array of stacks associated with their coordinates.
 */
export function getStacks(board: GameBoard): StackCoordinate[] {
  const stacks: StackCoordinate[] = [];
  eachStack(board, (coordinate, stack) => stacks.push({ coordinate, stack }));
  return stacks;
}

/**
 * Get the height of the stack located at the given hex coordinate.
 *
 * @param board The game board
 * @param coordinate The hex coordinate
 */
export function getStackHeight(
  board: GameBoard,
  coordinate: HexCoordinate
): number {
  const stack = getStack(board, coordinate);
  return stack ? stack.length : 0;
}

/**
 * Get the difference in stack height between two coordinates.
 *
 * @param board The game board.
 * @param a The first coordinate.
 * @param b The second coordinate.
 * @return The difference in stack heights (a-b).
 */
export function getStackHeightDifference(
  board: GameBoard,
  a: HexCoordinate,
  b: HexCoordinate
): number {
  return getStackHeight(board, a) - getStackHeight(board, b);
}

/**
 * Get the tile on top of the stack at the given coordinate.
 *
 * @param board The game board.
 * @param coordinate The hex coordinate.
 * @return The tile on top of that stack at the given coordinate if there is one, undefined otherwise.
 */
export function getTile(
  board: GameBoard,
  coordinate: HexCoordinate
): TileId | undefined {
  const stack = getStack(board, coordinate);
  return stack ? getTopTile(stack) : undefined;
}

/**
 * Get an array of all tile ids on a game board.
 *
 * @param board A game board
 * @return An array of all tile ids that are on the game board.
 */
export function getTiles(board: GameBoard): TileId[] {
  const tiles: TileId[] = [];
  eachStack(board, (_, ids) => tiles.push(...ids));
  return tiles;
}

/**
 * Get a list of the unoccupied coordinates touching the hive .
 *
 * @param board A game board.
 * @return An array of hex coordinates that are touching the hive and are not occupied by tiles.
 */
export function getUnoccupiedCoordinates(board: GameBoard): HexCoordinate[] {
  const visited = new Set<string>();
  const surr: HexCoordinate[] = [];
  getOccupiedCoordinates(board).forEach((coordinate) => {
    eachNeighboringSpace(board, coordinate, (neighbor, stack) => {
      const key = hexCoordinateKey(neighbor);
      if (!stack && !visited.has(key)) {
        visited.add(key);
        surr.push(neighbor);
      }
    });
  });
  return surr;
}

/**
 * Get an array of all unoccupied hex coordinates surrounding a given coordinate.
 *
 * @param board A game board.
 * @param coordinate The coordinate whose neighbors will be searched.
 * @return An array of coordinate surrounding the given coordinate that are not occupied.
 */
export function getUnoccupiedNeighbors(
  board: GameBoard,
  coordinate: HexCoordinate
): HexCoordinate[] {
  const coordinates: HexCoordinate[] = [];
  eachNeighboringSpace(board, coordinate, (neighbor, stack) => {
    if (stack === undefined) coordinates.push(neighbor);
  });
  return coordinates;
}

/**
 * Determine if a game board is empty.
 *
 * @param board A game board.
 * @return true if there are no tiles on the board, false otherwise.
 */
export function isBoardEmpty(board: GameBoard): boolean {
  return isEmpty(board);
}

/**
 * Determine if there is any tile at the given coordinate.
 *
 * @param board The game board.
 * @param coordinate The coordinate to check.
 * @return true if there is at least one tile at the given coordinate, false otherwise.
 */
export function isCoordinateOccupied(
  board: GameBoard,
  coordinate: HexCoordinate
): boolean {
  const { q, r } = coordinate;
  const rs = board[q];
  return rs !== undefined && rs[r] !== undefined;
}

/**
 * Determine if a coordinate is touching the hive.
 *
 * @param board A game board.
 * @param coordinate The coordinate to test.
 * @return true if the coordinate is part of the hive or adjacent to a coordinate that is part of the hive, false otherwise.
 */
export function isCoordinateTouchingHive(
  board: GameBoard,
  coordinate: HexCoordinate
): boolean {
  // If the coordinate is occupied, we consider it to be touching the hive
  if (isCoordinateOccupied(board, coordinate)) return true;

  // Determine if there is some neighboring space where there is a stack of tiles.
  return someNeighboringSpace(board, coordinate, (_, stack) => {
    return stack !== undefined;
  });
}

/**
 * Determine if there is a gate blocking movement (sliding or climbing) from the
 * given coordinate in the given direction.
 *
 * @param board A game board.
 * @param coordinate The source coordinate.
 * @param direction The direction of movement.
 */
export function isGated(
  board: GameBoard,
  coordinate: HexCoordinate,
  direction: number
): boolean {
  // Get the direction of the two neighboring coordinates
  const ldir = toHexDirection(direction - 1);
  const rdir = toHexDirection(direction + 1);

  // Get the destination and two neighboring coordinates.
  const dest = relativeHexCoordinate(coordinate, direction);
  const left = relativeHexCoordinate(coordinate, ldir);
  const rght = relativeHexCoordinate(coordinate, rdir);

  // Get the stack heights for the four coordinates in question
  const srcHeight = getStackHeight(board, coordinate);
  const destHeight = getStackHeight(board, dest);
  const leftHeight = getStackHeight(board, left);
  const rghtHeight = getStackHeight(board, rght);

  return (
    srcHeight <= leftHeight &&
    srcHeight <= rghtHeight &&
    destHeight < leftHeight &&
    destHeight < rghtHeight
  );
}

/**
 * Determine if the queen of the given color is on the board.
 *
 * @param board A game board.
 * @param color The color queen to look for.
 * @return true if the queen of the given color is on the board, false otherwise.
 */
export function isQueenPlaced(board: GameBoard, color: ColorKey): boolean {
  return findTileCoordinate(board, `${color}Q`) !== null;
}

/**
 * Determine if the queen of the given color is completely surrounded.
 *
 * @param board A game board.
 * @param color The color queen to look for.
 * @return true if all six spaces surrounding the queen of the given color are occupied, false otherwise.
 */
export function isQueenSurrounded(board: GameBoard, color: ColorKey): boolean {
  const queen = findTileCoordinate(board, `${color}Q`);
  if (!queen) return false;
  return everyNeighbor(board, queen, (_, stack) => stack !== undefined);
}

/**
 * Determine if there is at least one tile placed at the given coordinate.
 *
 * @param board A game board.
 * @param coordinate A hex coordinate.
 * @return true if there is at least one tile at the given coordinate, false otherwise.
 */
export function isSpaceOccupied(
  board: GameBoard,
  coordinate: HexCoordinate
): boolean {
  return getStack(board, coordinate) !== undefined;
}

/**
 * Determine if a specific tile is critical to the structure of the hive,
 * meaning that moving the tile from its current location would break the hive.
 *
 * @param board The game board.
 * @param tileId The tile to test.
 * @param coordinate The hex coordinate where the tile is located.
 * @return true if moving the tile would break the hive, false otherwise.
 */
export function isTileStructural(
  board: GameBoard,
  tileId: TileId,
  coordinate: HexCoordinate
): boolean {
  // Moving a tile on top of a stack will never break the hive.
  if (getStackHeight(board, coordinate) > 1) return false;

  // Remove the tile from the board
  board = removeTile(board, tileId, coordinate);

  // If the board is now empty, it's not going to break the hive (or even be
  // possible) to move the tile.
  if (isEmpty(board)) return false;

  // Pick a random occupied coordinate and walk the board
  const coordinates = getOccupiedCoordinates(board);
  const visited = walkBoard(board, coordinates[0]);

  // If the hive is broken, we won't have visited every coordinate.
  return visited.length !== coordinates.length;
}

/**
 * Move a tile that is on top of a stack on the board to the top of a new stack.
 *
 * @param board A game board
 * @param tileId The tile id to move to a new hex coordinate
 * @param from The hex coordinate where the tile is currently on top of the stack
 * @param to The hex coordinate where the tile will be moved
 */
export function moveTile(
  board: GameBoard,
  tileId: TileId,
  from: HexCoordinate,
  to: HexCoordinate
): GameBoard {
  return produce(board, (draft) => _moveTile(draft, tileId, from, to));
}

/**
 * Place a tile on top of the stack located at the given hex coordinate.
 *
 * @param board A game board.
 * @param tileId The tile id to place at the given hex coordinate
 * @param coordinate The hex coordinate where the tile will be placed
 * @return A new game board with the added tile
 */
export function placeTile(
  board: GameBoard,
  tileId: TileId,
  coordinate: HexCoordinate
): GameBoard {
  return produce(board, (draft) => _placeTile(draft, tileId, coordinate));
}

/**
 * Remove a tile from the top of a stack located at a specific coordinate on the
 * board. If the given tile id is not the one on top of the stack at the given
 * coordinate, an error is thrown.
 *
 * @param board A game board.
 * @param tileId The tile id to remove from the top of the stack at the given hex coordinate.
 * @param coordinate The hex coordinate where the given tile is located.
 */
export function removeTile(
  board: GameBoard,
  tileId: TileId,
  coordinate: HexCoordinate
): GameBoard {
  return produce(board, (draft) => _popTile(draft, tileId, coordinate));
}

/**
 * Determine if there is some stack on the board for which the given predicate
 * holds true.
 *
 * @param board A game board.
 * @param iteratee The predicate function called for each stack.
 * @return true if the predicate evaluates to true for any stack, false otherwise.
 */
export function someStack(board: GameBoard, iteratee: StackFn): boolean {
  return !eachStack(board, (coordinate, stack) => {
    return !iteratee(coordinate, stack);
  });
}

/**
 * Determine if there is some space neighboring the given space for which the
 * given predicate holds true.
 *
 * @param board A game board.
 * @param coordinate The coordinate whose neighbors will be tested.
 * @param iteratee The predicate function called for neighbors.
 * @return true if the predicate evaluates to true for any neighbor, false otherwise.
 */
export function someNeighboringSpace(
  board: GameBoard,
  coordinate: HexCoordinate,
  iteratee: SpaceFn
): boolean {
  return !eachNeighboringSpace(board, coordinate, (neighbor, stack) => {
    return !iteratee(neighbor, stack);
  });
}

/**
 * Visit every stack on the board, beginning at the provided start coordinate
 * and recursively visiting neighbors. Each stack will be visited exactly once
 * assuming that the hive is not broken.
 *
 * @param board The game board.
 * @param start The starting coordinate.
 * @param iteratee The function invoked at each coordinate.
 * @return An array of hex coordinates in the order that they were visited.
 */
export function walkBoard(
  board: GameBoard,
  start: HexCoordinate,
  iteratee?: StackFn
): HexCoordinate[] {
  const visited = new Set<string>();
  const path: HexCoordinate[] = [];
  const visit = (coordinate: HexCoordinate, stack: TileId[]) => {
    visited.add(hexCoordinateKey(coordinate));
    path.push(coordinate);
    if (iteratee) iteratee(coordinate, stack);
    eachNeighboringStack(board, coordinate, (neighbor, stack) => {
      if (!visited.has(hexCoordinateKey(neighbor))) {
        visit(neighbor, stack);
      }
    });
  };
  const startStack = getStack(board, start);
  if (startStack) {
    visit(start, startStack);
  }
  return path;
}

/**
 * A producer that modifies a draft game by popping a tile from the top of a
 * stack and placing that same tile on top of another stack.
 *
 * @param draft An immer draft of a game board.
 * @param tileId The tile id to move to a new hex coordinate.
 * @param from The hex coordinate where the tile is currently on top of the stack.
 * @param to The hex coordinate where the tile will be moved.
 */
export function _moveTile(
  draft: GameBoard,
  tileId: TileId,
  from: HexCoordinate,
  to: HexCoordinate
) {
  _popTile(draft, tileId, from);
  _placeTile(draft, tileId, to);
}

/**
 * A producer that modifies a draft game board by placing a tile on top of the
 * stack located at the given hex coordinate.
 *
 * @param draft An immer draft of a game board.
 * @param tileId The tile id to place at the given hex coordinate.
 * @param coordinate The hex coordinate where the tile will be place.
 */
export function _placeTile(
  draft: GameBoard,
  tileId: TileId,
  coordinate: HexCoordinate
) {
  const { q, r } = coordinate;
  if (!(q in draft)) draft[q] = {};
  if (!(r in draft[q])) draft[q][r] = [];
  draft[q][r].push(tileId);
}

/**
 * A producer that modifies a draft game board by removing a specific tile from
 * the top of the stack at a given hex coordinate. If the given tile id is not
 * the one on top of the stack at the given coordinate, an error is thrown.
 * Empty references are deleted after the tile has been removed.
 *
 * @param draft An immer draft of a game board.
 * @param tileId The tile id to remove from top of the stack at the given hex coordinate.
 * @param coordinate The hex coordinate where the given tile is located.
 */
export function _popTile(
  draft: GameBoard,
  tileId: TileId,
  coordinate: HexCoordinate
) {
  const { q, r } = coordinate;
  const stack = draft[q][r];
  const tile = stack.pop();
  if (tile && tile !== tileId) {
    stack.push(tile);
    throw tileNotOnTopError(tileId, coordinate);
  }
  if (stack.length === 0) delete draft[q][r];
  if (isEmpty(draft[q])) delete draft[q];
}
