import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'wM';
const tileCoordinate: HexCoordinate = { q: 0, r: 0 };

const board: GameBoard = {
  '0': {
    '0': ['wM'],
    '-1': ['bB1']
  },
  '1': {
    '-1': ['bG1']
  }
};

const expectedCoordinates: HexCoordinate[] = [
  {
    q: 0,
    r: -1
  },
  {
    q: 1,
    r: -1
  },
  {
    q: -1,
    r: 0
  },
  {
    q: 1,
    r: 0
  },
  {
    q: 2,
    r: -2
  },
  {
    q: 0,
    r: -2
  }
];

export default {
  tileId,
  tileCoordinate,
  board,
  expectedCoordinates
};
