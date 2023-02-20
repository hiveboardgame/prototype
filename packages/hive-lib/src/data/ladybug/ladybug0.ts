import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'wL';
const tileCoordinate: HexCoordinate = { q: 0, r: 0 };

const board: GameBoard = {
  '0': {
    '0': ['wL'],
    '-1': ['wB1'],
    '-2': ['wB2']
  },
  '1': {
    '-1': ['bB1'],
    '-3': ['bQ']
  },
  '2': {
    '-2': ['wQ']
  }
};

const expectedCoordinates: HexCoordinate[] = [
  {
    q: -1,
    r: 0
  },
  {
    q: 1,
    r: 0
  },
  {
    q: -1,
    r: -1
  },
  {
    q: -1,
    r: -2
  },
  {
    q: 0,
    r: -3
  },
  {
    q: 2,
    r: -3
  },
  {
    q: 1,
    r: -2
  },
  {
    q: 2,
    r: -1
  },
  {
    q: 3,
    r: -2
  },
  {
    q: 3,
    r: -3
  }
];

export default {
  board,
  tileId,
  tileCoordinate,
  expectedCoordinates
};
