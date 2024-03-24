import { configureStore } from '@reduxjs/toolkit';

import modalReducer from './slices/ModalSlice';
import loginRegisterReducer from './slices/LoginRegisterSlice';

const store = configureStore({
  reducer: {
    modal: modalReducer,
    logReg: loginRegisterReducer,
  },
});

export default store;
