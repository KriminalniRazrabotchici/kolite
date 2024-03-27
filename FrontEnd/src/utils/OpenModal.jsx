import { useDispatch, useSelector } from 'react-redux';

import Modal from '../ui/Modal';
import Login from '../features/login/LoginForm';
import Register from '../features/register/RegisterForm';

import { close } from '../slices/ModalSlice';
import { hideLogin, hideRegister } from '../slices/LoginRegisterSlice';

function OpenModal() {
  const isOpenModal = useSelector((state) => state.modal.isOpen);

  const isLogin = useSelector((state) => state.logReg.isLoginForm);
  const isRegister = useSelector((state) => state.logReg.isRegisterForm);
  const dispatch = useDispatch();

  function handleCloseLoginRegister() {
    dispatch(close());
    dispatch(hideLogin());
    dispatch(hideRegister());
  }

  return (
    <div>
      {isOpenModal && (
        <Modal onClose={handleCloseLoginRegister}>
          {isLogin && <Login onCloseModal={handleCloseLoginRegister} />}
          {isRegister && <Register onCloseModal={handleCloseLoginRegister} />}
        </Modal>
      )}
    </div>
  );
}

export default OpenModal;
