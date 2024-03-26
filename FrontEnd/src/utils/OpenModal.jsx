import { useDispatch, useSelector } from 'react-redux';

import Modal from '../ui/Modal';
import Login from '../features/login/LoginForm';
import Register from '../features/register/RegisterForm';

import { close } from '../slices/ModalSlice';
import { hideLogin, hideRegister } from '../slices/LoginRegisterSlice';

import {
  showCoupe,
  showBrand,
  showFuel,
  showTransmission,
  showPrice,
  showYear,
  showCity,
  showColor,
  showHP,
  showDoors,
  showExtras,
  showWheel,
  hideCoupe,
  hideBrand,
  hideFuel,
  hideTransmission,
  hidePrice,
  hideYear,
  hideCity,
  hideColor,
  hideHP,
  hideDoors,
  hideExtras,
  hideWheel,
} from '../slices/SearchButtonsSlice';
import SearchBy from '../ui/SearchBy';
import OpenButtons from './OpenButtons';

function OpenModal() {
  const isOpenModal = useSelector((state) => state.modal.isOpen);

  const {
    isCoupe,
    isBrand,
    isFuel,
    isTransmission,
    isPrice,
    isYear,
    isCity,
    isColor,
    isHP,
    isDoors,
    isExtras,
    isWheel,
  } = useSelector((state) => state.buttons);

  const isLogin = useSelector((state) => state.logReg.isLoginForm);
  const isRegister = useSelector((state) => state.logReg.isRegisterForm);
  const dispatch = useDispatch();

  function handleCloseLoginRegister() {
    dispatch(close());
    dispatch(hideLogin());
    dispatch(hideRegister());
  }

  function handleCloseButtons() {
    dispatch(close());

    dispatch(hideCoupe());
    dispatch(hideBrand());
    dispatch(hideFuel());
    dispatch(hideTransmission());
    dispatch(hidePrice());
    dispatch(hideYear());
    dispatch(hideCity());
    dispatch(hideColor());
    dispatch(hideHP());
    dispatch(hideDoors());
    dispatch(hideExtras());
    dispatch(hideWheel());
  }

  return (
    <div>
      {isOpenModal && (
        <Modal onClose={() => dispatch(close())}>
          {isLogin && <Login onCloseModal={handleCloseLoginRegister} />}
          {isRegister && <Register onCloseModal={handleCloseLoginRegister} />}

          {isCoupe && <OpenButtons onCloseModal={handleCloseButtons} />}
        </Modal>
      )}
    </div>
  );
}

export default OpenModal;
