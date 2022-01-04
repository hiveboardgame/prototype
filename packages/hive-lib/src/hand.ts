import { groupBy, values } from 'lodash';
import { getTiles } from './board';
import { isLadybug } from './ladybug';
import { isMosquito } from './mosquito';
import { isPillbug } from './pillbug';
import { compareTileNumber, getBugLetter, getTileBug } from './tile';
import {
  BLACK_TILES,
  ColorKey,
  GameBoard,
  GameOptions,
  TileId,
  WHITE_TILES
} from './types';

/**
 * Get a list of tile ids that a player has in their hand, grouped by bug type.
 *
 * @param board A game board.
 * @param color A player color.
 * @param options A game options object.
 * @return A list of tile ids in the given players hand, grouped by bug type.
 */
export function getStacksInHand(
  board: GameBoard,
  color: ColorKey,
  options: GameOptions
): TileId[][] {
  const hand = getTilesInHand(board, color, options);
  const groups = groupBy(hand, (tileId) => getBugLetter(getTileBug(tileId)));
  const stacks = values(groups);
  stacks.forEach((stack) => stack.sort(compareTileNumber));
  return stacks;
}

/**
 * Get a list of tile ids that a player has in their hand (ie. those that are
 * not on the board.
 *
 * @param board A game board.
 * @param color A player color.
 * @param options
 * @return A list of tile ids in the given player's hand.
 */
export function getTilesInHand(
  board: GameBoard,
  color: ColorKey,
  options: GameOptions
): TileId[] {
  if (color !== 'b' && color !== 'w') return [];
  const tilesOnBoard = new Set<string>(getTiles(board));
  const colorTiles = color === 'b' ? BLACK_TILES : WHITE_TILES;
  return colorTiles.filter((tileId: TileId) => {
    if (!options.ladybug && isLadybug(tileId)) return false;
    if (!options.mosquito && isMosquito(tileId)) return false;
    if (!options.pillbug && isPillbug(tileId)) return false;
    return !tilesOnBoard.has(tileId);
  });
}
