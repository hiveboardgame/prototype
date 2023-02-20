import { FormSubmitButtonProps } from './FormSubmitButtonProps';
import { usePlayer } from 'hive-db';
import { Button } from '@chakra-ui/react';
import { FcGoogle } from 'react-icons/fc';
import { PropsWithChildren } from 'react';

const GoogleButton = (props: PropsWithChildren<FormSubmitButtonProps>) => {
  const { children, onPending, onSuccess, onFailure, disabled } = props;
  const { signInWithGoogle } = usePlayer();

  return (
    <Button
      onClick={() => {
        onPending();
        signInWithGoogle().then(onSuccess).catch(onFailure);
      }}
      isDisabled={disabled}
      variant='solid'
      size='md'
      width='100%'
      leftIcon={<FcGoogle />}
    >
      {children}
    </Button>
  );
};

export { GoogleButton };
