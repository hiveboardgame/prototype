import { useRouter } from 'next/router';
import {
  Game,
  getGameResult,
  getIsLadybugUsed,
  getIsMosquitoUsed,
  getIsPillbugUsed,
  getMoveCount,
  getOpponentColor,
  getOpponentUsername,
  sortByLastPlayed,
  UserData
} from 'hive-db';
import { HTMLAttributes } from 'react';
import { HiveIcon } from '../common/HiveIcon';
import { Header, HeaderItem } from './Header';
import { ExpansionsItem } from './items/ExpansionsItem';
import { PlayerItem } from './items/PlayerItem';
import { Row, RowItem } from './Row';

interface PlayerCompletedRowProps {
  user: UserData;
  game: Game;
}

const PlayerCompletedRow = ({ user, game }: PlayerCompletedRowProps) => {
  const router = useRouter();
  const uid = user.uid;
  const opponent = getOpponentUsername(game, uid);
  const opponentColor = getOpponentColor(game, uid);
  const result = getGameResult(game);
  const ladybug = getIsLadybugUsed(game);
  const mosquito = getIsMosquitoUsed(game);
  const pillbug = getIsPillbugUsed(game);
  const moveCount = getMoveCount(game);
  return (
    <Row onClick={() => router.push(`/game/${game.gid}`)}>
      <PlayerItem
        mode='color'
        playerName={opponent}
        playerColor={opponentColor}
        isTurn={false}
      />
      <RowItem>{resultString(uid, result)}</RowItem>
      <RowItem>{moveCount}</RowItem>
      <ExpansionsItem ladybug={ladybug} mosquito={mosquito} pillbug={pillbug} />
    </Row>
  );
};

interface ListPlayerCompletedProps extends HTMLAttributes<HTMLDivElement> {
  user: UserData;
  games: Game[];
}

const ListPlayerCompleted = (props: ListPlayerCompletedProps) => {
  const { user, games, className, ...rest } = props;
  const sorted = games.sort(sortByLastPlayed);

  return (
    <div className={`grid grid-cols-4 w-full ${className || ''}`} {...rest}>
      <Header>
        <HeaderItem>
          <HiveIcon
            className='ml-1 mr-1.5 stroke-transparent fill-transparent'
            width={18}
            height={18}
          />
          Opponent
        </HeaderItem>
        <HeaderItem>Result</HeaderItem>
        <HeaderItem>Moves Played</HeaderItem>
        <HeaderItem>Expansions</HeaderItem>
      </Header>
      {sorted.map((game) => {
        return <PlayerCompletedRow key={game.gid} user={user} game={game} />;
      })}
    </div>
  );
};

function resultString(uid: string, result: string): string {
  if (result === 'draw') return 'Draw';
  if (result === 'tie') return 'Tie';
  return uid === result ? 'Win' : 'Loss';
}

export { ListPlayerCompleted };
