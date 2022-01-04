import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';
import { getUsersExcept, usePlayer, UserData } from 'hive-db';
import { Body } from '../components/common/Body';
import { Footer } from '../components/common/Footer';
import { ListPublicGames } from '../components/lists/ListPublicGames';
import { ListUsers } from '../components/lists/ListUsers';
import { NavBar } from '../components/nav/NavBar';
import Head from 'next/head';

const Community = () => {
  const { uid, incompleteProfile } = usePlayer();
  const [users, setUsers] = useState<UserData[]>([]);
  const router = useRouter();

  useEffect(() => {
    if (uid) getUsersExcept(uid).then(setUsers);
    if (incompleteProfile) router.push('/profile');
  }, [uid, incompleteProfile, router]);

  return (
    <>
      <Head>
        <title>lihive.org • Free Online Hive</title>
      </Head>
      <NavBar />
      <Body className='my-12'>
        <div className='grid grid-cols-12 gap-4'>
          <div className='flex flex-col col-span-4'>
            <div className=' bg-slate-50 rounded p-4 pt-3'>
              <div className='shrink prose prose-xl mb-2 font-semibold'>
                Send an Invitation
              </div>
              <ListUsers users={users} />
            </div>
          </div>

          <div className='col-span-8 flex flex-col space-y-4'>
            <div className='bg-slate-50 rounded p-4 pt-3'>
              <div className='prose prose-xl mb-2 font-semibold'>
                Watch a Game
              </div>
              <ListPublicGames maxGames={10} />
            </div>

            <div className='bg-slate-50 rounded p-4 pt-3'>
              <div className='prose prose-xl mb-2 font-semibold'>Lobby</div>
              <div className='prose'>Coming Soon!</div>
            </div>
            <div className='bg-slate-50 rounded p-4 pt-3'>
              <div className='prose prose-xl mb-2 font-semibold'>
                Leaderboards
              </div>
              <div className='prose'>Coming Soon!</div>
            </div>
          </div>
        </div>
      </Body>
      <Footer />
    </>
  );
};

export default Community;
