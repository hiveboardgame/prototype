import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'wG1';
const tileCoordinate: HexCoordinate = { q: 0, r: 0 };

const board: GameBoard = {
  '0': {
    '0': ['wG1'],
    '1': ['wB1'],
    '-1': ['bQ']
  },
  '1': {
    '0': ['bG1'],
    '-2': ['bA1']
  },
  '2': {
    '0': ['wQ'],
    '-2': ['wA1'],
    '-1': ['wS1']
  },
  '3': {
    '1': ['bS2'],
    '-1': ['bB1']
  },
  '4': {
    '0': ['wS2'],
    '-1': ['bS1']
  }
};

const expectedCoordinates: HexCoordinate[] = [
  {
    q: 0,
    r: 2
  },
  {
    q: 0,
    r: -2
  },
  {
    q: 3,
    r: 0
  }
];

export default {
  board,
  tileId,
  tileCoordinate,
  expectedCoordinates
};
