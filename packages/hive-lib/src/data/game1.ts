import { GameBoard, Move, Turn } from '../types';

const notation =
  '1. wQ, bS1 -wQ 2. wA1 wQ-, bQ \\bS1 3. wA1 /bS1, bQ -bS1 4. x, x';
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
const move4: Move = {
  notation: 'wA1 /bS1',
  tileId: 'wA1',
  refId: 'bS1',
  dir: 4
};
const board4: GameBoard = {
  '-2': {
    1: ['wA1']
  },
  '-1': {
    '-1': ['bQ'],
    0: ['bS1']
  },
  0: {
    0: ['wQ']
  }
};
const move5: Move = {
  notation: 'bQ -bS1',
  tileId: 'bQ',
  refId: 'bS1',
  dir: 5
};
const board5: GameBoard = {
  '-2': {
    0: ['bQ'],
    1: ['wA1']
  },
  '-1': {
    0: ['bS1']
  },
  0: {
    0: ['wQ']
  }
};
const move6: Move = {
  notation: 'x',
  tileId: 'x',
  refId: 'x',
  dir: -1
};
const board6: GameBoard = board5;
const move7: Move = {
  notation: 'x',
  tileId: 'x',
  refId: 'x',
  dir: -1
};
const board7: GameBoard = board6;
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
const turn2: Turn = {
  notation: '3. wA1 /bS1, bQ -bS1',
  index: 3,
  white: move4,
  black: move5
};
const turn3: Turn = {
  notation: '4. x, x',
  index: 4,
  white: move6,
  black: move7
};

export default {
  notation,
  moves: [move0, move1, move2, move3, move4, move5, move6, move7],
  turns: [turn0, turn1, turn2, turn3],
  boards: [board0, board1, board2, board3, board4, board5, board6, board7]
};
