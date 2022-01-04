import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';
import { Provider } from 'react-redux';
import { Game, usePlayer, watchGame } from 'hive-db';
import { GameOnline } from '../../components/game-online/GameOnline';
import { GameOnlineSidebar } from '../../components/game-online/GameOnlineSidebar';
import { NavBar } from '../../components/nav/NavBar';
import { useGameDispatch } from '../../state/game-online/hooks';
import { gameChanged, uidChanged } from '../../state/game-online/slice';
import store from '../../state/game-online/store';
import Head from 'next/head';

const GameView = ({ uid, game }: { uid: string | null; game: Game }) => {
  const dispatch = useGameDispatch();

  /**
   * The game is the actual source of truth, so tell the store when it changes
   * so that the game interface can update accordingly.
   */
  useEffect(() => {
    dispatch(gameChanged(game));
  }, [game, dispatch]);

  /**
   * We also need to know who is viewing the game so that we know what level
   * of interactivity to provide.
   */
  useEffect(() => {
    dispatch(uidChanged(uid));
  }, [uid, dispatch]);

  return (
    <>
      <GameOnline uid={uid} game={game} />
      <GameOnlineSidebar />
    </>
  );
};

const Game = () => {
  const router = useRouter();
  const { uid } = usePlayer();
  const { gameid } = router.query;
  const [game, setGame] = useState<Game | undefined>();

  useEffect(() => {
    if (typeof gameid === 'string') {
      return watchGame(gameid, setGame, () => setGame(undefined));
    }
  }, [gameid]);

  return (
    <>
      <Head>
        <title>lihive.org • Free Online Hive</title>
      </Head>
      <NavBar fullWidth className='border-b' />
      <div className='relative w-full h-full overflow-hidden'>
        <Provider store={store}>
          {game ? <GameView uid={uid} game={game} /> : 'Loading...'}
        </Provider>
      </div>
    </>
  );
};

export default Game;
