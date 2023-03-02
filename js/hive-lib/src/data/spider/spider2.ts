import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'wS1';
const tileCoordinate: HexCoordinate = { q: 2, r: -2 };

const board: GameBoard = {
  '0': {
    '1': ['wA1'],
    '-2': ['wB1']
  },
  '1': {
    '0': ['bB1']
  },
  '2': {
    '-1': ['wQ'],
    '-2': ['wS1']
  },
  '-1': {
    '0': ['bS1'],
    '1': ['bG1'],
    '-1': ['bA1']
  },
  '-2': {
    '0': ['bQ'],
    '1': ['wG1']
  }
};

const expectedCoordinates: HexCoordinate[] = [
  {
    q: 0,
    r: -1
  },
  {
    q: 2,
    r: 0
  }
];

export default {
  board,
  tileId,
  tileCoordinate,
  expectedCoordinates
};
