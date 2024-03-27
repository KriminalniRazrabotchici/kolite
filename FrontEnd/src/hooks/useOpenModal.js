import { useDispatch } from 'react-redux';
import { open } from '../slices/ModalSlice';
import { showLogin } from '../slices/LoginRegisterSlice';
import { showRegister } from '../slices/LoginRegisterSlice';

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
