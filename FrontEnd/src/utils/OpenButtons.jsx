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
import { getBrands, getCoupes, getModels } from '../services/cars';
import Loader from '../ui/Loader';
import { useState } from 'react';

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
  column-gap: 2.4rem;

  overflow: scroll;

  &::-webkit-scrollbar {
    display: none;
  }
`;

function OpenButtons() {
  const isOpenSearchModal = useSelector((state) => state.modal.isOpenSearch);

  const [chooseBrand, setChooseBrand] = useState('');

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

  function handleBrand(brand) {
    setChooseBrand(brand.name);
    handleCloseButtons();
  }

  const {
    isLoading,
    data: brands,
    error,
  } = useQuery({
    queryKey: ['brands'],
    queryFn: getBrands,
  });

  const {
    isLoading: isLoading2,
    data: coupes,
    error: error2,
  } = useQuery({
    queryKey: ['coupes'],
    queryFn: getCoupes,
  });

  const {
    isLoading: isLoading3,
    data: models,
    error: error3,
  } = useQuery({
    queryKey: ['models'],
    queryFn: getModels,
  });

  console.log(models?.[chooseBrand]);

  if (isLoading || isLoading2 || isLoading3) {
    return <Loader />;
  }

  // console.log(brands);

  const sortedBrands = brands.sort((a, b) => a.name.localeCompare(b.name));

  return (
    <div>
      {isOpenSearchModal && (
        <Modal onClose={handleCloseButtons}>
          <Container>
            {isCoupe &&
              coupes.map((coupe) => (
                <Button onClick={() => console.log('works')} key={coupe.name}>
                  {coupe.name}
                </Button>
              ))}

            {isBrand &&
              sortedBrands.map((brand) => (
                <Button onClick={() => handleBrand(brand)} key={brand.name}>
                  {brand.name}
                </Button>
              ))}

            {console.log(isModel)}

            {isModel && chooseBrand !== ''
              ? models?.[chooseBrand].map((el) => (
                  <Button onClick={() => console.log('works')} key={el.model}>
                    {el.model}
                  </Button>
                ))
              : // sortedBrands.map((model) => model.Audi)
                'Select brand first'}
          </Container>
        </Modal>
      )}
    </div>
  );
}

export default OpenButtons;
