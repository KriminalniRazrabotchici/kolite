import styled from 'styled-components';

import { useDispatch, useSelector } from 'react-redux';

import Modal from '../ui/Modal';

import { closeSearch } from '../slices/ModalSlice';

import {
  showCoupe,
  showBrand,
  showModel,
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
  hideModel,
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
import { useQuery } from '@tanstack/react-query';
import { getCars } from '../services/getCars';
import Loader from '../ui/Loader';

const Button = styled.button`
  display: flex;
  align-items: center;
  justify-content: center;

  width: 13rem;
  height: 3.5rem;

  font-size: 1.6rem;

  background-color: transparent;
  color: var(--color-red-500);
  /* word-wrap: break-word; */
  border: none;
  /* border-radius: var(--border-radius-round); */
  box-shadow: var(--shadow-md);

  position: relative;

  transition: all 0.5s;

  &::before {
    content: '';
    background-color: var(--color-red-500);
    position: absolute;
    left: 0;
    top: 100%;
    width: 0;
    height: 0.2rem;

    transition: all 0.5s;
  }

  &:hover,
  &:active {
    &::before {
      width: 100%;
    }
    color: var(--black);
    box-shadow: none;
  }
`;

const Container = styled.div`
  /* width: 20rem; */
  height: 80vh;

  display: flex;
  flex-wrap: wrap;
  gap: 2.4rem;

  overflow: scroll;

  &::-webkit-scrollbar {
    display: none;
  }
`;

function OpenButtons() {
  const isOpenSearchModal = useSelector((state) => state.modal.isOpenSearch);

  const {
    isCoupe,
    isBrand,
    isModel,
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
    dispatch(hideModel());
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

  const {
    isLoading,
    data: cars,
    error,
  } = useQuery({
    queryKey: ['brands'],
    queryFn: getCars,
  });

  if (isLoading) {
    return <Loader />;
  }

  console.log(cars);

  const sortedCars = cars.sort((a, b) => a.name.localeCompare(b.name));

  return (
    <div>
      {isOpenSearchModal && (
        <Modal onClose={handleCloseButtons}>
          <Container>
            {isBrand &&
              sortedCars.map((brand) => (
                <Button onClick={() => console.log('works')} key={brand.name}>
                  {brand.name}
                </Button>
              ))}
          </Container>
        </Modal>
      )}
    </div>
  );
}

export default OpenButtons;
