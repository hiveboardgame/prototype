import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'wM';
const tileCoordinate: HexCoordinate = { q: 0, r: 0 };

const board: GameBoard = {
  '0': {
    '0': ['bG1', 'wM'],
    '-1': ['bB1']
  },
  '1': {
    '-1': ['wA1', 'wA2']
  },
  '-1': {
    '0': ['bA1', 'bA2']
  }
};

const expectedCoordinates: HexCoordinate[] = [
  {
    q: -1,
    r: 0
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
  },
  {
    q: -1,
    r: 1
  }
];

export default {
  tileId,
  tileCoordinate,
  board,
  expectedCoordinates
};
