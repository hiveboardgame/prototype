import {
  Button,
  Checkbox,
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
  createGameChallenge,
  usePlayer,
  VisibilityChoice,
  GameChallenge,
  Game
} from 'hive-db';
import { CardPicker } from '../forms/CardPicker';
import { usePlayerChallenges } from '../../hooks/usePlayerChallenges';

const NewGameModal = (props: Omit<ModalProps, 'children'>) => {
  const DEFAULT_COLOR_CHOICE: ColorChoice = "Random";
  const DEFAULT_VISIBILITY_CHOICE: VisibilityChoice = "Private";

  const router = useRouter();
  const { user, authToken } = usePlayer();
  const { mutate } = usePlayerChallenges();

  const [color, setColor] = useState<ColorChoice>(DEFAULT_COLOR_CHOICE);
  const [visibility, setVisibility] = useState<VisibilityChoice>(DEFAULT_VISIBILITY_CHOICE);
  const [expansions, setExpansions] = useState<ExpansionsChoice>({
    ladybug: true,
    mosquito: true,
    pillbug: true
  });

  const [formDisabled, setFormDisabled] = useState(false);
  const submitDisabled =
    formDisabled ||
    user === null ||
    color === undefined ||
    visibility === undefined;

  const handleSubmit = () => {
    setFormDisabled(true);
    if (user && visibility && color) {
      createGameChallenge(
        visibility,
        color,
        expansions,
        authToken,
      ).then((challenge: GameChallenge) => {
        // immediately update the local view of the challenge list
        mutate(async (challenges: GameChallenge[]) => challenges.concat([challenge]));
        props.onClose();
      })
      .catch((error) => console.error(error))
      .finally(() => setFormDisabled(false));
    }
  };

  return (
    <Modal {...props}>
      <ModalOverlay />
      <ModalContent>
        <ModalHeader>
          New Game Challenge
        </ModalHeader>
          <ModalBody>
            <form>
              <FormControl isDisabled={formDisabled} m={[2, 4]}>
                <FormLabel>Game Visibility</FormLabel>
                <CardPicker
                  pl={4}
                  name='gamevisibility'
                  options={['Public', 'Private']}
                  defaultValue={DEFAULT_VISIBILITY_CHOICE}
                  onChange={(visibility) => setVisibility(visibility)}
                />
              </FormControl>
              <FormControl isDisabled={formDisabled} m={[2, 4]}>
                <FormLabel>Your Tile Color</FormLabel>
                <CardPicker
                  pl={4}
                  name='playercolor'
                  options={['Black', 'White', 'Random']}
                  defaultValue={DEFAULT_COLOR_CHOICE}
                  onChange={(color) => setColor(color)}
                />
              </FormControl>
              <FormControl isDisabled={formDisabled} m={[2, 4]} as='fieldset'>
                <FormLabel as='legend'>Expansions</FormLabel>
                <VStack align='start' pl={4}>
                  <Checkbox
                    value='pillbug'
                    isChecked={expansions.pillbug}
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
                    isChecked={expansions.ladybug}
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
                    isChecked={expansions.mosquito}
                    onChange={(e) =>
                      setExpansions((prevExpansions) => ({
                        ...prevExpansions,
                        mosquito: e.target.checked
                      }))
                    }
                  >
                    Mosquito
                  </Checkbox>
                </VStack>
              </FormControl>
            </form>
          </ModalBody>
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
            Create Game
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
};

export { NewGameModal };
