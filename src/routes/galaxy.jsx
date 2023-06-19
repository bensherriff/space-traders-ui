import { useEffect, useState } from 'react';
import { invoke } from "@tauri-apps/api/tauri";
import { Storage } from '../js';

export default function Galaxy() {
  const [systems, setSystems] = useState(null);

  useEffect(() => {
    get_all_systems();
  }, []);

  async function get_all_systems() {
    invoke("list_all_systems", { token: Storage.getToken() }).then(async response => {
      if (response && response.data) {
        console.log(response.data);
        setSystems(response.data);
      } else if (response && response.error) {
        console.error(response.error.message);
      }
    });
  }

  return (
    <div>
      <h1 className='text-center pb-2 text-2xl'>Galaxy</h1>
      {!systems? (
        <>If this is your first time seeing this message, it may take up to 5 minutes to completely populate the galaxy due to rate limits. Please wait... </>
      ):
      <>Load complete!</>
      }
    </div>
  )
}