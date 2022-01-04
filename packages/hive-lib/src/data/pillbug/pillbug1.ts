import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'bP';
const tileCoordinate: HexCoordinate = { q: 1, r: -1 };

const board: GameBoard = {
  '0': {
    '0': ['wP'],
    '-1': ['wQ']
  },
  '1': {
    '-1': ['bP']
  },
  '-1': {
    '0': ['bM'],
    '1': ['bQ']
  }
};

const expectedCoordinates: HexCoordinate[] = [
  {
    q: 1,
    r: 0
  },
  {
    q: 1,
    r: -2
  }
];

export default {
  tileId,
  tileCoordinate,
  board,
  expectedCoordinates
};
