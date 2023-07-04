import {useParams} from 'react-router-dom';
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from 'react';
import { getToken } from '../js/storage';
import Tag from '../components/Tag';
import { IFaction, IResponse } from '../js/interfaces';

export default function Faction() {
  const {factionId} = useParams();
  const [faction, setFaction] = useState<IFaction | null>(null);

  useEffect(() => {
    get_faction();
  }, []);
  
  async function get_faction() {
    let response: IResponse = await invoke("get_faction", { token: getToken(), factionSymbol: factionId });
    if (response && response.data) {
      setFaction(response.data);
    }
  }

  return (
    <div>
      {faction? (
        <>
          <h1 className='text-center pb-2 text-2xl'>{faction.name} ({faction.symbol})</h1>
          <h2 className='text-center'><i>{faction.description}</i></h2>
          <hr className='mb-5'/>
          <div className='w-full mb-5 text-center'>
            {faction && faction.traits && Array.isArray(faction.traits)? (
              faction.traits.map((trait, index) => (
                <Tag key={index} text={trait.name}/>
              ))
            ) : <></>}
          </div>
        </>
      ): <></>}
    </div>
  )
}