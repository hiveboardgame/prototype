import { Move, TileId } from '../types';

const moves: Move[] = [
  {
    notation: 'wQ',
    tileId: 'wQ',
    refId: 'wQ',
    dir: 0
  },
  {
    notation: 'bQ \\wQ',
    tileId: 'bQ',
    refId: 'wQ',
    dir: 6
  },
  {
    notation: 'wB1 wQ-',
    tileId: 'wB1',
    refId: 'wQ',
    dir: 2
  },
  {
    notation: 'bB1 -bQ',
    tileId: 'bB1',
    refId: 'bQ',
    dir: 5
  },
  {
    notation: 'wB1 -wB1',
    tileId: 'wB1',
    refId: 'wB1',
    dir: 5
  },
  {
    notation: 'bB1 \\wB1',
    tileId: 'bB1',
    refId: 'wB1',
    dir: 6
  },
  {
    notation: 'wB1 \\wB1',
    tileId: 'wB1',
    refId: 'wB1',
    dir: 6
  },
  {
    notation: 'x',
    tileId: 'x',
    refId: 'x',
    dir: -1
  },
  {
    notation: 'wS1 wQ/',
    tileId: 'wS1',
    refId: 'wQ',
    dir: 1
  },
  {
    notation: 'x',
    tileId: 'x',
    refId: 'x',
    dir: -1
  },
  {
    notation: 'wA1 \\wS1',
    tileId: 'wA1',
    refId: 'wS1',
    dir: 6
  },
  {
    notation: 'x',
    tileId: 'x',
    refId: 'x',
    dir: -1
  },
  {
    notation: 'wA2 -wA1',
    tileId: 'wA2',
    refId: 'wA1',
    dir: 5
  },
  {
    notation: 'x',
    tileId: 'x',
    refId: 'x',
    dir: -1
  },
  {
    notation: 'wG1 /wA2',
    tileId: 'wG1',
    refId: 'wA2',
    dir: 4
  },
  {
    notation: 'x',
    tileId: 'x',
    refId: 'x',
    dir: -1
  }
];

const endingMoveTile: TileId = 'wS2';

export default {
  moves,
  endingMoveTile,
  endingMoveCoordinate: { q: -1, r: 0 },
  endingNotation:
    '1. wQ, bQ \\wQ 2. wB1 wQ-, bB1 -bQ 3. wB1 -wB1, bB1 \\wB1 4. wB1 \\wB1, x 5. wS1 wQ/, x 6. wA1 \\wS1, x 7. wA2 -wA1, x 8. wG1 /wA2, x 9. wS2 /wB1#'
};
