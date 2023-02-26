import { FormSubmitButtonProps } from './FormSubmitButtonProps';
import { usePlayer } from 'hive-db';
import { Button } from '@chakra-ui/react';
import { PropsWithChildren } from 'react';

const GuestButton = (props: PropsWithChildren<FormSubmitButtonProps>) => {
  const { children, onPending, onSuccess, onFailure, disabled } = props;
  const { signInAsGuest } = usePlayer();

  return (
    <Button
      onClick={() => {
        onPending();
        signInAsGuest().then(onSuccess).catch(onFailure);
      }}
      isDisabled={disabled}
      variant='solid'
      size='md'
      width='100%'
    >
      {children}
    </Button>
  );
};

export { GuestButton };
