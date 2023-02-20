import { buildBoard } from './board';
import end0 from './data/end0';
import { buildMove } from './move';
import {
  _parseDirectionStrings,
  _parseGameNotation,
  _parseMoveNotation,
  _parseReferenceNotation,
  _parseTurnNotation,
  buildGameNotation,
  buildMoveNotation,
  getGameMoves,
  getGameTurns
} from './notation';

import game0 from './data/game0';

describe('notation parsing', () => {
  describe('_parseDirectionStrings', () => {
    test('no direction strings', () =>
      expect(_parseDirectionStrings()).toBe(0));
    test('direction = 1', () =>
      expect(_parseDirectionStrings(undefined, '/')).toBe(1));
    test('direction = 2', () =>
      expect(_parseDirectionStrings(undefined, '-')).toBe(2));
    test('direction = 3', () =>
      expect(_parseDirectionStrings(undefined, '\\')).toBe(3));
    test('direction = 4', () => expect(_parseDirectionStrings('/')).toBe(4));
    test('direction = 5', () => expect(_parseDirectionStrings('-')).toBe(5));
    test('direction = 6', () => expect(_parseDirectionStrings('\\')).toBe(6));
    test('invalid direction strings', () =>
      expect(_parseDirectionStrings('fdsa', 'fdsa')).toBe(0));
  });

  describe('_parseReferenceNotation', () => {
    test('white ant 1, direction 1', () =>
      expect(_parseReferenceNotation('wA1/')).toEqual({
        refId: 'wA1',
        dir: 1
      }));
    test('black beetle 1, direction 4', () =>
      expect(_parseReferenceNotation('/bB1')).toEqual({
        refId: 'bB1',
        dir: 4
      }));
    test('white queen ends game', () =>
      expect(_parseReferenceNotation('-bQ#')).toEqual({
        refId: 'bQ',
        dir: 5,
        end: true
      }));
    test('missing color', () =>
      expect(() => _parseReferenceNotation('A1/')).toThrow());
    test('unknown bug type', () =>
      expect(() => _parseReferenceNotation('wX/')).toThrow());
  });

  describe('_parseMoveNotation', () => {
    test('undefined move', () => expect(_parseMoveNotation()).toBeUndefined());
    test('passing move', () =>
      expect(_parseMoveNotation('x')).toEqual({
        notation: 'x',
        tileId: 'x',
        refId: 'x',
        dir: -1
      }));
    test('first move of the game', () =>
      expect(_parseMoveNotation('wL')).toEqual({
        notation: 'wL',
        tileId: 'wL',
        refId: 'wL',
        dir: 0
      }));
    test('white ant 1 to right of black queen', () =>
      expect(_parseMoveNotation('wA1 bQ-')).toEqual({
        notation: 'wA1 bQ-',
        tileId: 'wA1',
        refId: 'bQ',
        dir: 2
      }));
    test('white grasshopper 2 to bottom left of black mosquito', () =>
      expect(_parseMoveNotation('wG2 /wM')).toEqual({
        notation: 'wG2 /wM',
        tileId: 'wG2',
        refId: 'wM',
        dir: 4
      }));
  });

  describe('_parseTurnNotation', () => {
    test('turn 3: white pass, black has not moved', () =>
      expect(_parseTurnNotation('3. x')).toEqual({
        notation: '3. x',
        index: 3,
        white: {
          notation: 'x',
          tileId: 'x',
          refId: 'x',
          dir: -1
        }
      }));
    test('turn 150: white ant 2 to top right of black queen, game end', () =>
      expect(_parseTurnNotation('150. wA2 bQ/#')).toEqual({
        notation: '150. wA2 bQ/#',
        index: 150,
        white: {
          notation: 'wA2 bQ/#',
          tileId: 'wA2',
          refId: 'bQ',
          dir: 1,
          end: true
        }
      }));
    test('turn 10: white mosquito on top of black beetle 2, black pass', () =>
      expect(_parseTurnNotation('10. wM bB2, x')).toEqual({
        notation: '10. wM bB2, x',
        index: 10,
        white: {
          notation: 'wM bB2',
          tileId: 'wM',
          refId: 'bB2',
          dir: 0
        },
        black: {
          notation: 'x',
          tileId: 'x',
          refId: 'x',
          dir: -1
        }
      }));
    test('turn 73: white grasshopper 1 to bottom right of black queen, black ladybug to left of white pillbug', () =>
      expect(_parseTurnNotation('73. wG1 bQ\\, bL -wP')).toEqual({
        notation: '73. wG1 bQ\\, bL -wP',
        index: 73,
        white: {
          notation: 'wG1 bQ\\',
          tileId: 'wG1',
          refId: 'bQ',
          dir: 3
        },
        black: {
          notation: 'bL -wP',
          tileId: 'bL',
          refId: 'wP',
          dir: 5
        }
      }));
  });

  describe('_parseGameNotation', () => {
    test('a game with two completed turns (game0)', () =>
      expect(_parseGameNotation(game0.notation)).toEqual(game0.turns));
  });

  describe('getGameTurns', () => {
    test('a game with two completed turns (game0)', () =>
      expect(getGameTurns(game0.notation)).toEqual(game0.turns));
  });

  describe('getGameMoves', () => {
    test('a game with two completed turns (game0)', () =>
      expect(getGameMoves(game0.notation)).toEqual(game0.moves));
  });

  describe('buildGameNotation', () => {
    test('a game with two completed turns (game0)', () =>
      expect(buildGameNotation(game0.moves)).toBe(game0.notation));
    test('a completed game', () => {
      const board = buildBoard(end0.moves);
      const lastMove = buildMove(
        board,
        end0.endingMoveTile,
        end0.endingMoveCoordinate
      );
      const notation = buildGameNotation([...end0.moves, lastMove]);
      expect(notation).toEqual(end0.endingNotation);
    });
  });

  describe('buildMoveNotation', () => {
    test('game 0, move 0', () => {
      const move0 = game0.moves[0];
      const notation = buildMoveNotation(move0.tileId);
      expect(notation).toBe(move0.notation);
    });
    test('game 0, move 1', () => {
      const move1 = game0.moves[1];
      const notation = buildMoveNotation(
        move1.tileId,
        move1.refId,
        move1.dir,
        move1.end
      );
      expect(notation).toBe(move1.notation);
    });
    test('game 0, move 2', () => {
      const move2 = game0.moves[2];
      const notation = buildMoveNotation(
        move2.tileId,
        move2.refId,
        move2.dir,
        move2.end
      );
      expect(notation).toBe(move2.notation);
    });
    test('game 0, move 3', () => {
      const move3 = game0.moves[3];
      const notation = buildMoveNotation(
        move3.tileId,
        move3.refId,
        move3.dir,
        move3.end
      );
      expect(notation).toBe(move3.notation);
    });
  });
});
