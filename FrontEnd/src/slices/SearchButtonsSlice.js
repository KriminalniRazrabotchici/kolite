import { createSlice } from '@reduxjs/toolkit';

const initialStateButtons = {
  isCoupe: false,
  isBrand: false,
  isModel: false,
  isFuel: false,
  isTransmission: false,
  isPrice: false,
  isYear: false,
  isCity: false,
  isColor: false,
  isDoors: false,
  isHP: false,
  isExtras: false,
  isWheel: false,
};

const searchSlice = createSlice({
  name: 'search',
  initialState: initialStateButtons,
  reducers: {
    showCoupe(state) {
      state.isCoupe = true;
    },
    hideCoupe(state) {
      state.isCoupe = false;
    },
    showBrand(state) {
      state.isBrand = true;
    },
    hideBrand(state) {
      state.isBrand = false;
    },
    showModel(state) {
      state.isModel = true;
    },
    hideModel(state) {
      state.isModel = false;
    },
    showFuel(state) {
      state.isFuel = true;
    },
    hideFuel(state) {
      state.isFuel = false;
    },
    showTransmission(state) {
      state.isTransmission = true;
    },
    hideTransmission(state) {
      state.isTransmission = false;
    },
    showPrice(state) {
      state.isPrice = true;
    },
    hidePrice(state) {
      state.isPrice = false;
    },
    showYear(state) {
      state.isYear = true;
    },
    hideYear(state) {
      state.isYear = false;
    },
    showCity(state) {
      state.isCity = true;
    },
    hideCity(state) {
      state.isCity = false;
    },
    showColor(state) {
      state.isColor = true;
    },
    hideColor(state) {
      state.isColor = false;
    },
    showDoors(state) {
      state.isDoors = true;
    },
    hideDoors(state) {
      state.isDoors = false;
    },
    showHP(state) {
      state.isHP = true;
    },
    hideHP(state) {
      state.isHP = false;
    },
    showExtras(state) {
      state.isExtras = true;
    },
    hideExtras(state) {
      state.isExtras = false;
    },
    showWheel(state) {
      state.isWheel = true;
    },
    hideWheel(state) {
      state.isWheel = false;
    },
  },
});

export const {
  showCoupe,
  hideCoupe,
  showBrand,
  hideBrand,
  showModel,
  hideModel,
  showFuel,
  hideFuel,
  showTransmission,
  hideTransmission,
  showPrice,
  hidePrice,
  showYear,
  hideYear,
  showCity,
  hideCity,
  showColor,
  hideColor,
  showHP,
  hideHP,
  showDoors,
  hideDoors,
  showExtras,
  hideExtras,
  showWheel,
  hideWheel,
} = searchSlice.actions;

export default searchSlice.reducer;
