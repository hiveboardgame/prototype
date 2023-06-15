import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';
import { Provider } from 'react-redux';
import { Game, usePlayer, getGame, newGameFromBackendGame } from 'hive-db';
import { GameOnline } from '../../components/game-online/GameOnline';
import { GameOnlineSidebar } from '../../components/game-online/GameOnlineSidebar';
import { NavBar } from '../../components/nav/NavBar';
import { useNotifications } from '../../contexts/notifications/NotificationProvider';
import { useTitle } from '../../hooks/useTitle';
import { useGameDispatch } from '../../state/game-online/hooks';
import { gameChanged, uidChanged } from '../../state/game-online/slice';
import store from '../../state/game-online/store';
import Head from 'next/head';

const GameView = ({ uid, game }: { uid: string | null; game: Game }) => {
  const dispatch = useGameDispatch();
  const { notifications, markRead } = useNotifications();

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

  /**
   * If there's a notification that it's the user's turn in this game, mark it
   * as read since we're already here.
   */
  useEffect(() => {
    const notification = notifications.find((n) => {
      return n.gid === game.gid && !n.read;
    });
    if (notification) {
      markRead([notification]);
    }
  }, [notifications, markRead, game]);

  return (
    <>
      <GameOnline uid={uid} game={game} />
      <GameOnlineSidebar />
    </>
  );
};

const Game = () => {
  const router = useRouter();
  const { user, activeGames } = usePlayer();
  const { gameid } = router.query;
  const [game, setGame] = useState<Game | undefined>();
  const title = useTitle();

  useEffect(() => {
    // TODO: should this be a strict type check while enforcing that gameid is a string?
    if (gameid) {
      getGame(gameid).then((game) => {
        console.log(game);
        setGame(newGameFromBackendGame(game));
      });
    }
  }, [gameid]);

  return (
    <>
      <Head>
        <title>{title}</title>
      </Head>
      <NavBar fullWidth className='border-b' />
      <div className='relative w-full h-full overflow-hidden'>
        <Provider store={store}>
          {game && user ? (
            <GameView uid={user.uid} game={game} />
          ) : (
            'Loading...'
          )}
        </Provider>
      </div>
    </>
  );
};

export default Game;
