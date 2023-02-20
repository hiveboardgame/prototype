import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'wA1';
const tileCoordinate: HexCoordinate = { q: -1, r: 1 };
const pillCoordinate: HexCoordinate = { q: 0, r: 0 };

const board: GameBoard = {
  '0': {
    '0': ['bP']
  },
  '-1': {
    '1': ['wA1']
  }
};

const expectedCoordinates: HexCoordinate[] = [
  {
    q: -1,
    r: 0
  },
  {
    q: 0,
    r: -1
  },
  {
    q: 1,
    r: -1
  },
  {
    q: 1,
    r: 0
  },
  {
    q: 0,
    r: 1
  }
];

export default {
  tileId,
  tileCoordinate,
  pillCoordinate,
  board,
  expectedCoordinates
};
