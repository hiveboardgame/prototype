import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'bQ';
const tileCoordinate: HexCoordinate = { q: 0, r: 0 };

const board: GameBoard = {
  '0': {
    '0': ['bQ'],
    '2': ['wA1']
  },
  '1': {
    '1': ['bB1'],
    '2': ['wQ'],
    '-1': ['bA1']
  },
  '2': {
    '0': ['wB1'],
    '-2': ['wS1'],
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
    r: 0
  },
  {
    q: 0,
    r: -1
  },
  {
    q: -1,
    r: 0
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
