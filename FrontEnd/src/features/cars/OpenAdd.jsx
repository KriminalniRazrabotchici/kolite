import { useState } from 'react';
import { ButtonLink } from '../../ui/ButtonNav';
import Modal from '../../ui/Modal';
import Add from './AddForm';

function OpenAdd() {
  const [isOpenModal, setIsOpenModal] = useState(false);

  return (
    <div>
      <ButtonLink onClick={() => setIsOpenModal((show) => !show)}>
        Add
      </ButtonLink>
      {isOpenModal && (
        <Modal onClose={() => setIsOpenModal(false)}>
          <Add onCloseModal={() => setIsOpenModal(false)} />
        </Modal>
      )}
    </div>
  );
}

export default OpenAdd;
