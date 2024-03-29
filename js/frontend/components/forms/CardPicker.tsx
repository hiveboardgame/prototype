import { HStack, StackProps, useRadioGroup } from '@chakra-ui/react';
import { RadioCard } from './RadioCard';

type CardPickerProps<T> = Omit<StackProps, 'onChange'> & {
  name: string;
  options: T[];
  defaultValue?: T;
  onChange: (nextValue: T) => void;
};

const CardPicker = <T extends string>(props: CardPickerProps<T>) => {
  const { name, options, onChange, defaultValue, ...rest } = props;
  const { getRootProps, getRadioProps } = useRadioGroup({
    name,
    defaultValue,
    onChange,
  });
  const group = getRootProps();
  return (
    <HStack {...rest} {...group}>
      {options.map((value) => {
        const radio = getRadioProps({ value });
        return (
          <RadioCard key={value} {...radio}>
            {value}
          </RadioCard>
        );
      })}
    </HStack>
  );
};

export { CardPicker };
