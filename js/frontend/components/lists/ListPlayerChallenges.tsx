import { Button, Input, Popover, PopoverArrow, PopoverBody, PopoverCloseButton, PopoverContent, PopoverHeader, PopoverTrigger, Spinner, useClipboard } from '@chakra-ui/react';
import {
  deleteGameChallenge,
  GameChallenge,
  usePlayerChallenges,
  usePlayer,
  useLobbyChallenges,
} from 'hive-db';
import { HTMLAttributes, useState } from 'react';
import { Header, HeaderItem } from './Header';
import { ExpansionsItem } from './items/ExpansionsItem';
import { Row, RowItem } from './Row';

const ShareLinkButton = ({ text }: { text: string }) => {
  const handleInputFocus = (event) => event.target.select();
  return (
    <Popover>
      <PopoverTrigger>
        <Button size='xs'>
          Share link
        </Button>
      </PopoverTrigger>
      <PopoverContent>
        <PopoverArrow />
        <PopoverCloseButton />
        <PopoverBody>
          <PopoverHeader>
            Send this link to a friend to invite them!
          </PopoverHeader>
          <Input
            type="text"
            size='sm'
            isReadOnly={true}
            onFocus={handleInputFocus}
            value={text}
          ></Input>
        </PopoverBody>
      </PopoverContent>
    </Popover>
  )
}

const DeleteButton = ({ id, onDelete }: { id: string, onDelete: () => void }) => {
  const { authToken } = usePlayer();
  return (
    <Button
      size='xs'
      colorScheme='red'
      onClick={() => {
        deleteGameChallenge(id, authToken)
          .then(onDelete)
          .catch((error) => console.error(error));
      }}
    >
      Delete
    </Button>
  );
};

interface PlayerChallengeRowProps {
  challenge: GameChallenge;
}

const PlayerChallengeRow = ({ challenge }: PlayerChallengeRowProps) => {
  const { mutate: mutatePlayerChallenges } = usePlayerChallenges();
  const { mutate: mutateLobbyChallenges } = useLobbyChallenges();
  const id = challenge.id;
  const isPublic = challenge.public;
  const isRanked = challenge.ranked;
  const tournament = challenge.tournamentQueenRule;
  const mosquito = challenge.gameType.mosquito;
  const ladybug = challenge.gameType.ladybug;
  const pillbug = challenge.gameType.pillbug;
  return (
    <Row>
      <RowItem>{isRanked ? 'Ranked' : 'Unranked'}</RowItem>
      <RowItem>{isPublic ? 'Public' : 'Private'}</RowItem>
      <RowItem>{tournament ? 'Tournament' : 'Normal'}</RowItem>
      <ExpansionsItem ladybug={ladybug} mosquito={mosquito} pillbug={pillbug} />
      <RowItem>{challenge.createdAt.toDateString()}</RowItem>
      <RowItem>
        <ShareLinkButton text={challenge.getChallengeUrl()} />
      </RowItem>
      <RowItem>
        <DeleteButton id={id} onDelete={() => {
          const removeChallenge = (challenges) => challenges.filter((other) => {
            return other.id == challenge.id;
          });
          mutatePlayerChallenges(removeChallenge);
          if (challenge.public) {
            mutateLobbyChallenges(removeChallenge);
          }
        }} />
      </RowItem>
    </Row>
  );
};

const ListPlayerChallenges = (props: HTMLAttributes<HTMLDivElement>) => {
  const { className, ...rest } = props;
  const { challenges, error, isLoading } = usePlayerChallenges();

  if (isLoading || error) {
    return <Spinner />
  }

  return (
    <div className={`grid grid-cols-7 w-full ${className || ''}`} {...rest}>
      <Header>
        <HeaderItem>Ranked</HeaderItem>
        <HeaderItem>Visibility</HeaderItem>
        <HeaderItem>Opening</HeaderItem>
        <HeaderItem>Expansions</HeaderItem>
        <HeaderItem>Date Created</HeaderItem>
        <HeaderItem />
        <HeaderItem />
      </Header>
      {challenges.map((challenge) => {
        return <PlayerChallengeRow key={challenge.id} challenge={challenge} />;
      })}
    </div>
  );
};

export { ListPlayerChallenges };
