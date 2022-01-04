import pillbug0 from './data/pillbug/pillbug0';
import pillbug1 from './data/pillbug/pillbug1';
import pillbug2 from './data/pillbug/pillbug2';
import pillbug3 from './data/pillbug/pillbug3';
import pillbug4 from './data/pillbug/pillbug4';
import pillbug5 from './data/pillbug/pillbug5';
import {
  getValidPillbugMoveCoordinates,
  getValidPillbugPushCoordinates
} from './pillbug';

describe('pillbug moves', () => {
  const pillbugs = [
    { index: 0, ...pillbug0 },
    { index: 1, ...pillbug1 }
  ];

  test.each(pillbugs)(
    'test position $index',
    ({ board, tileCoordinate, expectedCoordinates }) => {
      const validCoordinates = getValidPillbugMoveCoordinates(
        board,
        tileCoordinate
      );
      expect(validCoordinates).toHaveLength(expectedCoordinates.length);
      expect(validCoordinates).toEqual(
        expect.arrayContaining(expectedCoordinates)
      );
    }
  );
});

describe('pillbug pushes', () => {
  const pillbugs = [
    { index: 0, ...pillbug2 },
    { index: 1, ...pillbug3 },
    { index: 2, ...pillbug4 },
    { index: 3, ...pillbug5 }
  ];

  test.each(pillbugs)(
    'test position $index',
    ({
      board,
      tileId,
      tileCoordinate,
      pillCoordinate,
      expectedCoordinates
    }) => {
      const validCoordinates = getValidPillbugPushCoordinates(
        board,
        tileId,
        tileCoordinate,
        pillCoordinate
      );
      expect(validCoordinates).toHaveLength(expectedCoordinates.length);
      expect(validCoordinates).toEqual(
        expect.arrayContaining(expectedCoordinates)
      );
    }
  );
});
