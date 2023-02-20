import ladybug0 from './data/ladybug/ladybug0';
import ladybug1 from './data/ladybug/ladybug1';
import ladybug2 from './data/ladybug/ladybug2';
import { getValidLadybugMoveCoordinates } from './ladybug';

describe('ladybug moves', () => {
  const ladybugs = [
    { index: 0, ...ladybug0 },
    { index: 1, ...ladybug1 },
    { index: 2, ...ladybug2 }
  ];

  test.each(ladybugs)(
    'test position $index',
    ({ board, tileId, tileCoordinate, expectedCoordinates }) => {
      const validCoordinates = getValidLadybugMoveCoordinates(
        board,
        tileId,
        tileCoordinate
      );
      expect(validCoordinates).toHaveLength(expectedCoordinates.length);
      expect(validCoordinates).toEqual(
        expect.arrayContaining(expectedCoordinates)
      );
    }
  );
});
