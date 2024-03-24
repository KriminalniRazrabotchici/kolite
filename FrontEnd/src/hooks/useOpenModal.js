import { useDispatch } from 'react-redux';
import { showLogin } from '../slices/LoginRegisterSlice';
import { showRegister } from '../slices/LoginRegisterSlice';
import { open } from '../slices/ModalSlice';

function useOpenModal() {
  const dispatch = useDispatch();

  function handleLoginButton() {
    dispatch(open());
    dispatch(showLogin());
  }

  function handleRegisterButton() {
    dispatch(open());
    dispatch(showRegister());
  }

  return {
    handleLoginButton,
    handleRegisterButton,
  };
}

export default useOpenModal;
