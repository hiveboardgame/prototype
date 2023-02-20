import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'bA1';
const tileCoordinate: HexCoordinate = { q: -1, r: 1 };
const pillCoordinate: HexCoordinate = { q: 0, r: 0 };

const board: GameBoard = {
  '0': {
    '0': ['wP'],
    '1': ['bA2']
  },
  '-1': {
    '0': ['bA3', 'wA2'],
    '1': ['bA1']
  }
};

const expectedCoordinates: HexCoordinate[] = [
  {
    q: 1,
    r: 0
  },
  {
    q: 1,
    r: -1
  },
  {
    q: 0,
    r: -1
  }
];

export default {
  tileId,
  tileCoordinate,
  pillCoordinate,
  board,
  expectedCoordinates
};
