import spider0 from './data/spider/spider0';
import spider1 from './data/spider/spider1';
import spider2 from './data/spider/spider2';
import { getValidSpiderMoveCoordinates } from './spider';

describe('spider moves', () => {
  const spiders = [
    { index: 0, ...spider0 },
    { index: 1, ...spider1 },
    { index: 2, ...spider2 }
  ];

  test.each(spiders)(
    'test position $index',
    ({ board, tileId, tileCoordinate, expectedCoordinates }) => {
      const validCoordinates = getValidSpiderMoveCoordinates(
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
