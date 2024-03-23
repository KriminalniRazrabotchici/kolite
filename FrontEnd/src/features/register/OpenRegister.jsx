import { useDispatch, useSelector } from 'react-redux';

import { ButtonLink } from '../../ui/ButtonNav';
import Modal from '../../ui/Modal';
import Register from './RegisterForm';
import Login from '../login/LoginForm';

import { open, close } from '../../slices/ModalSlice';
import { showRegister, hideRegister } from '../../slices/RegisterSlice';
import { hideLogin, showLogin } from '../../slices/LoginSlice';

function OpenRegister() {
  const isOpenModal = useSelector((state) => state.modal.isOpen);
  const isRegister = useSelector((state) => state.register.isRegisterForm);
  const dispatch = useDispatch();

  function handleRegisterButton() {
    dispatch(open());
    dispatch(showRegister());
    dispatch(hideLogin());
  }

  function handleClose() {
    dispatch(close());
    dispatch(hideLogin());
    dispatch(hideRegister());
  }

  return (
    <div>
      <ButtonLink onClick={handleRegisterButton}>Register</ButtonLink>
      {isOpenModal && (
        <Modal onClose={handleClose}>
          {isRegister ? (
            <Register onCloseModal={handleClose} />
          ) : (
            <Login onCloseModal={handleClose} />
          )}
        </Modal>
      )}
    </div>
  );
}

export default OpenRegister;
