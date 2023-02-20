import { As, FlexProps, forwardRef } from '@chakra-ui/react';
import { useCallback } from 'react';
import {
  useGameDispatch,
  useGameSelector
} from '../../state/game-online/hooks';
import {
  selectDisplayUpTo,
  selectHasNewMovesToView,
  selectMoves
} from '../../state/game-online/selectors';
import {
  firstMoveClicked,
  lastMoveClicked,
  nextMoveClicked,
  previousMoveClicked
} from '../../state/game-online/slice';
import { GameControls } from '../game-common/GameControls';

const GameOnlineGameControls = forwardRef<FlexProps, As>((props, ref) => {
  const dispatch = useGameDispatch();
  const hasNewMoves = useGameSelector(selectHasNewMovesToView);
  const moves = useGameSelector(selectMoves);
  const viewUpTo = useGameSelector(selectDisplayUpTo);

  const handleFirstMoveClicked = useCallback(
    () => dispatch(firstMoveClicked()),
    [dispatch]
  );
  const handleLastMoveClicked = useCallback(
    () => dispatch(lastMoveClicked()),
    [dispatch]
  );
  const handleNextMoveClicked = useCallback(
    () => dispatch(nextMoveClicked()),
    [dispatch]
  );
  const handlePreviousMoveClicked = useCallback(
    () => dispatch(previousMoveClicked()),
    [dispatch]
  );

  return (
    <GameControls
      pulseLast={hasNewMoves}
      totalMoves={moves.length}
      viewUpTo={viewUpTo}
      onClickFirst={handleFirstMoveClicked}
      onClickLast={handleLastMoveClicked}
      onClickNext={handleNextMoveClicked}
      onClickPrevious={handlePreviousMoveClicked}
      ref={ref}
      {...props}
    />
  );
});

export { GameOnlineGameControls };
