import {useParams} from 'react-router-dom';
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from 'react';
import { Storage, Text } from '../js';
import Tag from '../components/Tag';
import { NavLink } from "react-router-dom";
import { WaypointHeader } from '../components/Location/LocationHeader';

export default function Waypoint() {
  const {systemId, waypointId} = useParams();
  const [waypoint, setWaypoint] = useState({});
  const [waypointTraits, setWaypointTraits] = useState([]);

  useEffect(() => {
    get_waypoint();
  }, [systemId, waypointId]);

  async function get_waypoint() {
    invoke("get_waypoint", { token: Storage.getToken(), system: systemId, waypoint: waypointId}).then((response) => {
      if (response && response.data) {
        setWaypoint(response.data);
        setWaypointTraits(response.data.traits);
      } else if (response && response.error) {
        console.error(response.error);
      }
    });
  }

  return (
    <div>
      {waypoint && waypoint.type? (
        <div>
          <WaypointHeader symbol={waypoint.symbol} type={waypoint.type}/>
          <hr className='mb-5'/>
          <div className='w-full text-center mb-5'>
            {waypointTraits && Array.isArray(waypointTraits)? (
              waypointTraits.map((trait, index) => (
                <Tag key={index} text={trait.name}/>
              ))
            ): <></>}
          </div>
          <div className='flex'>
            <NavLink to={`/system/${systemId}`}>
              <span className='block p-2 mb-2 bg-[#4b5563] hover:bg-[#2b3e58] shadow-md rounded-md select-none mr-2'>System: {systemId}</span>
            </NavLink>
            {waypoint.faction? 
            <NavLink to={`/faction/${waypoint.faction.symbol}`}>
              <span className='block p-2 mb-2 bg-[#4b5563] hover:bg-[#2b3e58] shadow-md rounded-md select-none mr-2'>Faction: {waypoint.faction.symbol}</span>
            </NavLink>
            : <></>}
            <span className='block p-2 mb-2 bg-[#4b5563] shadow-md rounded-md select-none'>Coordinates: ({waypoint.x},{waypoint.y})</span>
          </div>
          {waypoint.orbitals && Array.isArray(waypoint.orbitals) && waypoint.orbitals.length > 0? (
            <div>
              <hr className='my-2'/>
              <h2 className='text-xl'>In Orbit</h2>
              <div className='flex'>
                {waypoint.orbitals.map((orbital, index) => (
                  <NavLink key={index} to={`/system/${systemId}/${orbital.symbol}`}>
                    <span className='block p-2 mb-2 bg-[#4b5563] hover:bg-[#2b3e58] shadow-md rounded-md select-none mr-2'>{orbital.symbol}</span>
                  </NavLink>
                ))}
              </div>
            </div> 
          ): <></>}
          {waypoint.type == 'JUMP_GATE'? (
            <JumpGate systemId={systemId} waypointId={waypointId}/>
          ): <></>}
          <div className='w-full flex'>
          {waypointTraits.some(trait => trait.symbol === 'MARKETPLACE')? (
            <Marketplace systemId={systemId} waypointId={waypointId}/>
          ): <></>}
          {waypointTraits.some(trait => trait.symbol === 'SHIPYARD')? (
            <Shipyard systemId={systemId} waypointId={waypointId}/>
          ): <></>}
          </div>
        </div>
      ): <></>}
    </div>
  )
}

function JumpGate({systemId, waypointId}) {
  const [jumpGate, setJumpGate] = useState({});

  useEffect(() => {
    get_jump_gate();
  }, []);

  async function get_jump_gate() {
    await invoke("get_jump_gate", { token: Storage.getToken(), system: systemId, waypoint: waypointId }).then(response => {
      if (response && response.data) {
        setJumpGate(response.data);
      }
    });
  }

  return (
    <div className='block p-4 shadow-md rounded-lg border bg-stone-800 border-stone-900'>
      <h1 className='text-center text-2xl'>Destinations</h1>
      <hr className='my-2'/>
      {jumpGate && jumpGate.connectedSystems? (
        <div>
          Range: {jumpGate.jumpRange}
          {jumpGate.factionSymbol}
          <ul className='block w-64'>
          {jumpGate.connectedSystems.map((system, index) => (
            <NavLink key={index} to={`/system/${system.symbol}`}>
              <li className='block p-2 mb-2 bg-[#4b5563] hover:bg-[#2b3e58] shadow-md rounded-md select-none'>
                <span className='mr-4'>{Text.capitalize(system.type)}</span>
                <span className='float-right'>{system.symbol}</span>
              </li>
            </NavLink>
          ))}
          </ul>
        </div>
      ): <></>}
    </div>
  )
}

function Marketplace({systemId, waypointId}) {
  const [market, setMarket] = useState({});

  useEffect(() => {
    get_market();
  }, [systemId, waypointId]);

  async function get_market() {
    invoke("get_market", { token: Storage.getToken(), system: systemId, waypoint: waypointId}).then((response) => {
      if (response && response.data) {
        setMarket(response.data);
      }
    });
  }

  return (
    <>
      {market && market.symbol? (
        <div className='block p-4 shadow-md rounded-lg border bg-stone-800 border-stone-900'>
          <h1 className='text-center text-2xl'>Marketplace</h1>
          <hr className='my-2'/>
          <div className='flex justify-center'>
          {market.transactions && Array.isArray(market.transactions)? (
            <div className='mx-2'>
              <h2 className='text-center text-lg'>Transactions</h2>
              <table className='w-full text-sm text-left text-gray-500 dark:text-gray-400'>
                <thead className='text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400'>
                  <tr>
                    <th scope='col' className='px-6 py-3'>Trade Good</th>
                    <th scope='col' className='px-6 py-3'>Ship</th>
                    <th scope='col' className='px-6 py-3'>Type</th>
                    <th scope='col' className='px-6 py-3'>Amount</th>
                    <th scope='col' className='px-6 py-3'>Price</th>
                    <th scope='col' className='px-6 py-3'>Total Price</th>
                    <th scope='col' className='px-6 py-3'>Timestamp</th>
                  </tr>
                </thead>
                <tbody>
                  {market.transactions.map((transaction, index) => (
                    <tr key={index} className='bg-white border-b dark:bg-gray-800 dark:border-gray-700'>
                      <th scope='row' className='px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white'>{Text.capitalize(transaction.tradeSymbol)}</th>
                      <td className='px-6 py-4'>{transaction.shipSymbol}</td>
                      <td className='px-6 py-4'>{Text.capitalize(transaction.type)}</td>
                      <td className='px-6 py-4'>{transaction.units}</td>
                      <td className='px-6 py-4'>{Text.currency(transaction.pricePerUnit)}</td>
                      <td className='px-6 py-4'>{Text.currency(transaction.totalPrice)}</td>
                      <td className='px-6 py-4'>{Text.date(transaction.timestamp)}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          ): <></>}
          {market.tradeGoods && Array.isArray(market.tradeGoods)? (
            <div className='mx-2'>
              <h2 className='text-center text-lg'>Trade Goods</h2>
              <table className='w-full text-sm text-left text-gray-500 dark:text-gray-400'>
                <thead className='text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400'>
                  <tr>
                    <th scope='col' className='px-6 py-3'>Trade Good</th>
                    <th scope='col' className='px-6 py-3'>Volume</th>
                    <th scope='col' className='px-6 py-3'>Supply</th>
                    <th scope='col' className='px-6 py-3'>Purchase Price</th>
                    <th scope='col' className='px-6 py-3'>Sell Price</th>
                  </tr>
                </thead>
                <tbody>
                  {market.tradeGoods.map((good, index) => (
                    <tr key={index} className='bg-white border-b dark:bg-gray-800 dark:border-gray-700'>
                      <th scope='row' className='px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white'>{Text.capitalize(good.symbol)}</th>
                      <td className='px-6 py-4'>{good.tradeVolume}</td>
                      <td className='px-6 py-4'>{Text.capitalize(good.supply)}</td>
                      <td className='px-6 py-4'>{Text.currency(good.purchasePrice)}</td>
                      <td className='px-6 py-4'>{Text.currency(good.sellPrice)}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          ): <></>}
          </div>
        </div>
      ): <></>}
    </>
  )
}

function Table({title, headers, rows}) {
  return (
    <div>
      <h2 className='text-center text-lg'>{title}</h2>
      <table className='w-full text-sm text-left text-gray-500 dark:text-gray-400'>
        <thead className='text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400'>
          <tr>
            {headers.map((header, index) => (
              <th key={index} scope='col' className='px-6 py-3'>{header}</th>
            ))}
          </tr>
        </thead>
        <tbody>
          {rows.map((row, index) => {
            <tr key={index} className='bg-white border-b dark:bg-gray-800 dark:border-gray-700'>
              {row.map((field, _index) => (
                <>
                  {_index == 0? (
                    <th key={`${index}_${_index}`} scope='row' className='px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white'>{field}</th>
                  ): <td key={`${index}_${_index}`} className='px-6 py-4'>{field}</td>}
                </>
              ))}
            </tr>
          })}
        </tbody>
      </table>
    </div>
  )
}

function Shipyard({systemId, waypointId}) {
  const [shipyard, setShipyard] = useState({});

  useEffect(() => {
    get_shipyard();
  }, [systemId, waypointId]);

  async function get_shipyard() {
    invoke("get_shipyard", { token: Storage.getToken(), system: systemId, waypoint: waypointId}).then((response) => {
      setShipyard(response.data);
    });
  }

  return (
    <div className='w-full mx-2'>
      <hr className='my-2'/>
      <h1 className='text-center text-xl'>Shipyard</h1>
    </div>
  )
}