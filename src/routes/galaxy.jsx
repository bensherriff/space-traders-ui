import { useEffect, useState } from 'react';
import { Storage } from '../js';

export default function Galaxy() {
  const [loadComplete, setLoadComplete] = useState(false);

  useEffect(() => {
    get_all_systems();
  }, []);

  async function get_all_systems() {
    invoke("load_all_systems", { token: Storage.getToken() }).then(response => {
      if (response && response.data) {
        console.log(response);
        setLoadComplete(true);
      }
    })
  }

  return (
    <div>
      <h1 className='text-center pb-2 text-2xl'>Galaxy</h1>
      {!loadComplete? (
        <>If this is your first time seeing this message, it may take up to 5 minutes to completely populate the galaxy due to rate limits. Please wait... </>
      ):
      <>Load complete!</>
      }
    </div>
  )
}