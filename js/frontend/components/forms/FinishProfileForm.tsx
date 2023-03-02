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

interface FinishProfileFormProps {
  usernameChanged: (username: string) => Promise<void>;
}

function FinishProfileForm(props: FinishProfileFormProps) {
  const { usernameChanged } = props;
  const initialValues: UsernameFormValues = { username: '' };
  const router = useRouter();
  const handleSubmit = (
    values: UsernameFormValues,
    helpers: FormikHelpers<UsernameFormValues>
  ) => {
    helpers.setSubmitting(true);
    usernameChanged(values.username).then(() => router.push('/'))
      .catch((err) => {
        console.log(`failed to create user with username "${values.username}": ${err}`);
        helpers.setSubmitting(false);
        helpers.setFieldError('username', 'Error setting username');
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
}

export { FinishProfileForm };
