import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'bS1';
const tileCoordinate: HexCoordinate = { q: 2, r: -3 };

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
    '-2': ['wS1'],
    '-3': ['bS1']
  },
  '-1': {
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
    r: -3
  },
  {
    q: -1,
    r: 0
  },
  {
    q: 0,
    r: 0
  },
  {
    q: 3,
    r: -1
  }
];

export default {
  board,
  tileId,
  tileCoordinate,
  expectedCoordinates
};
