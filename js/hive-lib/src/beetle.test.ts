import { getValidBeetleMoveCoordinates } from './beetle';
import beetle0 from './data/beetle/beetle0';
import beetle1 from './data/beetle/beetle1';
import beetle2 from './data/beetle/beetle2';

describe('beetle moves', () => {
  const beetles = [
    { index: 0, ...beetle0 },
    { index: 1, ...beetle1 },
    { index: 2, ...beetle2 }
  ];

  test.each(beetles)(
    'test position $index',
    ({ board, tileCoordinate, expectedCoordinates }) => {
      const validCoordinates = getValidBeetleMoveCoordinates(
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
