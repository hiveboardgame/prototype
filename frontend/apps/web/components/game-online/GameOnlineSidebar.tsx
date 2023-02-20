import { Tab, TabList, TabPanel, TabPanels, Tabs } from '@chakra-ui/react';
import { useCallback } from 'react';
import {
  useGameDispatch,
  useGameSelector
} from '../../state/game-online/hooks';
import {
  selectDisplayUpTo,
  selectMoves
} from '../../state/game-online/selectors';
import { viewUpToPicked } from '../../state/game-online/slice';
import { NotationView } from '../game-common/NotationView';
import { DebugPanel } from './DebugPanel';
import { GameOnlineBoardControls } from './GameOnlineBoardControls';
import { GameOnlineGameControls } from './GameOnlineGameControls';
import { GameOnlineHands } from './GameOnlineHands';
import { GameOnlineMessage } from './GameOnlineMessage';

const GameOnlineSidebar = () => {
  const dispatch = useGameDispatch();
  const moves = useGameSelector(selectMoves);
  const upTo = useGameSelector(selectDisplayUpTo);
  const onHoverMove = useCallback(
    (index: number) => {
      dispatch(viewUpToPicked(index + 1));
    },
    [dispatch]
  );

  return (
    <div className='absolute flex flex-col top-4 right-4 bottom-4 w-[360px] pointer-events-none'>
      <div className='shrink mb-4'>
        <div className='flex'>
          <GameOnlineBoardControls />
          <div className='flex grow ml-2 flex-col border border-slate-200 rounded bg-white pointer-events-auto'>
            <GameOnlineMessage />
            <GameOnlineGameControls />
          </div>
        </div>
      </div>
      <Tabs
        className='flex flex-col min-h-0 bg-white pointer-events-auto'
        isFitted
        variant='enclosed-colored'
        size='sm'
      >
        <TabList>
          <Tab>Game</Tab>
          <Tab>History</Tab>
          <Tab>Debug</Tab>
        </TabList>
        <TabPanels
          className='flex flex-col min-h-0'
          border='1px'
          borderColor='gray.200'
          borderBottomRadius='base'
        >
          <TabPanel className='flex flex-col overflow-auto'>
            <GameOnlineHands />
          </TabPanel>
          <TabPanel className='flex flex-col overflow-auto'>
            <NotationView
              ml={3}
              moves={moves}
              upTo={upTo}
              onHoverMove={onHoverMove}
            />
          </TabPanel>
          <TabPanel className='flex flex-col overflow-auto'>
            <DebugPanel />
          </TabPanel>
        </TabPanels>
      </Tabs>
    </div>
  );
};

export { GameOnlineSidebar };
