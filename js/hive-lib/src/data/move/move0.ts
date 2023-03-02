import { GameBoard, GameOptions } from '../../types';

const board: GameBoard = {
  '0': {
    '0': ['wB1'],
    '1': ['bL'],
    '2': ['bM']
  },
  '1': {
    '1': ['bQ'],
    '-1': ['wM']
  },
  '-1': {
    '0': ['wG1']
  }
};

const options: GameOptions = {
  tournament: true,
  ladybug: true,
  mosquito: true,
  pillbug: true
};

export { board, options };
