import {useParams} from 'react-router-dom';
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from 'react';
import { getSessionToken } from '../js/sessionStorage';
import Tag from '../components/Tag';

export default function Faction() {
  const {factionId} = useParams();
  const [faction, setFaction] = useState({});

  useEffect(() => {
    get_faction();
  }, []);
  
  async function get_faction() {
    invoke("get_faction", { token: getSessionToken(), factionSymbol: factionId }).then((response) => {
      setFaction(response.data);
    });
  }

  return (
    <div>
      <h1 className='text-center pb-2 text-2xl'>{faction.name} ({faction.symbol})</h1>
      <hr className='mb-5'/>
      <div className='mb-5 text-center'>
        {faction && faction.traits && Array.isArray(faction.traits)? (
          faction.traits.map((trait, index) => (
            <Tag key={index} text={trait.name}/>
          ))
        ) : <></>}
      </div>
      <h2 className='text-center'><i>{faction.description}</i></h2>
    </div>
  )
}