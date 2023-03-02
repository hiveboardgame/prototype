import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'wQ';
const tileCoordinate: HexCoordinate = { q: 1, r: 2 };

const board: GameBoard = {
  '0': {
    '0': ['wS1'],
    '1': ['bQ']
  },
  '1': {
    '2': ['wQ'],
    '-1': ['bA1']
  },
  '2': {
    '0': ['wB1'],
    '1': ['bB1'],
    '-1': ['bS1']
  },
  '-1': {
    '1': ['bG1'],
    '2': ['wG1']
  }
};

const expectedCoordinates: HexCoordinate[] = [
  {
    q: 1,
    r: 1
  },
  {
    q: 2,
    r: 2
  }
];

export default {
  board,
  tileId,
  tileCoordinate,
  expectedCoordinates
};
