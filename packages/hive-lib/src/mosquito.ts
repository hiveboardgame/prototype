import { uniqWith } from 'lodash';
import { getValidAntMoveCoordinates } from './ant';
import { getValidBeetleMoveCoordinates } from './beetle';
import {
  eachNeighboringStack,
  getStackHeight,
  getTile,
  someNeighboringSpace
} from './board';
import { getValidGrasshopperMoveCoordinates } from './grasshopper';
import { hexesEqual } from './hex';
import { getValidLadybugMoveCoordinates } from './ladybug';
import { getValidPillbugMoveCoordinates } from './pillbug';
import { getValidQueenMoveCoordinates } from './queen';
import { getValidSpiderMoveCoordinates } from './spider';
import { getBugLetter, getTileBug, getTileColor, getTopTile } from './tile';
import { BugLetter, ColorKey, GameBoard, HexCoordinate, TileId } from './types';

/**
 * Get all coordinates that are valid moves for the given tile id at the given
 * coordinate acting as a mosquito. The mosquito rules state that a mosquito
 * takes on the movement characteristics of every other bug it is touching. If
 * it takes on the role of a beetle and is on top of the hive, it remains a
 * beetle until it drops back down. If it is touching only a mosquito, it cannot
 * move.
 *
 * @param board A game board.
 * @param tileId The id of the tile being moved.
 * @param coordinate The hex coordinate where the given tile is located.
 * @return An array of hex coordinates.
 */
export function getValidMosquitoMoveCoordinates(
  board: GameBoard,
  tileId: TileId,
  coordinate: HexCoordinate
): HexCoordinate[] {
  const isBeetle = getStackHeight(board, coordinate) > 1;
  if (isBeetle) return getValidBeetleMoveCoordinates(board, coordinate);

  const valid: HexCoordinate[] = [];
  eachNeighboringStack(board, coordinate, (_, stack) => {
    const top = getTopTile(stack);
    const bug = getTileBug(top);
    switch (bug) {
      case 'A1':
      case 'A2':
      case 'A3':
        valid.push(...getValidAntMoveCoordinates(board, coordinate));
        break;
      case 'B1':
      case 'B2':
        valid.push(...getValidBeetleMoveCoordinates(board, coordinate));
        break;
      case 'G1':
      case 'G2':
      case 'G3':
        valid.push(...getValidGrasshopperMoveCoordinates(board, coordinate));
        break;
      case 'L':
        valid.push(
          ...getValidLadybugMoveCoordinates(board, tileId, coordinate)
        );
        break;
      case 'P':
        valid.push(...getValidPillbugMoveCoordinates(board, coordinate));
        break;
      case 'Q':
        valid.push(...getValidQueenMoveCoordinates(board, coordinate));
        break;
      case 'S1':
      case 'S2':
        valid.push(...getValidSpiderMoveCoordinates(board, tileId, coordinate));
    }
  });
  return uniqWith(valid, hexesEqual);
}

/**
 * Determine if a tile is a mosquito, optionally of a specific color.
 *
 * @param tileId A tile id.
 * @param color A tile color.
 * @return true if the tile is a mosquito (of the specified color if provided), false otherwise.
 */
export function isMosquito(tileId: TileId, color?: ColorKey): boolean {
  return (
    getBugLetter(getTileBug(tileId)) === 'M' &&
    (color ? getTileColor(tileId) === color : true)
  );
}

/**
 * Determine if a mosquito located at the given coordinate is able to act as a
 * certain bug type.
 *
 * @param board A game board.
 * @param coordinate The coordinate where the mosquito is located.
 * @param bug A bug type.
 * @return true if the mosquito can act as the given bug type, false otherwise.
 */
export function mosquitoIsBug(
  board: GameBoard,
  coordinate: HexCoordinate,
  bug: BugLetter
): boolean {
  if (getStackHeight(board, coordinate) > 1) {
    return bug === 'B';
  }
  return someNeighboringSpace(board, coordinate, (neighbor) => {
    const tile = getTile(board, neighbor);
    return tile && getBugLetter(getTileBug(tile)) === bug;
  });
}
