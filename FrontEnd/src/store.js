import { configureStore } from '@reduxjs/toolkit';

import modalReducer from './slices/ModalSlice';
import loginReducer from './slices/LoginSlice';
import registerReducer from './slices/RegisterSlice';

const store = configureStore({
  reducer: {
    modal: modalReducer,
    login: loginReducer,
    register: registerReducer,
  },
});

export default store;
