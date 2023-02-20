import { buildBoard } from './board';
import board0 from './data/board0';
import board1 from './data/board1';
import game0 from './data/game0';
import end0 from './data/end0';
import * as move0 from './data/move/move0';
import move1 from './data/move/move1';
import {
  buildMove,
  canMove,
  getValidCoordinates,
  getValidPlacementCoordinates,
  isMovePass,
  moveBreaksHive
} from './move';
import { GameBoard, GameOptions, Move } from './types';

describe('moves', () => {
  describe('canMove', () => {
    test('white can and must place queen', () => {
      const whiteCanMove = canMove(move0.board, 'w', move0.options);
      expect(whiteCanMove).toBe(true);
    });
  });
  describe('getValidPlacementCoordinates', () => {
    const options: GameOptions = {
      ladybug: true,
      mosquito: true,
      pillbug: true,
      tournament: false
    };
    test('empty board', () => {
      const valid = getValidPlacementCoordinates({}, 'w', 'wA1', options);
      expect(valid).toHaveLength(1);
      expect(valid).toEqual(expect.arrayContaining([{ q: 0, r: 0 }]));
    });
    test('first move with tournament opening cannot be queen', () => {
      const valid = getValidPlacementCoordinates({}, 'w', 'wQ', {
        ladybug: true,
        mosquito: true,
        pillbug: true,
        tournament: true
      });
      expect(valid).toHaveLength(0);
    });
    test('board with single tile', () => {
      const valid = getValidPlacementCoordinates(
        { 0: { 0: ['wA1'] } },
        'b',
        'bA1',
        options
      );
      expect(valid).toHaveLength(6);
      expect(valid).toEqual(
        expect.arrayContaining([
          { q: -1, r: 0 },
          { q: 0, r: -1 },
          { q: 1, r: -1 },
          { q: 1, r: 0 },
          { q: 0, r: 1 },
          { q: -1, r: 1 }
        ])
      );
    });
    test('board with multiple tiles', () => {
      const validWhite = getValidPlacementCoordinates(
        board0,
        'w',
        'wA3',
        options
      );
      expect(validWhite).toHaveLength(4);
      expect(validWhite).toEqual(
        expect.arrayContaining([
          { q: -2, r: 1 },
          { q: -2, r: 2 },
          { q: -1, r: 2 },
          { q: 0, r: 1 }
        ])
      );
      const validBlack = getValidPlacementCoordinates(
        board0,
        'b',
        'bA3',
        options
      );
      expect(validBlack).toHaveLength(5);
      expect(validBlack).toEqual(
        expect.arrayContaining([
          { q: -1, r: -1 },
          { q: 0, r: -2 },
          { q: 1, r: -2 },
          { q: 2, r: -2 },
          { q: 2, r: -1 }
        ])
      );
    });
    test('must place queen', () => {
      const nonQueenWhite = getValidPlacementCoordinates(
        board1,
        'w',
        'wL',
        options
      );
      const nonQueenBlack = getValidPlacementCoordinates(
        board1,
        'b',
        'bL',
        options
      );
      expect(nonQueenWhite).toHaveLength(0);
      expect(nonQueenBlack).toHaveLength(0);
      const queenWhite = getValidPlacementCoordinates(
        board1,
        'w',
        'wQ',
        options
      );
      const queenBlack = getValidPlacementCoordinates(
        board1,
        'b',
        'bQ',
        options
      );
      expect(queenWhite).toHaveLength(5);
      expect(queenWhite).toEqual(
        expect.arrayContaining([
          { q: -2, r: 1 },
          { q: -2, r: 2 },
          { q: -1, r: 2 },
          { q: 0, r: 2 },
          { q: 1, r: 1 }
        ])
      );
      expect(queenBlack).toHaveLength(6);
      expect(queenBlack).toEqual(
        expect.arrayContaining([
          { q: -1, r: -1 },
          { q: 0, r: -2 },
          { q: 1, r: -3 },
          { q: 2, r: -3 },
          { q: 2, r: -2 },
          { q: 2, r: -1 }
        ])
      );
    });
  });
  describe('isMovePass', () => {
    test('a passing move', () => {
      expect(
        isMovePass({
          notation: 'x',
          tileId: 'x',
          refId: 'x',
          dir: -1
        })
      ).toBe(true);
    });
    describe('non-passing moves', () => {
      const moves: [number, Move][] = game0.moves.map((move, index) => [
        index,
        move
      ]);
      test.each(moves)('move %d', (_, move) => {
        expect(isMovePass(move)).toBe(false);
      });
    });
  });
  describe('moveBreaksHive', () => {
    const board: GameBoard = {
      '0': {
        '-1': ['bL', 'wS1']
      },
      '1': {
        '-1': ['wM']
      },
      '2': {
        '-1': ['bA1']
      },
      '-1': {
        '0': ['bQ']
      }
    };
    test('move stacked tile', () => {
      expect(moveBreaksHive(board, 'wS1', { q: 0, r: -1 })).toBe(false);
    });
    test('move critical tile', () => {
      expect(moveBreaksHive(board, 'wM', { q: 1, r: -1 })).toBe(true);
    });
    test('move non-critical tile', () => {
      expect(moveBreaksHive(board, 'bA1', { q: 2, r: -1 })).toBe(false);
    });
    test('single tile board', () => {
      expect(moveBreaksHive({ 0: { 0: ['wA1'] } }, 'wA1', { q: 0, r: 0 })).toBe(
        false
      );
    });
  });
  describe('buildMove', () => {
    test('move ends game', () => {
      const board = buildBoard(end0.moves);
      const move = buildMove(
        board,
        end0.endingMoveTile,
        end0.endingMoveCoordinate
      );
      expect(move.end).toBe(true);
      expect(move.notation).toEqual('wS2 /wB1#');
    });
  });
  describe('available move scenarios', () => {
    const scenarios = [{ index: 1, ...move1 }];
    test.each(scenarios)(
      'available moves $index',
      ({ mover, options, tileId, lastId, board, expectedCoordinates }) => {
        const validCoordinates = getValidCoordinates(
          board,
          mover,
          tileId,
          options,
          lastId
        );
        expect(validCoordinates).toHaveLength(expectedCoordinates.length);
        expect(validCoordinates).toEqual(
          expect.arrayContaining(expectedCoordinates)
        );
      }
    );
  });
});
