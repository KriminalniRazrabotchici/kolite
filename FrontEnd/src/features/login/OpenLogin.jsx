import { useDispatch, useSelector } from 'react-redux';

import { ButtonLink } from '../../ui/ButtonNav';
import Modal from '../../ui/Modal';
import Login from './LoginForm';
import Register from '../register/RegisterForm';

import { close, open } from '../../slices/ModalSlice';
import { hideLogin, showLogin } from '../../slices/LoginSlice';
import { hideRegister } from '../../slices/RegisterSlice';

function OpenLogin() {
  const isOpenModal = useSelector((state) => state.modal.isOpen);

  const isLogin = useSelector((state) => state.login.isLoginForm);
  const dispatch = useDispatch();

  function handleLoginButton() {
    dispatch(open());
    dispatch(showLogin());
    dispatch(hideRegister());
  }

  function handleClose() {
    dispatch(close());
    dispatch(hideLogin());
    dispatch(hideRegister());
  }

  return (
    <div>
      <ButtonLink onClick={handleLoginButton}>Login</ButtonLink>
      {isOpenModal && (
        <Modal onClose={handleClose}>
          {isLogin ? (
            <Login onCloseModal={handleClose} />
          ) : (
            <Register onCloseModal={handleClose} />
          )}
        </Modal>
      )}
    </div>
  );
}

export default OpenLogin;
