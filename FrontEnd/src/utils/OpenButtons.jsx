import { useDispatch, useSelector } from 'react-redux';

import Modal from '../ui/Modal';

import { closeSearch } from '../slices/ModalSlice';

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

function OpenButtons() {
  const isOpenSearchModal = useSelector((state) => state.modal.isOpenSearch);

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
  } = useSelector((state) => state.search);

  const dispatch = useDispatch();

  function handleCloseButtons() {
    dispatch(closeSearch());

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
      {isOpenSearchModal && (
        <Modal onClose={handleCloseButtons}>{isCoupe && 'test'}</Modal>
      )}
    </div>
  );
}

export default OpenButtons;
