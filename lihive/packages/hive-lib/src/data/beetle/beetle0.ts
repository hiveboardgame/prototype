import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'wB1';
const tileCoordinate: HexCoordinate = { q: 0, r: 0 };

const board: GameBoard = {
  '0': {
    '0': ['wB1'],
    '2': ['wG1']
  },
  '-1': {
    '0': ['bS1'],
    '1': ['bA1'],
    '2': ['wQ']
  }
};

const expectedCoordinates: HexCoordinate[] = [
  {
    q: 0,
    r: -1
  },
  {
    q: -1,
    r: 0
  },
  {
    q: -1,
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
