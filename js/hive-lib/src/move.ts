import { uniqWith } from 'lodash';
import { getValidAntMoveCoordinates } from './ant';
import { getValidBeetleMoveCoordinates } from './beetle';
import {
  eachUnoccupiedCoordinate,
  everyNeighbor,
  findNeighborCoordinate,
  findTileCoordinate,
  getNumTiles,
  getOccupiedCoordinates,
  getStackHeight,
  getTile,
  getTiles,
  isBoardEmpty,
  isQueenPlaced,
  isQueenSurrounded,
  moveTile,
  placeTile,
  removeTile,
  someStack,
  walkBoard
} from './board';
import { noReferenceTileError } from './error';
import { getValidGrasshopperMoveCoordinates } from './grasshopper';
import { getStacksInHand } from './hand';
import { hexesEqual, relativeHexCoordinate, relativeHexDirection } from './hex';
import { getValidLadybugMoveCoordinates } from './ladybug';
import {
  getValidMosquitoMoveCoordinates,
  isMosquito,
  mosquitoIsBug
} from './mosquito';
import { buildMoveNotation } from './notation';
import {
  getValidPillbugMoveCoordinates,
  getValidPillbugPushCoordinates,
  isPillbug
} from './pillbug';
import { getValidQueenMoveCoordinates, isQueen } from './queen';
import { getValidSpiderMoveCoordinates } from './spider';
import { getTileBug, getTileColor, getTopTile } from './tile';
import {
  ColorKey,
  GameBoard,
  GameOptions,
  HexCoordinate,
  Move,
  MovePass,
  MovePlay,
  TileId
} from './types';

/**
 * Build the first move object of a game.
 *
 * @param tileId The id of the tile being placed first.
 * @return a Move object which describes the move.
 */
export function buildFirstMove(tileId: TileId): MovePlay {
  return {
    notation: buildMoveNotation(tileId),
    tileId: tileId,
    refId: tileId,
    dir: 0
  };
}

/**
 * Build a non-passing move object.
 *
 * @param board A game board
 * @param tileId The id of the tile being moved or placed.
 * @param destination The coordinate on the given board where the tile will end up.
 * @return a Move object which describes the move.
 */
export function buildMove(
  board: GameBoard,
  tileId: TileId,
  destination: HexCoordinate
): MovePlay {
  // If there are no tiles on the board, it's the first move of the game.
  if (isBoardEmpty(board)) return buildFirstMove(tileId);

  // Find a neighboring tile to use as reference.
  const neighbor = findNeighborCoordinate(
    board,
    destination,
    (_, stack) => stack !== undefined
  );

  // Get the reference tile id and relative direction
  if (!neighbor) throw noReferenceTileError(destination);
  const refId = getTile(board, neighbor);
  if (!refId) throw noReferenceTileError(destination);
  const dir = relativeHexDirection(neighbor, destination);

  // Perform the move so we can determine if it ends the game
  const coordinate = findTileCoordinate(board, tileId);
  board = coordinate
    ? moveTile(board, tileId, coordinate, destination)
    : placeTile(board, tileId, destination);

  const end = isQueenSurrounded(board, 'b') || isQueenSurrounded(board, 'w');

  return {
    tileId,
    refId,
    dir,
    end,
    notation: buildMoveNotation(tileId, refId, dir, end)
  };
}

/**
 * Build a passing move object.
 *
 * @return a Move object which describes a passing move.
 */
export function buildPassMove(): MovePass {
  return {
    notation: 'x',
    tileId: 'x',
    refId: 'x',
    dir: -1
  };
}

/**
 * Determine if there are any possible moves for the player of the given color.
 *
 * @param board A game board.
 * @param player A player color.
 * @param options A game options object.
 * @return true if the player can make a valid move, false otherwise.
 */
export function canMove(
  board: GameBoard,
  player: ColorKey,
  options: GameOptions
): boolean {
  // Each player can always play their first tile.
  if (isBoardEmpty(board) && player === 'w') return true;
  if (getTiles(board).length === 1 && player === 'b') return true;

  // Determine if any tiles in hand can be placed.
  const hand = getStacksInHand(board, player, options);
  if (hand.length) {
    return hand.some((handStack) => {
      return handStack.some(
        (tile) =>
          getValidPlacementCoordinates(board, player, tile, options).length > 0
      );
    });
  }

  // Determine if some tile on the board can be moved.
  return someStack(board, (coordinate, stack) => {
    const moves = getValidMoveCoordinates(
      board,
      player,
      stack[stack.length - 1],
      coordinate
    );
    return moves.length > 0;
  });
}

/**
 * Get all coordinates that are valid moves for the given tile at the given
 * coordinate acting as its own bug type.
 *
 * @param board A game board.
 * @param tileId The id of the tile being moved.
 * @param coordinate The hex coordinate where the given tile is located.
 * @return An array of hex coordinates.
 */
export function getValidBugMoveCoordinates(
  board: GameBoard,
  tileId: TileId,
  coordinate: HexCoordinate
): HexCoordinate[] {
  const bug = getTileBug(tileId);
  switch (bug) {
    case 'A1':
    case 'A2':
    case 'A3':
      return getValidAntMoveCoordinates(board, coordinate);
    case 'B1':
    case 'B2':
      return getValidBeetleMoveCoordinates(board, coordinate);
    case 'G1':
    case 'G2':
    case 'G3':
      return getValidGrasshopperMoveCoordinates(board, coordinate);
    case 'S1':
    case 'S2':
      return getValidSpiderMoveCoordinates(board, tileId, coordinate);
    case 'L':
      return getValidLadybugMoveCoordinates(board, tileId, coordinate);
    case 'M':
      return getValidMosquitoMoveCoordinates(board, tileId, coordinate);
    case 'P':
      return getValidPillbugMoveCoordinates(board, coordinate);
    case 'Q':
      return getValidQueenMoveCoordinates(board, coordinate);
  }
}

/**
 * Get all coordinates that are valid moves or placements for the given tile
 * played by the given player color.
 *
 * @param board A game board.
 * @param player The player performing the action.
 * @param tileId The id of the tile being played.
 * @param options A game options object.
 * @param lastId The id of the last tile that was played (if there is one).
 * @return An array of hex coordinates.
 */
export function getValidCoordinates(
  board: GameBoard,
  player: ColorKey,
  tileId: TileId,
  options: GameOptions,
  lastId: TileId | null
): HexCoordinate[] {
  // If the last tile id is not null, check if it is the same tile we're trying
  // to move right now. If it is, then it cannot be moved. This covers two
  // special cases for the pillbug: (1) the other player just moved the tile
  // we're considering using their pillbug; and (2) the other player just moved
  // the tile we're considering next to a pillbug and therefore the pillbug
  // cannot move that tile.
  if (tileId === lastId) return [];

  // Determine if the tile is on the board.
  const coordinate = findTileCoordinate(board, tileId);

  // If it is, return move coordinates, if it's not return placement coordinates.
  return coordinate
    ? getValidMoveCoordinates(board, player, tileId, coordinate)
    : getValidPlacementCoordinates(board, player, tileId, options);
}

/**
 * Get all coordinates that are valid moves for the given tile moved by the
 * given player color.
 *
 * @param board A game board.
 * @param player The player performing the move.
 * @param tileId The id of the tile being moved.
 * @param coordinate The hex coordinate where the tile to be moved is located.
 * @return An array of hex coordinates.
 */
export function getValidMoveCoordinates(
  board: GameBoard,
  player: ColorKey,
  tileId: TileId,
  coordinate: HexCoordinate
): HexCoordinate[] {
  // First check that the player's queen has been placed. If it hasn't they
  // aren't yet allowed to move tiles.
  if (!isQueenPlaced(board, player)) return [];

  // Check if moving the tile will break the hive. If it will, it cannot be
  // moved.
  if (moveBreaksHive(board, tileId, coordinate)) return [];

  // Get the tile color
  const color = getTileColor(tileId);

  // If a player is moving their own tile, get the moves allowed by the bug type.
  const bugMoves =
    player === color
      ? getValidBugMoveCoordinates(board, tileId, coordinate)
      : [];

  // Check for the player's pillbug in surrouding tiles and get valid pushes.
  const pillbugPushes = ownPillbugPushes(board, player, tileId, coordinate);

  // Check for the player's mosquito in surrounding tiles and get valid pushes
  // if it is able to act as a pillbug.
  const mosquitoPushes = ownMosquitoPushes(board, player, tileId, coordinate);

  // Filter out repeats and return all valid coordinates.
  return uniqWith(
    [...bugMoves, ...pillbugPushes, ...mosquitoPushes],
    hexesEqual
  );
}

/**
 * Get all coordinates that are valid placements (ie. from a player's hand onto
 * the board) for the given tile.
 *
 * @param board A game board.
 * @param player The color of the player placeing the tile.
 * @param tileId The tile to be placed on the game board.
 * @param options A game options object.
 * @return An array of hex coordinates.
 */
export function getValidPlacementCoordinates(
  board: GameBoard,
  player: ColorKey,
  tileId: TileId,
  options: GameOptions
): HexCoordinate[] {
  // Get the tile color
  const color = getTileColor(tileId);

  // A player can only place their own tiles
  if (color !== player) return [];

  // If the board is empty the origin is the only valid coordinate. If
  // tournament opening is used, the first tile cannot be a queen.
  if (isBoardEmpty(board)) {
    if (options.tournament && isQueen(tileId)) return [];
    return [{ q: 0, r: 0 }];
  }

  // If there's a single tile, it's at the origin and any open spot around it is valid.
  if (getNumTiles(board) === 1)
    return [1, 2, 3, 4, 5, 6].map((n) =>
      relativeHexCoordinate({ q: 0, r: 0 }, n)
    );

  // If the player has not placed their queen by the fourth move, they must do so.
  const playerColor = getTileColor(tileId);
  if (
    getNumTiles(board, playerColor) === 3 &&
    !isQueenPlaced(board, playerColor) &&
    !isQueen(tileId, playerColor)
  )
    return [];

  // A predicate that evaluates to true if a tile at a coordinate is the same
  // color as the player.
  const sameColor = (_: HexCoordinate, stack?: TileId[]) =>
    !stack || getTileColor(getTopTile(stack)) === playerColor;

  const valid: HexCoordinate[] = [];
  eachUnoccupiedCoordinate(board, (coordinate) => {
    if (everyNeighbor(board, coordinate, sameColor)) valid.push(coordinate);
  });
  return valid;
}

/**
 * Determine if a move is a passing move.
 *
 * @param move A move.
 * @return true if the move is a passing move, false otherwise.
 */
export function isMovePass(move: Move): move is MovePass {
  return move.tileId === 'x' && move.refId === 'x' && move.dir === -1;
}

/**
 * Determine if moving a tile would break the hive.
 *
 * @param board A game board.
 * @param tileId The tile to test.
 * @param coordinate The coordinate where the tile is located.
 * @return true if moving the tile would break the hive, false otherwise.
 */
export function moveBreaksHive(
  board: GameBoard,
  tileId: TileId,
  coordinate: HexCoordinate
): boolean {
  if (getStackHeight(board, coordinate) > 1) return false;

  board = removeTile(board, tileId, coordinate);
  const coordinates = getOccupiedCoordinates(board);
  if (coordinates.length <= 1) return false;

  const start = coordinates[0];
  const path = walkBoard(board, start);

  return path.length !== coordinates.length;
}

/**
 * Check for a player's mosquito acting as a pillbug in the tiles surrounding a
 * given coordinate. If it's there, return the coordinates where that mosquito
 * could push the tile located at the given coordinate.
 *
 * @param board A game board.
 * @param player The color of the player doing the pushing.
 * @param tileId The tile to be pushed.
 * @param coordinate The hex coordinate where the tile to be pushed is located.
 * @return An array of hex coordinates.
 */
export function ownMosquitoPushes(
  board: GameBoard,
  player: ColorKey,
  tileId: TileId,
  coordinate: HexCoordinate
): HexCoordinate[] {
  // Create a predicate that can be used to search the surrounding tiles for
  // the player's mosquito.
  const isOwnMosquito = (neighbor: HexCoordinate) => {
    const tile = getTile(board, neighbor);
    return tile && isMosquito(tile, player);
  };

  // Search the surrounding tiles for the player's mosquito.
  const ownMosquito = findNeighborCoordinate(board, coordinate, isOwnMosquito);

  // If the player's mosquito is adjacent, determine if it's able to act as a
  // pillbug. If so, return its valid pushes, otherwise return an empty array
  // since there are no valid mosquito-as-pillbug pushes.
  return ownMosquito && mosquitoIsBug(board, ownMosquito, 'P')
    ? getValidPillbugPushCoordinates(board, tileId, coordinate, ownMosquito)
    : [];
}

/**
 * Check for a player's pillbug in the tiles surrounding a given coordinate. If
 * it's there, return the coordinates where that pillbug could push the tile
 * located at the given coordinate.
 *
 * @param board A game board.
 * @param player The color of the player doing the pushing.
 * @param tileId The tile to be pushed.
 * @param coordinate The hex coordinate where the tile to be pushed is located.
 * @return An array of hex coordinates.
 */
export function ownPillbugPushes(
  board: GameBoard,
  player: ColorKey,
  tileId: TileId,
  coordinate: HexCoordinate
): HexCoordinate[] {
  // Create a predicate that can be used to search the surrounding tiles for
  // the player's pillbug.
  const isOwnPillbug = (neighbor: HexCoordinate) => {
    const tile = getTile(board, neighbor);
    return tile && isPillbug(tile, player);
  };

  // Now search the surrounding tiles for the player's pillbug.
  const ownPillbug = findNeighborCoordinate(board, coordinate, isOwnPillbug);

  // If the player's pillbug is adjacent, return its valid pushes, otherwise
  // return an empty array since there are no valid pillbug pushes.
  return ownPillbug
    ? getValidPillbugPushCoordinates(board, tileId, coordinate, ownPillbug)
    : [];
}
