import {
  _moveTile,
  _placeTile,
  _popTile,
  buildBoard,
  eachDirection,
  eachUnoccupiedCoordinate,
  eachNeighboringSpace,
  eachNeighboringStack,
  eachSlideDirection,
  eachStack,
  everyNeighbor,
  findNeighborCoordinate,
  findTileCoordinate,
  getOccupiedCoordinates,
  getOccupiedNeighbors,
  getStack,
  getStackHeight,
  getStackHeightDifference,
  getStacks,
  getTile,
  getTiles,
  getUnoccupiedCoordinates,
  getUnoccupiedNeighbors,
  isBoardEmpty,
  isCoordinateOccupied,
  isCoordinateTouchingHive,
  isGated,
  isQueenPlaced,
  isQueenSurrounded,
  isTileStructural,
  moveTile,
  placeTile,
  removeTile,
  someNeighboringSpace,
  walkBoard
} from './board';
import board2 from './data/board2';
import board3 from './data/board3';
import game0 from './data/game0';
import game1 from './data/game1';
import gates from './data/gates';
import { hexesEqual } from './hex';
import { GameBoard, HexCoordinate, Move } from './types';

describe('game board data and properties', () => {
  describe('buildBoard', () => {
    test('game0, up to move 0', () => {
      expect(buildBoard(game0.moves, 0)).toEqual({});
    });
    test('game0, up to move 1', () => {
      expect(buildBoard(game0.moves, 1)).toEqual(game0.boards[0]);
    });
    test('game0, up to move 2', () => {
      expect(buildBoard(game0.moves, 2)).toEqual(game0.boards[1]);
    });
    test('game0, up to move 3', () => {
      expect(buildBoard(game0.moves, 3)).toEqual(game0.boards[2]);
    });
    test('game0, all moves', () => {
      expect(buildBoard(game0.moves)).toEqual(game0.boards[3]);
    });
    test('game 1, all moves', () => {
      expect(buildBoard(game1.moves)).toEqual(game1.boards[7]);
    });
    test('invalid reference tile', () => {
      const move0: Move = {
        notation: 'wQ',
        tileId: 'wQ',
        refId: 'wQ',
        dir: 0
      };
      const move1: Move = {
        notation: 'bS1 -wA1',
        tileId: 'bS1',
        refId: 'wA1',
        dir: 5
      };
      expect(() => buildBoard([move0, move1])).toThrow();
    });
  });
  describe('getOccupiedCoordinates', () => {
    test('gets all coordinates with tiles', () => {
      const board = game0.boards[3];
      const coordinates = getOccupiedCoordinates(board);
      expect(coordinates).toHaveLength(4);
      expect(coordinates).toEqual(
        expect.arrayContaining([
          { q: -1, r: -1 },
          { q: -1, r: 0 },
          { q: 0, r: 0 },
          { q: 1, r: 0 }
        ])
      );
    });
    test('returns empty array when board is empty', () => {
      expect(getOccupiedCoordinates({})).toHaveLength(0);
    });
  });
  describe('getOccupiedNeighbors', () => {
    test('gets white queen neighbors', () => {
      const board = game0.boards[3];
      const coordinates = getOccupiedNeighbors(board, { q: 0, r: 0 });
      expect(coordinates).toHaveLength(2);
      expect(coordinates).toEqual(
        expect.arrayContaining([
          { q: 1, r: 0 },
          { q: -1, r: 0 }
        ])
      );
    });
  });
  describe('getStack', () => {
    const board: GameBoard = { 0: { 0: ['wQ', 'bQ'] } };
    test('no stack', () =>
      expect(getStack({}, { q: 0, r: 0 })).toBeUndefined());
    test('single tile', () => {
      expect(getStack(game0.boards[0], { q: 0, r: 0 })).toEqual(['wQ']);
    });
    test('multiple tiles', () => {
      expect(getStack(board, { q: 0, r: 0 })).toEqual(['wQ', 'bQ']);
    });
    test('same first coordinate', () => {
      expect(getStack(board, { q: 0, r: -1 })).toBeUndefined();
    });
  });
  describe('getStacks', () => {
    test('get all stacks from sample board', () => {
      const board = game0.boards[3];
      const stacks = getStacks(board);
      expect(stacks).toEqual(
        expect.arrayContaining([
          {
            coordinate: { q: -1, r: -1 },
            stack: expect.arrayContaining(['bQ'])
          },
          {
            coordinate: { q: -1, r: 0 },
            stack: expect.arrayContaining(['bS1'])
          },
          { coordinate: { q: 0, r: 0 }, stack: expect.arrayContaining(['wQ']) },
          { coordinate: { q: 1, r: 0 }, stack: expect.arrayContaining(['wA1']) }
        ])
      );
    });
  });
  describe('getStackHeight', () => {
    test('no stack', () => expect(getStackHeight({}, { q: 0, r: 0 })).toBe(0));
    test('one tile', () =>
      expect(getStackHeight(game0.boards[0], { q: 0, r: 0 })).toBe(1));
    test('two tiles', () => {
      const board: GameBoard = { 0: { 0: ['wQ', 'bQ'] } };
      expect(getStackHeight(board, { q: 0, r: 0 })).toBe(2);
    });
  });
  describe('getStackHeightDifference', () => {
    test('no stack, no stack', () =>
      expect(getStackHeightDifference({}, { q: 0, r: 0 }, { q: 1, r: 0 })).toBe(
        0
      ));
    test('one tile, no stack', () => {
      expect(
        getStackHeightDifference(
          game0.boards[0],
          { q: 0, r: 0 },
          { q: 1, r: 0 }
        )
      ).toBe(1);
    });
    test('one tile, one tile', () => {
      expect(
        getStackHeightDifference(
          game0.boards[1],
          { q: 0, r: 0 },
          { q: -1, r: 0 }
        )
      ).toBe(0);
    });
  });
  describe('getTiles', () => {
    test('get tiles from empty board', () => {
      expect(getTiles({})).toHaveLength(0);
    });
    test('get tiles from game0 boards', () => {
      const tiles0 = getTiles(game0.boards[0]);
      const tiles1 = getTiles(game0.boards[1]);
      const tiles2 = getTiles(game0.boards[2]);
      const tiles3 = getTiles(game0.boards[3]);
      expect(tiles0).toHaveLength(1);
      expect(tiles1).toHaveLength(2);
      expect(tiles2).toHaveLength(3);
      expect(tiles3).toHaveLength(4);
      expect(tiles0).toEqual(expect.arrayContaining(['wQ']));
      expect(tiles1).toEqual(expect.arrayContaining(['wQ', 'bS1']));
      expect(tiles2).toEqual(expect.arrayContaining(['wQ', 'bS1', 'wA1']));
      expect(tiles3).toEqual(
        expect.arrayContaining(['wQ', 'bS1', 'wA1', 'bQ'])
      );
    });
  });
  describe('getTopTile', () => {
    test('no stack', () => expect(getTile({}, { q: 0, r: 0 })).toBeUndefined());
    test('one tile', () =>
      expect(getTile(game0.boards[0], { q: 0, r: 0 })).toBe('wQ'));
    test('two tiles', () => {
      const board: GameBoard = { 0: { 0: ['wQ', 'bQ'] } };
      expect(getTile(board, { q: 0, r: 0 })).toBe('bQ');
    });
  });
  describe('getUnoccupiedCoordinates', () => {
    test('empty board returns empty list', () =>
      expect(getUnoccupiedCoordinates({})).toEqual([]));
    test('returns unoccupied coordinates', () => {
      const unoccupied = getUnoccupiedCoordinates(game0.boards[3]);
      expect(unoccupied).toHaveLength(12);
      expect(unoccupied).toEqual(
        expect.arrayContaining([
          { q: -2, r: 0 },
          { q: -2, r: -1 },
          { q: -1, r: -2 },
          { q: 0, r: -2 },
          { q: 0, r: -1 },
          { q: 1, r: -1 },
          { q: 2, r: -1 },
          { q: 2, r: 0 },
          { q: 1, r: 1 },
          { q: 0, r: 1 },
          { q: -1, r: 1 },
          { q: -2, r: 1 }
        ])
      );
    });
  });
  describe('getUnoccupiedNeighbors', () => {
    test('empty board returns all neighbors', () => {
      const unoccupied = getUnoccupiedNeighbors({}, { q: 0, r: 0 });
      expect(unoccupied).toHaveLength(6);
      expect(unoccupied).toEqual(
        expect.arrayContaining([
          { q: 1, r: -1 },
          { q: 1, r: 0 },
          { q: 0, r: 1 },
          { q: -1, r: 1 },
          { q: -1, r: 0 },
          { q: 0, r: -1 }
        ])
      );
    });
    test('four coordinates returned when two neighbors occupied', () => {
      const unoccupied = getUnoccupiedNeighbors(game0.boards[3], {
        q: 0,
        r: 0
      });
      expect(unoccupied).toHaveLength(4);
      expect(unoccupied).toEqual(
        expect.arrayContaining([
          { q: 1, r: -1 },
          { q: 0, r: 1 },
          { q: -1, r: 1 },
          { q: 0, r: -1 }
        ])
      );
    });
  });
  describe('isBoardEmpty', () => {
    test('empty board', () => expect(isBoardEmpty({})).toBe(true));
    test('not empty board', () =>
      expect(isBoardEmpty(game0.boards[0])).toBe(false));
  });
  describe('isCoordinateOccupied', () => {
    test('occupied coordinate', () =>
      expect(isCoordinateOccupied(game0.boards[0], { q: 0, r: 0 })).toBe(true));
    test('unoccupied coordinate', () =>
      expect(isCoordinateOccupied(game0.boards[0], { q: 1, r: 0 })).toBe(
        false
      ));
  });
  describe('isCoordinateTouchingHive', () => {
    test('a coordinate that is part of the hive', () => {
      expect(isCoordinateTouchingHive(game0.boards[3], { q: 0, r: 0 })).toBe(
        true
      );
    });
    test('a coordinate touching the hive', () => {
      expect(isCoordinateTouchingHive(game0.boards[3], { q: 0, r: -1 })).toBe(
        true
      );
    });
    test('a coordinate not touching the hive', () => {
      expect(isCoordinateTouchingHive(game0.boards[3], { q: 4, r: 2 })).toBe(
        false
      );
    });
  });
  describe('isGated', () => {
    const table: [string, any][] = gates.map((gate, index) => [
      `gate example ${index}`,
      gate
    ]);
    test.each(table)('%s', (_, gate) => {
      expect(isGated(gate.board, gate.coordinate, gate.direction)).toBe(
        gate.gated
      );
    });
  });
  describe('isQueenPlaced', () => {
    test('queens on board', () => {
      const board = game0.boards[3];
      expect(isQueenPlaced(board, 'w')).toBe(true);
      expect(isQueenPlaced(board, 'b')).toBe(true);
    });
    test('queens not on board', () => {
      expect(isQueenPlaced({}, 'w')).toBe(false);
      expect(isQueenPlaced({}, 'b')).toBe(false);
    });
  });
  describe('isQueenSurrounded', () => {
    test('empty board', () => {
      expect(isQueenSurrounded({}, 'w')).toBe(false);
      expect(isQueenSurrounded({}, 'b')).toBe(false);
    });
    test('not surrounded', () => {
      const board = game0.boards[3];
      expect(isQueenSurrounded(board, 'w')).toBe(false);
      expect(isQueenSurrounded(board, 'b')).toBe(false);
    });
    test('surrounded', () => {
      expect(isQueenSurrounded(board2, 'w')).toBe(true);
      expect(isQueenSurrounded(board3, 'b')).toBe(true);
    });
  });
  describe('isTileStructural', () => {
    const board = game0.boards[3];
    test('internal tiles are structural', () => {
      expect(isTileStructural(board, 'wQ', { q: 0, r: 0 })).toBe(true);
      expect(isTileStructural(board, 'bS1', { q: -1, r: 0 })).toBe(true);
    });
    test('external tiles are not structural', () => {
      expect(isTileStructural(board, 'bQ', { q: -1, r: -1 })).toBe(false);
      expect(isTileStructural(board, 'wA1', { q: 1, r: 0 })).toBe(false);
    });
    test('stacks higher than 1 are not structural', () => {
      const withStack: GameBoard = {
        '-1': { 0: ['wA1'] },
        0: { 0: ['wQ', 'bQ'] },
        1: { 0: ['bA1'] }
      };
      expect(isTileStructural(withStack, 'bQ', { q: 0, r: 0 })).toBe(false);
    });
    test('a single tile on the board is not structural', () => {
      const singleTile: GameBoard = { 0: { 0: ['wQ'] } };
      expect(isTileStructural(singleTile, 'wQ', { q: 0, r: 0 })).toBe(false);
    });
  });
  describe('someNeighboringSpace', () => {
    const board = game0.boards[3];
    test("some neighbor has the tile we're looking for", () => {
      const result = someNeighboringSpace(board, { q: 0, r: 0 }, (_, ids) => {
        return ids !== undefined && ids.includes('wA1');
      });
      expect(result).toBe(true);
    });
    test('some neighbor does not have a stack', () => {
      const result = someNeighboringSpace(board, { q: 0, r: 0 }, (_, ids) => {
        return ids === undefined;
      });
      expect(result).toBe(true);
    });
    test("no neighbor has the tile we're looking for", () => {
      const result = someNeighboringSpace(board, { q: 0, r: 0 }, (_, ids) => {
        return ids && ids.includes('bL');
      });
      expect(result).toBe(false);
    });
  });
});

describe('game board iteration, search, and traversal', () => {
  describe('eachDirection', () => {
    test('visit all 6 directions', () => {
      const cb = jest.fn();
      const finished = eachDirection(cb);
      expect(cb).toHaveBeenCalledTimes(6);
      expect(cb).toHaveBeenCalledWith(1);
      expect(cb).toHaveBeenCalledWith(2);
      expect(cb).toHaveBeenCalledWith(3);
      expect(cb).toHaveBeenCalledWith(4);
      expect(cb).toHaveBeenCalledWith(5);
      expect(cb).toHaveBeenCalledWith(6);
      expect(finished).toBe(true);
    });
    test('early exit on direction = 3', () => {
      const cb = jest.fn();
      const finished = eachDirection((direction) => {
        cb();
        if (direction === 3) return false;
      });
      expect(cb).toHaveBeenCalledTimes(3);
      expect(finished).toBe(false);
    });
  });
  describe('eachUnoccupiedCoordinate', () => {
    test('visit all unoccupied coordinates', () => {
      const cb = jest.fn();
      const finished = eachUnoccupiedCoordinate({ 0: { 0: ['wA1'] } }, cb);
      expect(cb).toHaveBeenCalledTimes(6);
      expect(cb).toHaveBeenCalledWith({ q: 1, r: -1 });
      expect(cb).toHaveBeenCalledWith({ q: 1, r: 0 });
      expect(cb).toHaveBeenCalledWith({ q: 0, r: 1 });
      expect(cb).toHaveBeenCalledWith({ q: -1, r: 1 });
      expect(cb).toHaveBeenCalledWith({ q: -1, r: 0 });
      expect(cb).toHaveBeenCalledWith({ q: 0, r: -1 });
      expect(finished).toBe(true);
    });
    test('visit nothing on empty board', () => {
      const cb = jest.fn();
      const finished = eachUnoccupiedCoordinate({}, cb);
      expect(cb).toHaveBeenCalledTimes(0);
      expect(finished).toBe(true);
    });
  });
  describe('eachNeighboringSpace', () => {
    test('visit all 6 neighboring spaces', () => {
      const board = game0.boards[3];
      const cb = jest.fn();
      const finished = eachNeighboringSpace(board, { q: 0, r: 0 }, cb);
      expect(cb).toHaveBeenCalledTimes(6);
      expect(cb).toHaveBeenCalledWith({ q: 1, r: -1 }, undefined);
      expect(cb).toHaveBeenCalledWith({ q: 1, r: 0 }, ['wA1']);
      expect(cb).toHaveBeenCalledWith({ q: 0, r: 1 }, undefined);
      expect(cb).toHaveBeenCalledWith({ q: -1, r: 1 }, undefined);
      expect(cb).toHaveBeenCalledWith({ q: -1, r: 0 }, ['bS1']);
      expect(cb).toHaveBeenCalledWith({ q: 0, r: -1 }, undefined);
      expect(finished).toBe(true);
    });
    test('early exit after direction = 2', () => {
      const board = game0.boards[3];
      const cb = jest.fn();
      const finished = eachNeighboringSpace(
        board,
        { q: 0, r: 0 },
        (coordinate, ids) => {
          cb(coordinate, ids);
          if (hexesEqual(coordinate, { q: 1, r: 0 })) return false;
        }
      );
      expect(cb).toHaveBeenCalledTimes(2);
      expect(cb).toHaveBeenCalledWith({ q: 1, r: -1 }, undefined);
      expect(cb).toHaveBeenCalledWith({ q: 1, r: 0 }, ['wA1']);
      expect(finished).toBe(false);
    });
  });
  describe('eachNeighboringStack', () => {
    test('visit all neighboring stacks', () => {
      const board = game0.boards[3];
      const cb = jest.fn();
      const finished = eachNeighboringStack(board, { q: 0, r: 0 }, cb);
      expect(cb).toHaveBeenCalledTimes(2);
      expect(cb).toHaveBeenCalledWith({ q: 1, r: 0 }, ['wA1']);
      expect(cb).toHaveBeenCalledWith({ q: -1, r: 0 }, ['bS1']);
      expect(finished).toBe(true);
    });
    test('early exit after direction = 1', () => {
      const board = game0.boards[3];
      const cb = jest.fn();
      const finished = eachNeighboringStack(
        board,
        { q: 0, r: 0 },
        (coordinate, ids) => {
          cb(coordinate, ids);
          if (hexesEqual(coordinate, { q: 1, r: 0 })) return false;
        }
      );
      expect(cb).toHaveBeenCalledTimes(1);
      expect(cb).toHaveBeenCalledWith({ q: 1, r: 0 }, ['wA1']);
      expect(finished).toBe(false);
    });
  });
  describe('eachSlideDirection', () => {
    test('white queen', () => {
      const board = game0.boards[3];
      const cb = jest.fn();
      const finished = eachSlideDirection(board, { q: 0, r: 0 }, cb);
      expect(cb).toHaveBeenCalledTimes(4);
      expect(cb).toHaveBeenCalledWith({ q: 1, r: -1 }, [], 1);
      expect(cb).toHaveBeenCalledWith({ q: 0, r: 1 }, [], 3);
      expect(cb).toHaveBeenCalledWith({ q: -1, r: 1 }, [], 4);
      expect(cb).toHaveBeenCalledWith({ q: 0, r: -1 }, [], 6);
      expect(finished).toBe(true);
    });
    test('white ant', () => {
      const board = game0.boards[3];
      const cb = jest.fn();
      const finished = eachSlideDirection(board, { q: 1, r: 0 }, cb);
      expect(cb).toHaveBeenCalledTimes(2);
      expect(cb).toHaveBeenCalledWith({ q: 0, r: 1 }, [], 4);
      expect(cb).toHaveBeenCalledWith({ q: 1, r: -1 }, [], 6);
      expect(finished).toBe(true);
    });
    test('black queen, early exit', () => {
      const board = game0.boards[3];
      const cb = jest.fn();
      const finished = eachSlideDirection(
        board,
        { q: -1, r: -1 },
        (coordinate, stack, direction) => {
          cb(coordinate, stack, direction);
          return direction !== 2;
        }
      );
      expect(cb).toHaveBeenCalledTimes(1);
      expect(cb).toHaveBeenCalledWith({ q: 0, r: -1 }, [], 2);
      expect(finished).toBe(false);
    });
  });
  describe('eachStack', () => {
    test('visit correct number of coordinates', () => {
      const cb0 = jest.fn();
      const cb1 = jest.fn();
      const cb2 = jest.fn();
      const cb3 = jest.fn();
      eachStack(game0.boards[0], cb0);
      eachStack(game0.boards[1], cb1);
      eachStack(game0.boards[2], cb2);
      eachStack(game0.boards[3], cb3);
      expect(cb0).toHaveBeenCalledTimes(1);
      expect(cb1).toHaveBeenCalledTimes(2);
      expect(cb2).toHaveBeenCalledTimes(3);
      expect(cb3).toHaveBeenCalledTimes(4);
    });
    test('visit correct coordinates', () => {
      const visited: HexCoordinate[] = [];
      const finished = eachStack(game0.boards[3], (coordinate) =>
        visited.push(coordinate)
      );
      expect(visited).toHaveLength(4);
      expect(visited).toEqual(
        expect.arrayContaining([
          { q: 0, r: 0 },
          { q: -1, r: -1 },
          { q: -1, r: 0 },
          { q: 1, r: 0 }
        ])
      );
      expect(finished).toBe(true);
    });
    test('early exit after finding origin', () => {
      const visited: HexCoordinate[] = [];
      const finished = eachStack(game0.boards[3], (coordinate) => {
        visited.push(coordinate);
        if (hexesEqual(coordinate, { q: 0, r: 0 })) return false;
      });
      expect(visited).toEqual(expect.arrayContaining([{ q: 0, r: 0 }]));
      expect(finished).toBe(false);
    });
    test('visit every stack', () => {
      const board = game0.boards[3];
      const cb = jest.fn();
      const finished = eachStack(board, cb);
      expect(cb).toHaveBeenCalledTimes(4);
      expect(cb).toHaveBeenCalledWith({ q: -1, r: -1 }, ['bQ']);
      expect(cb).toHaveBeenCalledWith({ q: -1, r: 0 }, ['bS1']);
      expect(cb).toHaveBeenCalledWith({ q: 0, r: 0 }, ['wQ']);
      expect(cb).toHaveBeenCalledWith({ q: 1, r: 0 }, ['wA1']);
      expect(finished).toBe(true);
    });
    test('early exit once black spider 1 found', () => {
      const board = game0.boards[3];
      const cb = jest.fn();
      const finished = eachStack(board, (coordinate, ids) => {
        cb(coordinate, ids);
        if (ids.includes('bS1')) return false;
      });
      expect(cb).toHaveBeenCalledWith({ q: -1, r: 0 }, ['bS1']);
      expect(finished).toBe(false);
    });
  });
  describe('everyNeighbor', () => {
    test('all neighbors eval to true', () => {
      const board: GameBoard = {};
      const cb = jest.fn(() => true);
      const result = everyNeighbor(board, { q: 0, r: 0 }, cb);
      expect(cb).toHaveBeenCalledTimes(6);
      expect(result).toBe(true);
    });
    test('all neighbors eval to false', () => {
      const board: GameBoard = {};
      const cb = jest.fn(() => false);
      const result = everyNeighbor(board, { q: 0, r: 0 }, cb);
      expect(cb).toHaveBeenCalledTimes(1);
      expect(result).toBe(false);
    });
  });
  describe('findNeighborCoordinate', () => {
    const board = game0.boards[3];
    test('find the white ant', () => {
      const wA1 = findNeighborCoordinate(board, { q: 0, r: 0 }, (_, stack) => {
        return stack && stack.length && stack[0] === 'wA1';
      });
      expect(wA1).toEqual({ q: 1, r: 0 });
    });
  });
  describe('findTileCoordinate', () => {
    const board = game0.boards[3];
    test('find all tiles on example board', () => {
      const bQ = findTileCoordinate(board, 'bQ');
      const bS1 = findTileCoordinate(board, 'bS1');
      const wQ = findTileCoordinate(board, 'wQ');
      const wA1 = findTileCoordinate(board, 'wA1');
      expect(bQ).toEqual({ q: -1, r: -1 });
      expect(bS1).toEqual({ q: -1, r: 0 });
      expect(wQ).toEqual({ q: 0, r: 0 });
      expect(wA1).toEqual({ q: 1, r: 0 });
    });
    test('search for tile not on board', () => {
      const bL = findTileCoordinate(board, 'bL');
      const wP = findTileCoordinate(board, 'wP');
      expect(bL).toBeNull();
      expect(wP).toBeNull();
    });
  });
  describe('walkBoard', () => {
    const board = game0.boards[3];
    test('walk starting at each tile', () => {
      const cbs = [jest.fn(), jest.fn(), jest.fn(), jest.fn()];
      const starts = [
        { q: -1, r: -1 },
        { q: -1, r: 0 },
        { q: 0, r: 0 },
        { q: 1, r: 0 }
      ];
      starts.forEach((start, index) => {
        const cb = cbs[index];
        const order = walkBoard(board, start, cb);
        expect(cb).toHaveBeenCalledWith({ q: -1, r: -1 }, ['bQ']);
        expect(cb).toHaveBeenCalledWith({ q: -1, r: 0 }, ['bS1']);
        expect(cb).toHaveBeenCalledWith({ q: 0, r: 0 }, ['wQ']);
        expect(cb).toHaveBeenCalledWith({ q: 1, r: 0 }, ['wA1']);
        expect(order).toHaveLength(4);
        expect(order).toEqual(expect.arrayContaining(starts));
      });
    });
  });
});

describe('game board editing', () => {
  describe('immer producers', () => {
    const board: GameBoard = {};
    describe('_placeTile', () => {
      test('place tile in empty location', () => {
        _placeTile(board, 'wQ', { q: 0, r: 0 });
        expect(board).toEqual({
          '0': { '0': ['wQ'] }
        });
      });
      test('place tile on top of stack', () => {
        _placeTile(board, 'bQ', { q: 0, r: 0 });
        expect(board).toEqual({
          '0': { '0': ['wQ', 'bQ'] }
        });
      });
    });
    describe('_moveTile', () => {
      test('move top tile to adjacent hex', () => {
        _moveTile(board, 'bQ', { q: 0, r: 0 }, { q: 0, r: 1 });
        expect(board).toEqual({
          '0': { '0': ['wQ'], '1': ['bQ'] }
        });
      });
      test('move tile on top of adjacent stack', () => {
        _moveTile(board, 'bQ', { q: 0, r: 1 }, { q: 0, r: 0 });
        expect(board).toEqual({
          '0': { '0': ['wQ', 'bQ'] }
        });
      });
    });
    describe('_popTile', () => {
      test('error thrown when popping tile not on top of stack', () => {
        expect(() => _popTile(board, 'wQ', { q: 0, r: 0 })).toThrow();
      });
      test('remove tile on top of stack', () => {
        _popTile(board, 'bQ', { q: 0, r: 0 });
        expect(board).toEqual({
          '0': { '0': ['wQ'] }
        });
      });
      test('remove last tile from board', () => {
        _popTile(board, 'wQ', { q: 0, r: 0 });
        expect(board).toEqual({});
      });
    });
  });
  describe('placeTile', () => {
    test('stack tiles at origin', () => {
      const board0: GameBoard = {};
      const board1 = placeTile(board0, 'wQ', { q: 0, r: 0 });
      const board2 = placeTile(board1, 'bQ', { q: 0, r: 0 });
      expect(board0).toEqual({});
      expect(board1).toEqual({ 0: { 0: ['wQ'] } });
      expect(board2).toEqual({ 0: { 0: ['wQ', 'bQ'] } });
    });
  });
  describe('removeTile', () => {
    test('remove tiles from origin stack', () => {
      const board0: GameBoard = { 0: { 0: ['wQ', 'bQ'] } };
      const board1 = removeTile(board0, 'bQ', { q: 0, r: 0 });
      const board2 = removeTile(board1, 'wQ', { q: 0, r: 0 });
      expect(board0).toEqual({ 0: { 0: ['wQ', 'bQ'] } });
      expect(board1).toEqual({ 0: { 0: ['wQ'] } });
      expect(board2).toEqual({});
    });
  });
  describe('moveTile', () => {
    test('move two tiles to origin', () => {
      const board0: GameBoard = { 0: { 1: ['wQ'], 2: ['bQ'] } };
      const board1 = moveTile(board0, 'wQ', { q: 0, r: 1 }, { q: 0, r: 0 });
      const board2 = moveTile(board1, 'bQ', { q: 0, r: 2 }, { q: 0, r: 0 });
      expect(board0).toEqual({ 0: { 1: ['wQ'], 2: ['bQ'] } });
      expect(board1).toEqual({ 0: { 0: ['wQ'], 2: ['bQ'] } });
      expect(board2).toEqual({ 0: { 0: ['wQ', 'bQ'] } });
    });
  });
});
