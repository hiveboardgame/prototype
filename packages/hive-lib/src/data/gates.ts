import { GameBoard } from '../types';

const board0: GameBoard = {
  '-1': {
    '0': ['bQ'],
    '1': ['wA1']
  },
  '0': {
    '-1': ['wB1']
  },
  '1': {
    '-1': ['wQ'],
    '0': ['wS1']
  }
};

const board1: GameBoard = {
  '-1': {
    '0': ['bQ']
  },
  '0': {
    '-1': ['wB1']
  },
  '1': {
    '-1': ['wQ'],
    '0': ['wS1']
  }
};

const board3: GameBoard = {
  '0': {
    '-1': ['wB1', 'bQ']
  },
  '1': {
    '-1': ['wQ'],
    '0': ['wS1', 'wA1']
  }
};

export default [
  {
    board: board0,
    coordinate: { q: 0, r: 0 },
    direction: 3,
    gated: true
  },
  {
    board: board0,
    coordinate: { q: 0, r: 1 },
    direction: 6,
    gated: true
  },
  {
    board: board1,
    coordinate: { q: 0, r: 0 },
    direction: 3,
    gated: false
  },
  {
    board: board1,
    coordinate: { q: 0, r: 0 },
    direction: 4,
    gated: false
  },
  {
    board: board0,
    coordinate: { q: 0, r: 0 },
    direction: 1,
    gated: false
  },
  {
    board: board3,
    coordinate: { q: 0, r: 0 },
    direction: 1,
    gated: true
  },
  {
    board: board3,
    coordinate: { q: 1, r: -1 },
    direction: 4,
    gated: true
  }
];
