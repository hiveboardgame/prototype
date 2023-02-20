import * as Yup from 'yup';
import {
  Form,
  Formik,
  FormikHelpers,
  useField,
  useFormikContext
} from 'formik';
import {
  Box,
  Button,
  FormControl,
  FormErrorMessage,
  FormLabel,
  Input,
  Stack
} from '@chakra-ui/react';
import { useRouter } from 'next/router';
import { updateUsername, getUsernameAvailable } from 'hive-db';

interface UsernameFormValues {
  username: string;
}

const usernameValidationSchema = Yup.object({
  username: Yup.string()
    .min(3, 'Username must be at least 3 characters')
    .max(20, 'Username must be 20 characters or less')
    .required('Required')
});

const UsernameInput = ({
  label,
  placeholder
}: {
  label: string;
  placeholder?: string;
}) => {
  const [field, meta] = useField('username');
  const { isSubmitting } = useFormikContext();
  return (
    <FormControl
      isDisabled={isSubmitting}
      isInvalid={meta.touched && !!meta.error}
    >
      <FormLabel htmlFor={field.name}>{label}</FormLabel>
      <Input
        {...field}
        id={field.name}
        bg='white'
        placeholder={placeholder || 'Username'}
      />
      {meta.touched && meta.error && (
        <FormErrorMessage>{meta.error}</FormErrorMessage>
      )}
    </FormControl>
  );
};

const SubmitButton = () => {
  const { isSubmitting } = useFormikContext();
  return (
    <Button
      isLoading={isSubmitting}
      isDisabled={isSubmitting}
      type='submit'
      colorScheme='teal'
    >
      Submit
    </Button>
  );
};

const FinishProfileForm = ({ uid }: { uid: string }) => {
  const initialValues: UsernameFormValues = { username: '' };
  const router = useRouter();
  const handleSubmit = (
    values: UsernameFormValues,
    helpers: FormikHelpers<UsernameFormValues>
  ) => {
    helpers.setSubmitting(true);
    getUsernameAvailable(values.username)
      .then((available) => {
        if (available) {
          updateUsername(uid, values.username).then(() => router.push('/'));
        } else {
          helpers.setSubmitting(false);
          helpers.setFieldError('username', 'Username is taken');
        }
      })
      .catch((err) => {
        helpers.setSubmitting(false);
        helpers.setFieldError('username', 'Error checking username');
      });
  };
  return (
    <Formik
      initialValues={initialValues}
      validationSchema={usernameValidationSchema}
      onSubmit={handleSubmit}
    >
      <Form>
        <Stack>
          <UsernameInput label='Pick a username to finish setting up your profile:' />
          <Box>
            <SubmitButton />
          </Box>
        </Stack>
      </Form>
    </Formik>
  );
};

export { FinishProfileForm };
