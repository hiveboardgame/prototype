import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'wQ';
const tileCoordinate: HexCoordinate = { q: 0, r: 1 };

const board: GameBoard = {
  '0': {
    '0': ['wS1'],
    '1': ['wQ']
  },
  '1': {
    '1': ['wG1'],
    '-1': ['bA1']
  },
  '2': {
    '0': ['wB1'],
    '-1': ['bS1']
  }
};

const expectedCoordinates: HexCoordinate[] = [
  {
    q: 0,
    r: 2
  },
  {
    q: -1,
    r: 1
  }
];

export default {
  board,
  tileId,
  tileCoordinate,
  expectedCoordinates
};
