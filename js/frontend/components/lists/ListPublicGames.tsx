import { HTMLAttributes, useState } from 'react';
import {
  Game,
  getBlackUsername,
  getColorTurn,
  getIsLadybugUsed,
  getIsMosquitoUsed,
  getIsPillbugUsed,
  getLastPlayDate,
  getMoveCount,
  getWhiteUsername,
} from 'hive-db';
import { HiveIcon } from '../common/HiveIcon';
import { Header, HeaderItem } from './Header';
import { Row, RowItem } from './Row';
import { useRouter } from 'next/router';
import { PlayerItem } from './items/PlayerItem';
import { ElapsedTimeItem } from './items/ElapsedTimeItem';
import { ExpansionsItem } from './items/ExpansionsItem';

const PublicGameRow = ({ game }: { game: Game }) => {
  const router = useRouter();
  const black = getBlackUsername(game);
  const white = getWhiteUsername(game);
  const turn = getColorTurn(game);
  const updated = getLastPlayDate(game);
  const moveCount = getMoveCount(game);
  const ladybug = getIsLadybugUsed(game);
  const mosquito = getIsMosquitoUsed(game);
  const pillbug = getIsPillbugUsed(game);
  return (
    <Row onClick={() => router.push(`/game/${game.gid}`)}>
      <PlayerItem
        mode='color'
        playerName={black}
        playerColor='b'
        isTurn={turn === 'b'}
      />
      <PlayerItem
        mode='color'
        playerName={white}
        playerColor='w'
        isTurn={turn === 'w'}
      />
      <ElapsedTimeItem since={updated} placeholder='-' />
      <RowItem>{moveCount}</RowItem>
      <ExpansionsItem ladybug={ladybug} mosquito={mosquito} pillbug={pillbug} />
    </Row>
  );
};

interface ListPublicGamesProps extends HTMLAttributes<HTMLDivElement> {
  maxGames: number;
}

const ListPublicGames = (props: ListPublicGamesProps) => {
  const { maxGames, className, ...rest } = props;
  const [games, setGames] = useState<Game[]>([]);
  const [error, setError] = useState<Error>();

  if (error) return null;

  return (
    <div
      className={`grid w-full grid-cols-[repeat(2,minmax(max-content,auto))_repeat(3,minmax(0,auto))] place-content-stretch ${
        className || ''
      }`}
      {...rest}
    >
      <Header>
        <HeaderItem>
          <HiveIcon
            width={18}
            height={18}
            className='ml-1 mr-1.5 stroke-hiveblack fill-hiveblack'
          />
          Black
        </HeaderItem>
        <HeaderItem>
          <HiveIcon
            width={18}
            height={18}
            className='ml-1 mr-1.5 stroke-hiveblack fill-hivewhite'
          />
          White
        </HeaderItem>
        <HeaderItem>Last Played</HeaderItem>
        <HeaderItem>Moves</HeaderItem>
        <HeaderItem>Expansions</HeaderItem>
      </Header>
      {games.map((game) => {
        return <PublicGameRow key={game.gid} game={game} />;
      })}
    </div>
  );
};

export { ListPublicGames };
