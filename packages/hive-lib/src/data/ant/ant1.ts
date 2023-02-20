import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'wA1';
const tileCoordinate: HexCoordinate = { q: 0, r: 0 };

const board: GameBoard = {
  '0': {
    '0': ['wA1'],
    '-1': ['bA1']
  },
  '1': {
    '0': ['bS2'],
    '-2': ['bA2']
  },
  '2': {
    '-2': ['bA3'],
    '-1': ['bS1']
  }
};

const expectedCoordinates: HexCoordinate[] = [
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
    r: -3
  },
  {
    q: 2,
    r: -3
  },
  {
    q: 3,
    r: -3
  },
  {
    q: 3,
    r: -2
  },
  {
    q: 3,
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
    r: 1
  }
];

export default {
  board,
  tileId,
  tileCoordinate,
  expectedCoordinates
};
