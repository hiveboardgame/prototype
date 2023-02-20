import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'wL';
const tileCoordinate: HexCoordinate = { q: 0, r: 0 };

const board: GameBoard = {
  '0': {
    '0': ['wL'],
    '-1': ['wB2']
  }
};

const expectedCoordinates: HexCoordinate[] = [];

export default {
  board,
  tileId,
  tileCoordinate,
  expectedCoordinates
};
