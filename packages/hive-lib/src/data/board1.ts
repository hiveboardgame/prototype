import { GameBoard } from '../types';

const board1: GameBoard = {
  '0': {
    '0': ['wA1'],
    '1': ['wA3'],
    '-1': ['bA1']
  },
  '1': {
    '-1': ['bA2'],
    '-2': ['bA3']
  },
  '-1': {
    '1': ['wA2']
  }
};

export default board1;
