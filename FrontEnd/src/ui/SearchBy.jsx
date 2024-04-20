import OpenButtons from '../utils/OpenButtons';

import { useDispatch } from 'react-redux';
import { openSearch } from '../slices/ModalSlice';

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
} from '../slices/SearchButtonsSlice';

import { SearchDiv } from './SearchDiv';
import { Button } from './SearchButton';

const buttons = [
  { id: 1, name: 'Coupe', fnShow: showCoupe() },
  { id: 2, name: 'Brand', fnShow: showBrand() },
  { id: 3, name: 'Model', fnShow: showModel() },
  { id: 4, name: 'Fuel', fnShow: showFuel() },
  { id: 5, name: 'Transmission', fnShow: showTransmission() },
  { id: 6, name: 'Price', fnShow: showPrice() },
  { id: 7, name: 'Year', fnShow: showYear() },
  { id: 8, name: 'City', fnShow: showCity() },
  { id: 9, name: 'Color', fnShow: showColor() },
  { id: 10, name: 'Doors', fnShow: showDoors() },
  { id: 11, name: 'HP', fnShow: showHP() },
  { id: 12, name: 'Extras', fnShow: showExtras() },
  { id: 13, name: 'Steering wheel', fnShow: showWheel() },
];

function SearchBy() {
  const dispatch = useDispatch();

  function handleClick(btn) {
    dispatch(openSearch());
    dispatch(btn.fnShow);
  }

  return (
    <>
      <OpenButtons />
      <SearchDiv>
        {buttons.map((btn) => (
          <Button onClick={() => handleClick(btn)} key={btn.id}>
            {btn.name}
          </Button>
        ))}
      </SearchDiv>
    </>
  );
}

export default SearchBy;
