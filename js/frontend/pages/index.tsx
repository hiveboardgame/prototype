import {
  Button,
  Icon,
  Stack,
  Tab,
  TabList,
  TabPanel,
  TabPanels,
  Tabs
} from '@chakra-ui/react';
import { usePlayer } from 'hive-db';
import Head from 'next/head';
import { useRouter } from 'next/router';
import { FaGamepad, FaPlus } from 'react-icons/fa';
import { Body } from '../components/common/Body';
import { Footer } from '../components/common/Footer';
import { ListLobbyGames } from '../components/lists/ListLobbyGames';
import { ListPlayerGames } from '../components/lists/ListPlayerGames';
import { ListPublicGames } from '../components/lists/ListPublicGames';
import { NavBar } from '../components/nav/NavBar';
import { useHasMounted } from '../hooks/useHasMounted';
import { useTitle } from '../hooks/useTitle';

const IndexTabs = () => {
  const { user, incompleteProfile, activeGames } = usePlayer();
  const showOwnGames = user && !incompleteProfile;
  return (
    <Tabs
      id='index-tabs'
      isFitted
      variant='enclosed-colored'
      colorScheme='teal'
      size='sm'
    >
      <TabList>
        <Tab>Watch</Tab>
        <Tab>Lobby</Tab>
        {showOwnGames && <Tab>Your Games</Tab>}
      </TabList>
      <TabPanels w='full'>
        <TabPanel p={0}>
          <ListPublicGames className='border' maxGames={5} />
        </TabPanel>
        <TabPanel p={0}>
          <ListLobbyGames className='border' />
        </TabPanel>
        {showOwnGames && (
          <TabPanel p={0}>
            <ListPlayerGames className='border' user={user} games={activeGames} />
          </TabPanel>
        )}
      </TabPanels>
    </Tabs>
  );
};

const Index = () => {
  const { user, incompleteProfile } = usePlayer();
  const title = useTitle();
  const router = useRouter();
  const mounted = useHasMounted();
  const loggedIn = mounted && !!user && !incompleteProfile;
  return (
    <>
      <Head>
        <title>{title}</title>
      </Head>
      <NavBar />
      <div className='bg-slate-50 mb-16'>
        {!loggedIn && (
          <div className='prose mx-auto my-16'>
            <div className='prose prose-xl mb-2 font-semibold'>
              Welcome Beta Testers!
            </div>
          </div>
        )}
      </div>
      <Body>
        <div className='grid gap-6 grid-cols-12'>
          <div className='col-span-9'>
            <IndexTabs />
          </div>
          <div className='col-span-3'>
            <Stack>
              <Button
                leftIcon={<Icon as={FaPlus} w={4} h={4} />}
                iconSpacing={3}
                colorScheme='teal'
                size='md'
                disabled={!loggedIn}
                onClick={() => router.push('/community')}
              >
                Create New Game
              </Button>
              <Button
                leftIcon={<Icon as={FaGamepad} w={5} h={5} />}
                iconSpacing={3}
                colorScheme='teal'
                size='md'
                disabled={true}
              >
                Play Offline
              </Button>
            </Stack>
          </div>
        </div>
      </Body>
      <Footer />
    </>
  );
};

export default Index;
