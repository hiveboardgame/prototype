import {
  ColorKey,
  GameBoard,
  GameOptions,
  HexCoordinate,
  TileId
} from '../../types';

const mover: ColorKey = 'w';
const options: GameOptions = {
  tournament: true,
  ladybug: true,
  mosquito: true,
  pillbug: true
};
const tileId: TileId = 'wS1';
const lastId: TileId | null = 'wG1';
const board: GameBoard = {
  '0': {
    '0': ['wL'],
    '-2': ['bP'],
    '-3': ['wG1']
  },
  '1': {
    '0': ['wB1', 'bM'],
    '-2': ['bQ', 'wM']
  },
  '2': {
    '-1': ['wB2'],
    '-3': ['wS1'],
    '-2': ['wP']
  },
  '3': {
    '-1': ['bA1']
  },
  '-1': {
    '1': ['wQ'],
    '2': ['bG2'],
    '-2': ['bA2'],
    '-3': ['wA1'],
    '-4': ['bG1']
  },
  '-2': {
    '1': ['bL'],
    '2': ['bA3']
  }
};

const expectedCoordinates: HexCoordinate[] = [
  { q: 3, r: -3 },
  { q: 3, r: -2 },
  { q: 4, r: -2 },
  { q: 1, r: -1 },
  { q: 0, r: -4 }
];

export default {
  mover,
  options,
  tileId,
  lastId,
  board,
  expectedCoordinates
};
