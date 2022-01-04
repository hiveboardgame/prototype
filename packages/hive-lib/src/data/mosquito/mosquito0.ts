import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'wM';
const tileCoordinate: HexCoordinate = { q: 0, r: 0 };

const board: GameBoard = {
  '0': {
    '0': ['wM'],
    '-1': ['bA1']
  }
};

const expectedCoordinates: HexCoordinate[] = [
  {
    q: -1,
    r: 0
  },
  {
    q: -1,
    r: -1
  },
  {
    q: 0,
    r: -2
  },
  {
    q: 1,
    r: -2
  },
  {
    q: 1,
    r: -1
  }
];

export default {
  tileId,
  tileCoordinate,
  board,
  expectedCoordinates
};
