import { BugKey, ColorKey, TileId } from './types';

/**
 * Comparator that can be used by Array.sort() to sort a tile stack or array of
 * tiles in order from highest to lowest number.
 *
 * @param a A tile id
 * @param b A tile id
 * @return The difference in tile number between b and a (ie. b-a). Tiles without a number are assigned 0 for the calculation.
 */
export function compareTileNumber(a: TileId, b: TileId): number {
  const anum = getBugNumber(getTileBug(a)) || 0;
  const bnum = getBugNumber(getTileBug(b)) || 0;
  return bnum - anum;
}

/**
 * Get the letter from a bug key.
 *
 * @param bug The bug key.
 * @return The letter portion of the bug key.
 */
export function getBugLetter(bug: BugKey): string {
  return bug[0];
}

/**
 * Get the number from a bug key.
 *
 * @param bug The bug key.
 * @return The number portion of the bug key if it exists, otherwise undefined.
 */
export function getBugNumber(bug: BugKey): number | undefined {
  return bug[1] ? parseInt(bug[1]) : undefined;
}

/**
 * Get the bug key from a tile id.
 *
 * @param tileId The tile id.
 * @return The bug key portion of the tile id.
 */
export function getTileBug(tileId: TileId): BugKey {
  return tileId.slice(1) as BugKey;
}

/**
 * Get the color of a tile.
 *
 * @param tileId The tile's tile id
 * @return The tile's color
 */
export function getTileColor(tileId: TileId): ColorKey {
  return tileId[0] as ColorKey;
}

/**
 * Get the tile on top of a tile stack.
 *
 * @param stack A tile stack.
 * @return The id of the tile on top of the given stack.
 */
export function getTopTile(stack: TileId[]): TileId {
  return stack[stack.length - 1];
}

/**
 * Determine if a tile belongs to a player.
 *
 * @param tileId The tile id in question.
 * @param player The player in question.
 * @return true if the tile id and player are the same color, false otherwise.
 */
export function isOwnTile(tileId: TileId, player: ColorKey): boolean {
  return getTileColor(tileId) === player;
}
