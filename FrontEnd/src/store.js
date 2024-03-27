import { configureStore } from '@reduxjs/toolkit';

import modalReducer from './slices/ModalSlice';
import loginRegisterReducer from './slices/LoginRegisterSlice';
import searchReducer from './slices/SearchButtonsSlice';

const store = configureStore({
  reducer: {
    modal: modalReducer,
    logReg: loginRegisterReducer,
    search: searchReducer,
  },
});

export default store;
