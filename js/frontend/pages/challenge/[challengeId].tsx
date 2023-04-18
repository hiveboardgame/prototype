import Head from 'next/head';
import { useRouter } from 'next/router';
import { acceptGameChallenge, ExpansionsChoice, GameChallenge, getGameChallenge, usePlayer, UserData } from 'hive-db';
import { NavBar } from '../../components/nav/NavBar';
import { useTitle } from '../../hooks/useTitle';
import { useEffect, useState } from 'react';
import { Body } from '../../components/common/Body';
import { Footer } from '../../components/common/Footer';
import { Button, List, ListIcon, ListItem } from '@chakra-ui/react';
import { MdCalendarToday, MdCircle, MdOutlineContrast, MdLooksOne, MdOutlineBugReport, MdOutlineCircle, MdOutlineEmojiEvents } from 'react-icons/md';

function getExpansionDescription(gameType: ExpansionsChoice): string {
  const expansionPieces = []
  if (gameType.mosquito) {
    expansionPieces.push('Mosquito');
  }
  if (gameType.ladybug) {
    expansionPieces.push('Ladybug');
  }
  if (gameType.pillbug) {
    expansionPieces.push('Pillbug');
  }
  if (expansionPieces.length === 0) {
    return 'Base game';
  } else {
    return expansionPieces.join(', ');
  }
}

const ColorChoiceItem = ({colorChoice}: {colorChoice: string}) => {
  switch (colorChoice) {
    case 'White':
      return <><ListIcon as={MdOutlineCircle} />{'Challenger is white'}</>;
    case 'Black':
      return <><ListIcon as={MdCircle} />{'Challenger is black'}</>;
    case 'Random':
      return <><ListIcon as={MdOutlineContrast} />{'Random color choice'}</>;
  }
}

interface ChallengeInfoBoxProps {
  challenge: GameChallenge,
}

const ChallengeInfoBox = ({ challenge }: ChallengeInfoBoxProps) => {
  const { ranked, tournamentQueenRule, gameType, colorChoice, createdAt } = challenge;
  return (
    <>
      <List>
        <ListItem>
          <ListIcon as={MdOutlineEmojiEvents} />
          { ranked ? 'Ranked' : 'Unranked' }
        </ListItem>
        <ListItem>
          <ListIcon as={MdLooksOne}/>
          { tournamentQueenRule ? 'Tournament opening' : 'Unrestricted opening' }
        </ListItem>
        <ListItem>
          <ListIcon as={MdOutlineBugReport} />
          { getExpansionDescription(gameType) }
        </ListItem>
        <ListItem>
          <ListIcon as={MdCalendarToday} />
          { `Created ${createdAt.toLocaleDateString()}` }
        </ListItem>
        <ListItem>
          <ColorChoiceItem colorChoice={colorChoice}></ColorChoiceItem>
        </ListItem>
      </List>
    </>
  )
}

interface AcceptChallengeProps {
  challenge: GameChallenge,
  user?: UserData,
}

const AcceptChallenge = (props: AcceptChallengeProps) => {
  const { challenge, user } = props;
  const { authToken } = usePlayer();

  return (
    <>
      <div className='bg-slate-50 rounded p-4 pt-3'>
        <div className='prose prose-xl mb-2 font-semibold'>
          {`${challenge.challenger.username} has challenged you to a game`}
        </div>
        <ChallengeInfoBox challenge={challenge}></ChallengeInfoBox>
        { !user ? 'In order to accept this invite, please log in first.' :
          <Button
            colorScheme="green"
            size="lg"
            onClick={() => {
              acceptGameChallenge(challenge.id, authToken)
                .then((game) => console.log(game)) // TODO: navigate to the newly created game
                .catch((err) => console.error(err));
            }}
          >
            Accept Challenge
          </Button>
        }
      </div>
    </>
  );
}

const Challenge = () => {
  const router = useRouter();
  const { user } = usePlayer();
  const { challengeId } = router.query;
  const [challenge, setChallenge] = useState<GameChallenge | null>();
  const title = useTitle();

  useEffect(() => {
    if (!challengeId) return;

    getGameChallenge(challengeId as string)
      .then((challenge) => setChallenge(challenge));
  }, [challengeId]);

  return (
    <>
      <Head>
        <title>{title}</title>
      </Head>
      <NavBar fullWidth className='border-b' />
      <Body className='my-12'>
        <div className='grid grid-cols-12 gap-4'>
          <div className='col-span-8 flex flex-col space-y-4'>
            { !challenge ? 'Loading...' : <AcceptChallenge challenge={challenge} user={user}></AcceptChallenge>}
          </div>
        </div>
      </Body>
      <Footer />
    </>
  );
};

export default Challenge;
