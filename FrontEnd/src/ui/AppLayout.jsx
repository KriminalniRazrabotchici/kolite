import { Outlet } from 'react-router';
import NavBar from './NavBar';
import { Main } from './Main';

function AppLayout() {
  return (
    <>
      <NavBar />

      <Main>
        <Outlet />
      </Main>
    </>
  );
}

export default AppLayout;
