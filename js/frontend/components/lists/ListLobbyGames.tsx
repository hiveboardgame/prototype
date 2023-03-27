import { HTMLAttributes } from 'react';
import {
  acceptGameChallenge,
  GameChallenge,
  useLobbyChallenges,
  usePlayer,
} from 'hive-db';
import { Button, Spinner } from '@chakra-ui/react';
import { Header, HeaderItem } from './Header';
import { ExpansionsItem } from './items/ExpansionsItem';
import { Row, RowItem } from './Row';

interface LobbyChallengeRowProps {
  challenge: GameChallenge;
}

const LobbyChallengeRow = ({ challenge }: LobbyChallengeRowProps) => {
  const { user, authToken } = usePlayer();
  const isRanked = challenge.ranked;
  const tournament = challenge.tournamentQueenRule;
  const mosquito = challenge.gameType.mosquito;
  const ladybug = challenge.gameType.ladybug;
  const pillbug = challenge.gameType.pillbug;
  const canAccept = user && user.uid !== challenge.challenger.uid;
  return (
    <Row>
      <RowItem>{challenge.challenger.username}</RowItem>
      <RowItem>{isRanked ? 'Ranked' : 'Unranked'}</RowItem>
      <RowItem>{tournament ? 'Tournament' : 'Normal'}</RowItem>
      <ExpansionsItem ladybug={ladybug} mosquito={mosquito} pillbug={pillbug} />
      <RowItem>{challenge.createdAt.toDateString()}</RowItem>
      <RowItem>
        <Button
          colorScheme="green"
          disabled={!canAccept}
          size="sm"
          onClick={() => {
            acceptGameChallenge(challenge.id, authToken)
              .then((game) => console.log(game)) // TODO: navigate to the newly created game
              .catch((err) => console.error(err));
          }}
        >
          Accept
        </Button>
      </RowItem>
    </Row>
  );
};

interface ListLobbyGamesProps extends HTMLAttributes<HTMLDivElement> {}

const ListLobbyGames = (props: ListLobbyGamesProps) => {
  const { className, ...rest } = props;
  const { data: challenges, isLoading, error } = useLobbyChallenges();

  if (isLoading || error) {
    return <Spinner />
  }

  return (
    <div className={`grid grid-cols-6 w-full ${className || ''}`} {...rest}>
      <Header>
        <HeaderItem>User</HeaderItem>
        <HeaderItem>Ranked</HeaderItem>
        <HeaderItem>Opening</HeaderItem>
        <HeaderItem>Expansions</HeaderItem>
        <HeaderItem>Date Created</HeaderItem>
        <HeaderItem />
      </Header>
      {challenges.map((challenge) => {
        return <LobbyChallengeRow key={challenge.id} challenge={challenge} />;
      })}
    </div>
  );
};

export { ListLobbyGames };
