import {
  Button,
  Checkbox,
  CheckboxGroup,
  FormControl,
  FormLabel,
  Modal,
  ModalBody,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalOverlay,
  ModalProps,
  VStack
} from '@chakra-ui/react';
import { useRouter } from 'next/router';
import { useState } from 'react';
import {
  ColorChoice,
  ExpansionsChoice,
  OpeningChoice,
  createGameInvitation,
  usePlayer,
  UserData,
  VisibilityChoice
} from 'hive-db';
import { CardPicker } from '../forms/CardPicker';

interface InviteModalProps extends Omit<ModalProps, 'children'> {
  opponent?: UserData;
}

const InviteModal = (props: InviteModalProps) => {
  const { opponent, ...rest } = props;
  const router = useRouter();
  const { user } = usePlayer();

  const [color, setColor] = useState<ColorChoice>();
  const [visibility, setVisibility] = useState<VisibilityChoice>();
  const [opening, setOpening] = useState<OpeningChoice>();
  const [expansions, setExpansions] = useState<ExpansionsChoice>({
    ladybug: false,
    mosquito: false,
    pillbug: false
  });

  const [formDisabled, setFormDisabled] = useState(false);
  const submitDisabled =
    formDisabled ||
    user === null ||
    opponent === undefined ||
    color === undefined ||
    visibility === undefined ||
    opening === undefined;

  const handleSubmit = () => {
    setFormDisabled(true);
    if (user && opponent && visibility && color && opening) {
      createGameInvitation(
        user,
        opponent,
        visibility,
        color,
        expansions,
        opening
      )
        .then(() => {
          props.onClose();
          router.push('/games');
        })
        .catch((error) => {
          console.error(error);
          setFormDisabled(false);
        });
    }
  };

  return (
    <Modal {...rest}>
      <ModalOverlay />
      <ModalContent>
        <ModalHeader>
          {opponent ? (
            <>
              Invite <span className='text-hive'>{opponent.username}</span> to
              Play
            </>
          ) : (
            'New Invitation'
          )}
        </ModalHeader>
        {opponent && (
          <ModalBody>
            <form>
              <FormControl isDisabled={formDisabled} m={[2, 4]}>
                <FormLabel>Game Visibility</FormLabel>
                <CardPicker
                  pl={4}
                  name='gamevisibility'
                  options={['Public', 'Private']}
                  onChange={(visibility) => setVisibility(visibility)}
                />
              </FormControl>
              <FormControl isDisabled={formDisabled} m={[2, 4]}>
                <FormLabel>Your Tile Color</FormLabel>
                <CardPicker
                  pl={4}
                  name='playercolor'
                  options={['Black', 'White', 'Random']}
                  onChange={(color) => setColor(color)}
                />
              </FormControl>
              <FormControl isDisabled={formDisabled} m={[2, 4]} as='fieldset'>
                <FormLabel as='legend'>Expansions</FormLabel>
                <VStack align='start' pl={4}>
                  <CheckboxGroup colorScheme='teal'>
                    <Checkbox
                      value='pillbug'
                      checked={expansions.pillbug}
                      onChange={(e) =>
                        setExpansions((prevExpansions) => ({
                          ...prevExpansions,
                          pillbug: e.target.checked
                        }))
                      }
                    >
                      Pillbug
                    </Checkbox>
                    <Checkbox
                      value='ladybug'
                      checked={expansions.ladybug}
                      onChange={(e) =>
                        setExpansions((prevExpansions) => ({
                          ...prevExpansions,
                          ladybug: e.target.checked
                        }))
                      }
                    >
                      Ladybug
                    </Checkbox>
                    <Checkbox
                      value='mosquito'
                      checked={expansions.mosquito}
                      onChange={(e) =>
                        setExpansions((prevExpansions) => ({
                          ...prevExpansions,
                          mosquito: e.target.checked
                        }))
                      }
                    >
                      Mosquito
                    </Checkbox>
                  </CheckboxGroup>
                </VStack>
              </FormControl>
              <FormControl isDisabled={formDisabled} m={[2, 4]}>
                <FormLabel>Game Opening</FormLabel>
                <CardPicker
                  pl={4}
                  name='opening'
                  options={['Normal', 'Tournament']}
                  onChange={(opening) => setOpening(opening)}
                />
              </FormControl>
            </form>
          </ModalBody>
        )}
        <ModalFooter>
          <Button mr={3} colorScheme='teal' onClick={props.onClose}>
            Cancel
          </Button>
          <Button
            my={2}
            colorScheme='teal'
            disabled={submitDisabled}
            onClick={handleSubmit}
          >
            Send Invitation
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
};

export { InviteModal };
