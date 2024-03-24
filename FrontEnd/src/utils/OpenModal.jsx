import { useDispatch, useSelector } from 'react-redux';

import Modal from '../ui/Modal';
import Login from '../features/login/LoginForm';
import Register from '../features/register/RegisterForm';

import { close } from '../slices/ModalSlice';
import { hideLogin } from '../slices/LoginRegisterSlice';
import { hideRegister } from '../slices/LoginRegisterSlice';

function OpenModal() {
  const isOpenModal = useSelector((state) => state.modal.isOpen);

  const isLogin = useSelector((state) => state.logReg.isLoginForm);
  const isRegister = useSelector((state) => state.logReg.isRegisterForm);
  const dispatch = useDispatch();

  function handleClose() {
    dispatch(close());
    dispatch(hideLogin());
    dispatch(hideRegister());
  }

  return (
    <div>
      {isOpenModal && (
        <Modal onClose={handleClose}>
          {isLogin && <Login onCloseModal={handleClose} />}
          {isRegister && <Register onCloseModal={handleClose} />}
        </Modal>
      )}
    </div>
  );
}

export default OpenModal;
