import { Outlet } from 'react-router';
import NavBar from './NavBar';
import { Main } from './Main';
import SearchBy from './SearchBy';

function AppLayout() {
  return (
    <>
      <NavBar />
      <SearchBy />

      <Main>
        <Outlet />
      </Main>
    </>
  );
}

export default AppLayout;
