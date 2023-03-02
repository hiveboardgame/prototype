import { usePlayer } from 'hive-db';
import Head from 'next/head';
import { useRouter } from 'next/router';
import { useEffect } from 'react';
import { Body } from '../components/common/Body';
import { Footer } from '../components/common/Footer';
import { ListPlayerCompleted } from '../components/lists/ListPlayerCompleted';
import { ListPlayerGames } from '../components/lists/ListPlayerGames';
import { NavBar } from '../components/nav/NavBar';
import { useTitle } from '../hooks/useTitle';

const None = () => {
  return <div className='px-8 py-4 prose'>None yet.</div>;
};

const Games = () => {
  const { user, incompleteProfile, activeGames, completedGames } = usePlayer();
  const title = useTitle();
  const router = useRouter();

  useEffect(() => {
    if (incompleteProfile) router.push('/profile');
  }, [incompleteProfile, router]);

  if (!user) {
    return null;
  }

  return (
    <>
      <Head>
        <title>{title}</title>
      </Head>
      <NavBar />
      <Body className='my-12'>
        <div className='prose prose-xl mb-2 font-semibold'>Active Games</div>
        <div className='bg-slate-50 rounded'>
          <ListPlayerGames user={user} games={activeGames} />
        </div>
        <div className='prose prose-xl mt-8 mb-2 font-semibold'>
          Completed Games
        </div>
        <div className='bg-slate-50 rounded'>
          {completedGames.length > 0 ? (
            <ListPlayerCompleted user={user} games={completedGames} />
          ) : (
            <None />
          )}
        </div>
      </Body>
      <Footer />
    </>
  );
};

export default Games;
