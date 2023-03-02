import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'wA1';
const tileCoordinate: HexCoordinate = { q: 0, r: 0 };

const board: GameBoard = {
  '0': {
    '0': ['wA1'],
    '1': ['bS1'],
    '-1': ['bA1']
  },
  '1': {
    '0': ['bS2'],
    '-1': ['bA3']
  }
};

const expectedCoordinates: HexCoordinate[] = [
  {
    q: -1,
    r: 1
  },
  {
    q: -1,
    r: 0
  },
  {
    q: -1,
    r: -1
  },
  {
    q: 0,
    r: -2
  },
  {
    q: 1,
    r: -2
  },
  {
    q: 2,
    r: -2
  },
  {
    q: 2,
    r: -1
  },
  {
    q: 2,
    r: 0
  },
  {
    q: 1,
    r: 1
  },
  {
    q: 0,
    r: 2
  },
  {
    q: -1,
    r: 2
  }
];

export default {
  board,
  tileId,
  tileCoordinate,
  expectedCoordinates
};
