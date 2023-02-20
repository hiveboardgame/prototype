import { GameBoard, HexCoordinate, TileId } from '../../types';

const tileId: TileId = 'wM';
const tileCoordinate: HexCoordinate = { q: 0, r: 0 };

const board: GameBoard = {
  '0': {
    '0': ['wM'],
    '-1': ['bM']
  }
};

const expectedCoordinates: HexCoordinate[] = [];

export default {
  tileId,
  tileCoordinate,
  board,
  expectedCoordinates
};
