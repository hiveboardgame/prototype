export interface FormSubmitButtonProps {
  onPending: () => void;
  onSuccess: () => void;
  onFailure: (message?: string) => void;
  disabled?: boolean;
}
