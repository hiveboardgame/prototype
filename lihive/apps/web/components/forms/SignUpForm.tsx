import { useToast } from '@chakra-ui/react';
import { useRouter } from 'next/router';
import { useState } from 'react';
import { GoogleButton } from './GoogleButton';
import { SignUpEmailForm } from './SignUpEmailForm';

const SignUpForm = () => {
  const router = useRouter();
  const [isPending, setPending] = useState(false);
  const toast = useToast();

  const errorToast = (message: string) => {
    toast({
      title: 'Error',
      description: message,
      status: 'error',
      duration: 9000,
      isClosable: true
    });
  };

  const onPending = () => {
    setPending(true);
  };

  const onSuccess = () => {
    router.push('/profile');
  };

  return (
    <div className='divide-y divide-dashed'>
      <div className='pb-4'>
        <GoogleButton
          onPending={onPending}
          onFailure={() => {
            setPending(false);
            errorToast('Unable to sign up with Google');
          }}
          disabled={isPending}
          onSuccess={onSuccess}
        >
          Sign up with Google
        </GoogleButton>
      </div>
      <div className='pt-4'>
        <SignUpEmailForm
          setPending={setPending}
          disabled={isPending}
          onSuccess={onSuccess}
        />
      </div>
    </div>
  );
};

export { SignUpForm };
