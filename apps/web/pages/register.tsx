import { usePlayer } from 'hive-db';
import Head from 'next/head';
import { useRouter } from 'next/router';
import { SignUpForm } from '../components/forms/SignUpForm';
import { NavBar } from '../components/nav/NavBar';
import { useTitle } from '../hooks/useTitle';

const Register = () => {
  const { uid, incompleteProfile } = usePlayer();
  const router = useRouter();
  const title = useTitle();

  if (uid !== null && incompleteProfile) router.push('/profile');
  if (uid !== null && !incompleteProfile) router.push('/');

  return (
    <>
      <Head>
        <title>{title}</title>
      </Head>
      <NavBar />
      <div className='prose mx-auto my-16'>
        <SignUpForm />
      </div>
    </>
  );
};

export default Register;
