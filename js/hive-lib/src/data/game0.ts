import { GameBoard, Move, Turn } from '../types';

const notation = '1. wQ, bS1 -wQ 2. wA1 wQ-, bQ \\bS1';
const move0: Move = {
  notation: 'wQ',
  tileId: 'wQ',
  refId: 'wQ',
  dir: 0
};
const board0: GameBoard = {
  0: {
    0: ['wQ']
  }
};
const move1: Move = {
  notation: 'bS1 -wQ',
  tileId: 'bS1',
  refId: 'wQ',
  dir: 5
};
const board1: GameBoard = {
  '-1': {
    0: ['bS1']
  },
  0: {
    0: ['wQ']
  }
};
const move2: Move = {
  notation: 'wA1 wQ-',
  tileId: 'wA1',
  refId: 'wQ',
  dir: 2
};
const board2: GameBoard = {
  '-1': {
    0: ['bS1']
  },
  0: {
    0: ['wQ']
  },
  1: {
    0: ['wA1']
  }
};
const move3: Move = {
  notation: 'bQ \\bS1',
  tileId: 'bQ',
  refId: 'bS1',
  dir: 6
};
const board3: GameBoard = {
  '-1': {
    '-1': ['bQ'],
    0: ['bS1']
  },
  0: {
    0: ['wQ']
  },
  1: {
    0: ['wA1']
  }
};
const turn0: Turn = {
  notation: '1. wQ, bS1 -wQ',
  index: 1,
  white: move0,
  black: move1
};
const turn1: Turn = {
  notation: '2. wA1 wQ-, bQ \\bS1',
  index: 2,
  white: move2,
  black: move3
};

export default {
  notation,
  moves: [move0, move1, move2, move3],
  turns: [turn0, turn1],
  boards: [board0, board1, board2, board3]
};
