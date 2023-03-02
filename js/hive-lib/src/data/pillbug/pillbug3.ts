import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'bA1';
const tileCoordinate: HexCoordinate = { q: -1, r: 1 };
const pillCoordinate: HexCoordinate = { q: 0, r: 0 };

const board: GameBoard = {
  '0': {
    '0': ['wP'],
    '1': ['bA2', 'wA1']
  },
  '-1': {
    '0': ['bA3', 'wA2'],
    '1': ['bA1']
  }
};

const expectedCoordinates: HexCoordinate[] = [];

export default {
  tileId,
  tileCoordinate,
  pillCoordinate,
  board,
  expectedCoordinates
};
