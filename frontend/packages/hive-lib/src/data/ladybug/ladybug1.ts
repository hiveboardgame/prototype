import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'wL';
const tileCoordinate: HexCoordinate = { q: 3, r: -3 };

const board: GameBoard = {
  '0': {
    '-1': ['wB1'],
    '-2': ['wB2']
  },
  '1': {
    '-1': ['bB1'],
    '-3': ['bQ']
  },
  '2': {
    '-2': ['wQ']
  },
  '3': {
    '-3': ['wL']
  }
};

const expectedCoordinates: HexCoordinate[] = [
  {
    q: 1,
    r: -2
  },
  {
    q: 0,
    r: 0
  },
  {
    q: 1,
    r: 0
  },
  {
    q: 2,
    r: -1
  }
];

export default {
  board,
  tileId,
  tileCoordinate,
  expectedCoordinates
};
