import { Game } from 'hive-db';
import { HexCoordinate, TileId } from 'hive-lib';
import { useCallback } from 'react';
import {
  useGameDispatch,
  useGameSelector
} from '../../state/game-online/hooks';
import {
  selectBoardCentered,
  selectDisplayBoardStacks,
  selectSelectedTileId,
  selectValidMovesForTile
} from '../../state/game-online/selectors';
import {
  ghostClicked,
  tableClicked,
  tableStackClicked
} from '../../state/game-online/slice';
import { Table } from '../game-common/Table';

const GameOnline = ({ uid, game }: { uid: string | null; game: Game }) => {
  const hexSize = 50;
  const tilePadding = 3;
  const dispatch = useGameDispatch();
  const stacks = useGameSelector(selectDisplayBoardStacks);
  const ghosts = useGameSelector(selectValidMovesForTile);
  const selectedTileId = useGameSelector(selectSelectedTileId);
  const boardCentered = useGameSelector(selectBoardCentered);

  const onClickTable = useCallback(() => dispatch(tableClicked()), [dispatch]);
  const onClickTableStack = useCallback(
    (coordinate: HexCoordinate, stack: TileId[]) =>
      dispatch(tableStackClicked({ coordinate, stack })),
    [dispatch]
  );
  const onClickGhost = useCallback(
    (coordinate: HexCoordinate) => dispatch(ghostClicked(coordinate)),
    [dispatch]
  );

  return (
    <Table
      hexSize={hexSize}
      tilePadding={tilePadding}
      stacks={stacks}
      validMoves={ghosts}
      selectedTileId={selectedTileId || undefined}
      onClickGhost={onClickGhost}
      onClickTableStack={onClickTableStack}
      onClickTable={onClickTable}
      boardCentered={boardCentered}
    />
  );
};

export { GameOnline };
