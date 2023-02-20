import queen0 from './data/queen/queen0';
import queen1 from './data/queen/queen1';
import queen2 from './data/queen/queen2';
import queen3 from './data/queen/queen3';
import { getValidQueenMoveCoordinates } from './queen';

describe('queen moves', () => {
  const queens = [
    { index: 0, ...queen0 },
    { index: 1, ...queen1 },
    { index: 2, ...queen2 },
    { index: 3, ...queen3 }
  ];

  test.each(queens)(
    'test position $index',
    ({ board, tileCoordinate, expectedCoordinates }) => {
      const validCoordinates = getValidQueenMoveCoordinates(
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
