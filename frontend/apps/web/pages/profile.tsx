import { usePlayer } from 'hive-db';
import Head from 'next/head';
import { useRouter } from 'next/router';
import { useEffect } from 'react';
import { FinishProfileForm } from '../components/forms/FinishProfileForm';
import { NavBar } from '../components/nav/NavBar';
import { useTitle } from '../hooks/useTitle';

const Profile = () => {
  const { uid, incompleteProfile } = usePlayer();
  const title = useTitle();
  const router = useRouter();

  useEffect(() => {
    if (!incompleteProfile) router.push('/');
  }, [incompleteProfile, router]);

  return (
    <>
      <Head>
        <title>{title}</title>
      </Head>
      <NavBar hideFinishProfile />
      <div className='prose mx-auto my-16'>
        {uid && incompleteProfile && <FinishProfileForm uid={uid} />}
      </div>
    </>
  );
};

export default Profile;
