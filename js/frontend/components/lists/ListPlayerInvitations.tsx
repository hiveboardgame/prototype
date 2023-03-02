import { Button, ButtonGroup } from '@chakra-ui/react';
import {
  acceptGameInvitation,
  Game,
  getIsLadybugUsed,
  getIsMosquitoUsed,
  getIsPillbugUsed,
  getIsTournamentRuleUsed,
  getOpponentColor,
  getOpponentUsername,
  getUserCreatedGame,
  rejectGameInvitation
} from 'hive-db';
import { HTMLAttributes } from 'react';
import { HiveIcon } from '../common/HiveIcon';
import { Header, HeaderItem } from './Header';
import { ExpansionsItem } from './items/ExpansionsItem';
import { PlayerItem } from './items/PlayerItem';
import { Row, RowItem } from './Row';

const CreatorButtons = ({ gid }: { gid: string }) => {
  return (
    <Button
      size='xs'
      colorScheme='teal'
      onClick={() => {
        rejectGameInvitation(gid).catch((error) => console.error(error));
      }}
    >
      Delete
    </Button>
  );
};

const InviteeButtons = ({ gid }: { gid: string }) => {
  return (
    <ButtonGroup size='xs' colorScheme='teal' isAttached>
      <Button
        onClick={() => {
          acceptGameInvitation(gid).catch((error) => console.error(error));
        }}
      >
        Accept
      </Button>
      <Button
        onClick={() => {
          rejectGameInvitation(gid).catch((error) => console.error(error));
        }}
      >
        Decline
      </Button>
    </ButtonGroup>
  );
};

interface PlayerInvitationRowProps {
  uid: string;
  game: Game;
}

const PlayerInvitationRow = ({ uid, game }: PlayerInvitationRowProps) => {
  const gid = game.gid;
  const opponent = getOpponentUsername(game, uid);
  const opponentColor = getOpponentColor(game, uid);
  const tournament = getIsTournamentRuleUsed(game);
  const ladybug = getIsLadybugUsed(game);
  const mosquito = getIsMosquitoUsed(game);
  const pillbug = getIsPillbugUsed(game);
  const didCreate = getUserCreatedGame(game, uid);
  return (
    <Row>
      <PlayerItem
        mode='color'
        playerName={opponent}
        playerColor={opponentColor}
        isTurn={false}
      />
      <RowItem>{tournament ? 'Tournament' : 'Normal'}</RowItem>
      <ExpansionsItem ladybug={ladybug} mosquito={mosquito} pillbug={pillbug} />
      <RowItem>
        {didCreate ? (
          <CreatorButtons gid={gid} />
        ) : (
          <InviteeButtons gid={gid} />
        )}
      </RowItem>
    </Row>
  );
};

interface ListPlayerInvitationsProps extends HTMLAttributes<HTMLDivElement> {
  uid: string;
  games: Game[];
}

const ListPlayerInvitations = (props: ListPlayerInvitationsProps) => {
  const { uid, games, className, ...rest } = props;

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
        <HeaderItem>Opening</HeaderItem>
        <HeaderItem>Expansions</HeaderItem>
        <HeaderItem />
      </Header>
      {games.map((game) => {
        return <PlayerInvitationRow key={game.gid} uid={uid} game={game} />;
      })}
    </div>
  );
};

export { ListPlayerInvitations };
