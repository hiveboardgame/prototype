import { useGameDispatch } from '../../state/game-online/hooks';
import { boardCentered } from '../../state/game-online/slice';
import { BoardControls } from '../game-common/BoardControls';

const GameOnlineBoardControls = () => {
  const dispatch = useGameDispatch();
  return (
    <BoardControls
      onClickCenter={() => {
        dispatch(boardCentered());
      }}
    />
  );
};

export { GameOnlineBoardControls };
