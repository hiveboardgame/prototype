import { useRouter } from 'next/router';
import { HTMLAttributes } from 'react';
import {
  Game,
  getIsLadybugUsed,
  getIsMosquitoUsed,
  getIsPillbugUsed,
  getLastPlayDate,
  getMoveCount,
  getOpponentColor,
  getOpponentUsername,
  getTurnUid,
  sortByLastPlayed,
  UserData
} from 'hive-db';
import { HiveIcon } from '../common/HiveIcon';
import { Header, HeaderItem } from './Header';
import { ElapsedTimeItem } from './items/ElapsedTimeItem';
import { ExpansionsItem } from './items/ExpansionsItem';
import { PlayerItem } from './items/PlayerItem';
import { Row, RowItem } from './Row';

interface PlayerGameRowProps {
  user: UserData;
  game: Game;
}

const PlayerGameRow = ({ user, game }: PlayerGameRowProps) => {
  const router = useRouter();
  const uid = user.uid;
  const opponent = getOpponentUsername(game, uid);
  const opponentColor = getOpponentColor(game, uid);
  const ladybug = getIsLadybugUsed(game);
  const mosquito = getIsMosquitoUsed(game);
  const pillbug = getIsPillbugUsed(game);
  const updated = getLastPlayDate(game);
  const moveCount = getMoveCount(game);
  const isOwnTurn = getTurnUid(game) === uid;
  return (
    <Row onClick={() => router.push(`/game/${game.gid}`)}>
      <PlayerItem
        mode='bar'
        playerName={opponent}
        playerColor={opponentColor}
        isTurn={isOwnTurn}
      />
      <ElapsedTimeItem since={updated} placeholder='-' />
      <RowItem>{moveCount}</RowItem>
      <ExpansionsItem ladybug={ladybug} mosquito={mosquito} pillbug={pillbug} />
    </Row>
  );
};

interface ListPlayerGamesProps extends HTMLAttributes<HTMLDivElement> {
  user: UserData;
  games: Game[];
}

const ListPlayerGames = (props: ListPlayerGamesProps) => {
  const { user, games, className, ...rest } = props;
  const sorted = games.sort(sortByLastPlayed);

  return (
    <div className={`grid grid-cols-4 w-full ${className || ''}`} {...rest}>
      <Header>
        <HeaderItem>
          <HiveIcon
            width={18}
            height={18}
            className='ml-3 mr-1.5 stroke-transparent fill-transparent'
          />
          Opponent
        </HeaderItem>
        <HeaderItem>Last Played</HeaderItem>
        <HeaderItem>Moves Played</HeaderItem>
        <HeaderItem>Expansions</HeaderItem>
      </Header>
      {sorted.map((game) => {
        return <PlayerGameRow key={game.gid} user={user} game={game} />;
      })}
    </div>
  );
};

export { ListPlayerGames };
