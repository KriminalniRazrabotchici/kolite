import styled from 'styled-components';

import { useDispatch, useSelector } from 'react-redux';

import ReactSlider from 'react-slider';

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
import {
  getBrands,
  getCities,
  getColors,
  getCoupes,
  getDoors,
  getExtras,
  getFuel,
  getModels,
  getTransmission,
  getWheel,
} from '../services/cars';
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

const SliderContainer = styled.div`
  width: 55rem;
  height: 10rem;
  display: flex;
  justify-content: center;
  align-items: center;
`;

const Slider = styled(ReactSlider)`
  width: 50rem;
  color: var(--black);

  .example-track {
    top: 0.7rem;
    background-color: var(--color-red-500);
    height: 0.5rem;
    border-radius: 4px;
  }

  .example-thumb {
    font-size: 1.2rem;
    height: 2rem;
    width: 2rem;
    background-color: var(--color-red-500);
    border-radius: 50%;
    cursor: grab;
    display: flex;
    justify-content: center;
    align-items: center;
    color: var(--black);
  }

  .example-thumb span {
    width: 10rem;
    padding: 0.2rem 0.8rem;
    color: var(--color-red-500);
    border: 1px solid var(--color-red-500);
    transform: translateY(100%);
  }
`;

function OpenButtons() {
  const isOpenSearchModal = useSelector((state) => state.modal.isOpenSearch);

  const [chooseBrand, setChooseBrand] = useState('');

  const [price, setPrice] = useState([0, 100000]);
  const [year, setYear] = useState([1970, 2024]);
  const [hp, setHP] = useState([0, 500]);

  const getStep = (value) => {
    return value[0] > 25000 || value[1] > 25000 ? 5000 : 1000;
  };

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

  const handleChangePrice = (value) => {
    setPrice(value);
  };

  const handleChangeYear = (value) => {
    setYear(value);
  };

  const handleChangeHP = (value) => {
    setHP(value);
  };

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

  const {
    isLoading: isLoading4,
    data: fuel,
    error: error4,
  } = useQuery({
    queryKey: ['fuel'],
    queryFn: getFuel,
  });

  const {
    isLoading: isLoading5,
    data: transmission,
    error: error5,
  } = useQuery({
    queryKey: ['transmission'],
    queryFn: getTransmission,
  });

  const {
    isLoading: isLoading6,
    data: cities,
    error: error6,
  } = useQuery({
    queryKey: ['cities'],
    queryFn: getCities,
  });

  const {
    isLoading: isLoading7,
    data: colors,
    error: error7,
  } = useQuery({
    queryKey: ['colors'],
    queryFn: getColors,
  });

  const {
    isLoading: isLoading8,
    data: doors,
    error: error8,
  } = useQuery({
    queryKey: ['doors'],
    queryFn: getDoors,
  });

  const {
    isLoading: isLoading9,
    data: extras,
    error: error9,
  } = useQuery({
    queryKey: ['extras'],
    queryFn: getExtras,
  });

  const {
    isLoading: isLoading10,
    data: wheels,
    error: error10,
  } = useQuery({
    queryKey: ['wheels'],
    queryFn: getWheel,
  });

  // console.log(models?.[chooseBrand]);

  if (
    isLoading ||
    isLoading2 ||
    isLoading3 ||
    isLoading4 ||
    isLoading5 ||
    isLoading6 ||
    isLoading7 ||
    isLoading8 ||
    isLoading9 ||
    isLoading10
  ) {
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

            {isModel &&
              chooseBrand !== '' &&
              models?.[chooseBrand].map((el) => (
                <Button onClick={() => console.log('works')} key={el.model}>
                  {el.model}
                </Button>
              ))}

            {isFuel &&
              fuel.map((fuel) => (
                <Button onClick={() => console.log('works')} key={fuel.type}>
                  {fuel.type}
                </Button>
              ))}

            {isTransmission &&
              transmission.map((transmission) => (
                <Button
                  onClick={() => console.log('works')}
                  key={transmission.type}>
                  {transmission.type}
                </Button>
              ))}

            {isCity &&
              cities.map((city) => (
                <Button onClick={() => console.log('works')} key={city.name}>
                  {city.name}
                </Button>
              ))}

            {isColor &&
              colors.map((color) => (
                <Button onClick={() => console.log('works')} key={color.type}>
                  {color.type}
                </Button>
              ))}

            {isDoors &&
              doors.map((door) => (
                <Button onClick={() => console.log('works')} key={door.number}>
                  {door.number}
                </Button>
              ))}

            {isExtras &&
              extras.map((extra) => (
                <Button onClick={() => console.log('works')} key={extra.type}>
                  {extra.type}
                </Button>
              ))}

            {isWheel &&
              wheels.map((wheel) => (
                <Button onClick={() => console.log('works')} key={wheel.side}>
                  {wheel.side}
                </Button>
              ))}

            {isPrice && (
              <SliderContainer>
                <Slider
                  className='horizontal-slider'
                  thumbClassName='example-thumb'
                  trackClassName='example-track'
                  defaultValue={price}
                  max={100000}
                  min={0}
                  step={getStep(price)}
                  // value={price}
                  onChange={handleChangePrice}
                  renderThumb={(props, state) => (
                    <div {...props}>
                      <span>{state.valueNow}</span>
                    </div>
                  )}
                />
              </SliderContainer>
            )}
            {isYear && (
              <SliderContainer>
                <Slider
                  className='horizontal-slider'
                  thumbClassName='example-thumb'
                  trackClassName='example-track'
                  defaultValue={year}
                  max={2024}
                  min={1970}
                  // step={1}
                  onChange={handleChangeYear}
                  renderThumb={(props, state) => (
                    <div {...props}>
                      <span>{state.valueNow}</span>
                    </div>
                  )}
                />
              </SliderContainer>
            )}
            {isHP && (
              <SliderContainer>
                <Slider
                  className='horizontal-slider'
                  thumbClassName='example-thumb'
                  trackClassName='example-track'
                  defaultValue={hp}
                  max={500}
                  min={0}
                  step={25}
                  onChange={handleChangeHP}
                  renderThumb={(props, state) => (
                    <div {...props}>
                      <span>{state.valueNow}</span>
                    </div>
                  )}
                />
              </SliderContainer>
            )}
          </Container>
        </Modal>
      )}
    </div>
  );
}

export default OpenButtons;
