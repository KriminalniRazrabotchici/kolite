import { BrowserRouter, Navigate, Route, Routes } from 'react-router-dom';
import { Toaster } from 'react-hot-toast';

import Home from './pages/Home';
import AppLayout from './ui/AppLayout';
import PageNotFound from './pages/PageNotFound';
import GlobalStyles from './styles/GlobalStyles';
import Contacts from './pages/Contacts';
import Login from './pages/Login';
import Register from './pages/Register';
import AddNew from './pages/AddNew';

function App() {
  return (
    <>
      <GlobalStyles />
      <BrowserRouter>
        <Routes>
          <Route element={<AppLayout />}>
            <Route index element={<Navigate replace to='home' />} />
            <Route path='home' element={<Home />} />
            <Route path='contacts' element={<Contacts />} />
            <Route path='login' element={<Login />} />
            <Route path='register' element={<Register />} />
            <Route path='publish' element={<AddNew />} />
          </Route>
          <Route path='*' element={<PageNotFound />} />
        </Routes>
        <Toaster />
      </BrowserRouter>
    </>
  );
}

export default App;
