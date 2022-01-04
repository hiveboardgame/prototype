import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'wA1';
const tileCoordinate: HexCoordinate = { q: 0, r: 0 };

const board: GameBoard = {
  '0': {
    '0': ['wA1'],
    '1': ['bS1'],
    '-1': ['bA1']
  },
  '1': {
    '0': ['bS2'],
    '-1': ['bA3']
  },
  '-1': {
    '1': ['bA2']
  }
};

const expectedCoordinates: HexCoordinate[] = [];

export default {
  tileId,
  tileCoordinate,
  board,
  expectedCoordinates
};
