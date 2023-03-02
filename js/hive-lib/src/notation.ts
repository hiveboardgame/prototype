import { chunk, flatten } from 'lodash';
import { Move, TileId, Turn } from './types';

/**
 * The regular expression used to parse reference tile notation, which includes
 * the reference tile and direction identifiers.
 */
const REGEX_REF =
  /(?<lDir>[\/\\-]?)(?<tileId>[wb][ABGLMPQS][123]?)(?<rDir>[\/\\-]?)(?<end>#?)/;

/**
 * Get a game notation string from an array of *Move*s.
 *
 * @param moves An array of game moves.
 * @return A game notation string.
 */
export function buildGameNotation(moves: Move[]): string {
  return chunk<Move>(moves, 2)
    .reduce((notation: string, current: Move[], index: number) => {
      return (
        notation + _buildTurnNotation(index + 1, current[0], current[1]) + ' '
      );
    }, '')
    .trim();
}

/**
 * Build a move notation string from the properties that define a *Move* object.
 * If only a tile id is provided, a string representing the first move of the
 * game is returned.
 *
 * @param tileId A tile id.
 * @param refId A reference tile id.
 * @param direction A hex direction.
 * @param end A boolean indicating if the move ends the game.
 */
export function buildMoveNotation(
  tileId: TileId,
  refId?: TileId,
  direction?: number,
  end?: boolean
): string {
  const e = end ? '#' : '';
  const r =
    refId === undefined ? '' : _buildReferenceNotation(refId, direction);
  return `${tileId} ${r}${e}`.trim();
}

/**
 * Get an ordered array of *Move* objects from a game notation string.
 *
 * @param notation A game notation string.
 * @return An array of *Move* objects.
 */
export function getGameMoves(notation: string): Move[] {
  const turns = _parseGameNotation(notation);
  return flatten(
    turns.map((turn) =>
      turn.white ? (turn.black ? [turn.white, turn.black] : [turn.white]) : []
    )
  );
}

/**
 * Get an ordered array of *Turn* objects from a game notation string.
 *
 * @param notation A game notation string.
 * @return An array of *Turn* objects.
 */
export function getGameTurns(notation: string): Turn[] {
  return _parseGameNotation(notation);
}

/**
 * Build a reference tile notation string from a reference tile id an hex
 * direction.
 *
 * @param refId A tile id.
 * @param direction A hex direction.
 * @return A reference tile notation string.
 */
export function _buildReferenceNotation(
  refId?: TileId,
  direction?: number
): string {
  if (refId === undefined || direction === undefined) return '';
  switch (direction) {
    case 0:
      return refId;
    case 1:
      return `${refId}/`;
    case 2:
      return `${refId}-`;
    case 3:
      return `${refId}\\`;
    case 4:
      return `/${refId}`;
    case 5:
      return `-${refId}`;
    case 6:
      return `\\${refId}`;
    default:
      return '';
  }
}

/**
 * Build a turn notation string from a white and (optionally) black move.
 *
 * @param turn The turn number.
 * @param white White's move for this turn.
 * @param black Black's move for this turn.
 */
export function _buildTurnNotation(
  turn: number,
  white: Move,
  black?: Move
): string {
  return `${turn}. ${white.notation}${black ? `, ${black.notation}` : ''}`;
}

/**
 * Create an ordered array of *Turn* objects by parsing a game notation string.
 *
 * @param notation A game notation string.
 * @return An array of *Turn* objects.
 */
export function _parseGameNotation(notation: string): Turn[] {
  return notation.split(/\s(?=\d+\.)/g).map(_parseTurnNotation);
}

/**
 * Create a *Turn* object by parsing a turn notation string.
 *
 * @param notation A turn notation string.
 * @return A *Turn* object.
 */
export function _parseTurnNotation(notation: string): Turn {
  const sepLocation = notation.indexOf('.');
  const indexString = notation.slice(0, sepLocation);
  const placementsString = notation.slice(sepLocation + 1);
  const placements = placementsString.split(',');
  return {
    notation,
    index: parseInt(indexString),
    white: _parseMoveNotation(placements[0]),
    black: _parseMoveNotation(placements[1])
  };
}

/**
 * Create a *Move* object by parsing a move notation string.
 *
 * @param notation A move notation string.
 * @return A *Move* object.
 */
export function _parseMoveNotation(notation?: string): Move | undefined {
  if (!notation) return undefined;

  // Split notation into moving tile and reference tile portions
  notation = notation.trim();
  const [tileId, refNotation] = notation.split(/\s/g);

  // Check for and return a passing move
  if (tileId === 'x')
    return {
      notation,
      tileId: 'x',
      refId: 'x',
      dir: -1
    };

  // Parse the reference notation to get the reference tile and direction
  const { refId, dir, end } =
    refNotation !== undefined
      ? _parseReferenceNotation(refNotation) // only when there is one
      : { refId: tileId, dir: 0, end: false }; // first move notation

  // Return a playing move
  return {
    notation,
    tileId: tileId as TileId,
    refId: refId as TileId,
    dir,
    ...(end ? { end: true } : {})
  };
}

/**
 * Extract the reference tile, direction, and optional boolean indicating game
 * end from a reference tile notation string.
 *
 * @param notation A reference tile notation string.
 * @return An object containing the *refId*, *direction*, and *end* fields for a *Move* object.
 */
export function _parseReferenceNotation(
  notation: string
): Pick<Move, 'refId' | 'dir' | 'end'> {
  const match = notation.match(REGEX_REF);
  const groups = match?.groups;
  if (!match || !groups)
    throw Error(`Invalid target notation string: ${notation}`);

  return {
    refId: groups.tileId as TileId,
    dir: _parseDirectionStrings(groups.lDir, groups.rDir),
    ...(groups.end ? { end: true } : {})
  };
}

/**
 * Parse a pair of direction strings. These strings appear to the left or right
 * of a reference tile notation string and indicate which hex direction relative
 * to the reference tile should be used to place a tile.
 *
 * @param lDir A direction string that appears to the left of the reference tile
 * @param rDir A direction string that appears to the right of the reference tile
 * @return A hex direction, or 0 if no direction strings or two invalid direction strings are supplied.
 */
export function _parseDirectionStrings(lDir?: string, rDir?: string): number {
  switch (rDir) {
    case '/':
      return 1;
    case '-':
      return 2;
    case '\\':
      return 3;
  }
  switch (lDir) {
    case '/':
      return 4;
    case '-':
      return 5;
    case '\\':
      return 6;
  }
  return 0;
}
