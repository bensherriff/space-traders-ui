import {useParams} from 'react-router-dom';
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from 'react';
import { SessionStorage, Text } from '../js';
import Tag from '../components/Tag';
import { NavLink } from "react-router-dom";

export default function Waypoint() {
  const {systemId, waypointId} = useParams();
  const [waypoint, setWaypoint] = useState({});
  const [waypointTraits, setWaypointTraits] = useState([]);

  useEffect(() => {
    get_waypoint();
  }, [systemId, waypointId]);

  async function get_waypoint() {
    invoke("get_waypoint", { token: SessionStorage.getSessionToken(), system: systemId, waypoint: waypointId}).then((response) => {
      setWaypoint(response.data);
      setWaypointTraits(response.data.traits);
    })
  }

  return (
    <div>
      {waypoint && waypoint.type? (
        <div>
          <h1 className='text-center text-2xl'>{waypoint.symbol} ({Text.capitalize(waypoint.type)})</h1>
          <hr className='mb-5'/>
          <div className='w-full text-center mb-5'>
            {waypointTraits && Array.isArray(waypointTraits)? (
              waypointTraits.map((trait, index) => (
                <Tag key={index} text={trait.name}/>
              ))
            ): <></>}
          </div>
          <ul>
            <li>System: <NavLink to={`/system/${systemId}`}>{systemId}</NavLink></li>
            {waypoint.faction? 
            <li>Faction: <NavLink to={`/faction/${waypoint.faction.symbol}`}>
              {waypoint.faction.symbol}</NavLink></li>
            : <></>}
            <li>Coordinates: ({waypoint.x},{waypoint.y})</li>
          </ul>
          {waypoint.orbitals && Array.isArray(waypoint.orbitals) && waypoint.orbitals.length > 0? (
            <div>
              <hr className='my-2'/>
              <h2 className='text-xl'>Orbitals</h2>
              <ul>
                {waypoint.orbitals.map((orbital, index) => (
                  <li key={index} className='pl-4'><NavLink to={`/system/${systemId}/${orbital.symbol}`}>{orbital.symbol}</NavLink></li>
                ))}
              </ul>
            </div> 
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

function Marketplace({systemId, waypointId}) {
  const [market, setMarket] = useState({});

  useEffect(() => {
    get_market();
  }, [systemId, waypointId]);

  async function get_market() {
    invoke("get_market", { token: SessionStorage.getSessionToken(), system: systemId, waypoint: waypointId}).then((response) => {
      setMarket(response.data);
    });
  }

  return (
    <>
      {market && market.symbol? (
        <div className='w-full mx-2'>
        <hr className='my-2'/>
        <h1 className='text-center text-xl'>Marketplace</h1>
        {market.transactions && Array.isArray(market.transactions)? (
          <div>
            <h2>Transactions</h2>
            <table>
              <tr>
                <th>Type</th>
                <th>Ship</th>
                <th>Trade Good</th>
                <th>Amount</th>
                <th>Price</th>
                <th>Total Price</th>
                <th>Timestamp</th>
              </tr>
              {market.transactions.map((transaction, index) => (
                <tr key={index}>
                  <td>{Text.capitalize(transaction.type)}</td>
                  <td>{transaction.shipSymbol}</td>
                  <td>{Text.capitalize(transaction.tradeSymbol)}</td>
                  <td>{transaction.units}</td>
                  <td>{transaction.pricePerUnit}</td>
                  <td>{transaction.totalPrice}</td>
                  <td>{transaction.timestamp}</td>
                </tr>
              ))}
            </table>
          </div>
        ): <></>}
      </div>
      ): <></>}
    </>
  )
}

function Shipyard({systemId, waypointId}) {
  const [shipyard, setShipyard] = useState({});

  useEffect(() => {
    get_shipyard();
  }, [systemId, waypointId]);

  async function get_shipyard() {
    invoke("get_shipyard", { token: SessionStorage.getSessionToken(), system: systemId, waypoint: waypointId}).then((response) => {
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