import {useParams} from 'react-router-dom';
import { useRecoilState} from "recoil";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from 'react';
import { Storage, Text, State } from '../js';
import Tag from '../components/Tag';
import { NavLink } from "react-router-dom";
import { WaypointHeader } from '../components/Location/LocationHeader';
import { Button } from '../components';
import SymbolAutoComplete from '../components/Form/SymbolAutoComplete';

export default function Waypoint() {
  const {systemId, waypointId} = useParams();
  const [waypoint, setWaypoint] = useState(null);
  const [waypointTraits, setWaypointTraits] = useState([]);
  const [marketToggle, setMarketToggle] = useState(false);
  const [shipyardToggle, setShipyardToggle] = useState(false);
  const [jumpGateToggle, setJumpGateToggle] = useState(false);
  const [ships, setShips] = useState({});
  const [localShips, setLocalShips] = useState([]);
  const [localQuery, setLocalQuery] = useState("");
  const [localShip, setLocalShip] = useState(null);
  const [otherShips, setOtherShips] = useState([]);
  const [otherQuery, setOtherQuery] = useState("");
  const [otherShip, setOtherShip] = useState(null);

  useEffect(() => {
    get_waypoint();
    get_ships();
  }, [systemId, waypointId]);

  async function get_waypoint() {
    invoke("get_waypoint", { token: Storage.getToken(), system: systemId, waypoint: waypointId}).then((response) => {
      if (response && response.data) {
        setWaypoint(response.data);
        setWaypointTraits(response.data.traits);
        setMarketToggle(response.data.traits.some(trait => trait.symbol === 'MARKETPLACE'));
        setJumpGateToggle(response.data.type == 'JUMP_GATE');
      } else if (response && response.error) {
        console.error(response.error);
      }
    });
  }

  function get_ships() {
    invoke("list_ships", { token: Storage.getToken() }).then((response) => {
      if (response && response.data) {
        let _ships = ships;
        response.data.forEach((ship) => {
          _ships[ship.symbol] = ship;
        });
        setShips(_ships);
        // Local ships
        invoke("get_ships_at_waypoint", { waypoint: waypointId }).then((res) => {
          if (res && res.data) {
            setLocalShips(res.data);
            if (res.data.length > 0) {
              setLocalShip(res.data[0]);
              // Other ships
              let _otherShips = Object.values(_ships).filter(object1 => {
                return !res.data.some(object2 => {
                  return object1.symbol === object2.symbol;
                });
              });
              setOtherShips(_otherShips);
              if (_otherShips.length > 0) {
                setOtherShip(_otherShips[0]);
              }
            } else {
              setLocalShips([]);
              setLocalShip(null);
              setOtherShips(response.data);
              if (response.data.length > 0) {
                setOtherShip(response.data[0]);
              } else {
                setOtherShip(null);
              }
            }
          } else if (res && res.error) {
            console.error(res.error);
          }
        });
      } else if (response && response.error) {
        console.error(response.error);
      }
    });
  }

  function toggleMarket() {
    setShipyardToggle(false);
    setJumpGateToggle(false);
    setMarketToggle(!marketToggle);
  }

  function toggleShipyard() {
    setMarketToggle(false);
    setJumpGateToggle(false);
    setShipyardToggle(!shipyardToggle);
  }

  function toggleJumpGate() {
    setMarketToggle(false);
    setShipyardToggle(false);
    setJumpGateToggle(!jumpGateToggle);
  }

  async function orbit_ship() {
    invoke("orbit_ship", { token: Storage.getToken(), symbol: localShip.symbol}).then(response => {
      if (response && response.data) {
        setLocalShip(null);
        get_ships();
      }
    });
  }
  
  async function dock_ship() {
    invoke("dock_ship", { token: Storage.getToken(), symbol: otherShip.symbol}).then(response => {
      if (response && response.data) {
        setOtherShip(null);
        get_ships();
      }
    });
  }

  async function navigate() {
    invoke("navigate_ship_anywhere", {
      token: Storage.getToken(),
      symbol: otherShip.symbol,
      waypoint: waypointId,
      system: systemId
    }).then(response => {
      console.log(response);
    });
  }

  return (
    <div>
      {waypoint? (
        <div>
          <WaypointHeader waypoint={waypoint}/>
          <div className='flex ml-6 cursor-default select-none'>
            <NavLink to={`/system/${systemId}`}><span>System {systemId}</span></NavLink>
            <span className='pl-4'>({waypoint.x},{waypoint.y})</span>
          </div>
          <hr className='mb-5'/>
          <div className='w-full text-center mb-5'>
            {waypointTraits && Array.isArray(waypointTraits)? (
              waypointTraits.map((trait, index) => (
                <Tag key={index} text={trait.name}/>
              ))
            ): <></>}
          </div>
          {waypoint.orbitals.length > 0? (
            <>
              <h2 className='text-xl'>In Orbit</h2>
              <div className='grid grid-cols-12 gap-1'>
                {waypoint.orbitals.map((orbital, index) => (
                  <NavLink key={index} to={`/system/${systemId}/${orbital.symbol}`}>
                    <span className='inline-block py-2 p-1 mb-2 bg-[#4b5563] hover:bg-[#2b3e58] shadow-md rounded-md select-none mr-2'>{orbital.symbol}</span>
                  </NavLink>
                ))}
              </div>
            </>
          ): <></>}
          <div className='flex'>
            <div className='w-full mr-1'>
              {waypoint.type == 'JUMP_GATE'? (
                <Button onClick={toggleJumpGate}>Jump Gate</Button>
              ): <></>}
              {waypointTraits.some(trait => trait.symbol === 'MARKETPLACE')? (
                <Button onClick={toggleMarket}>Marketplace</Button>
              ): <></>}
              {waypointTraits.some(trait => trait.symbol === 'SHIPYARD')? (
                <Button onClick={toggleShipyard}>Shipyard</Button>
              ): <></>}
              {jumpGateToggle? (
                <JumpGate systemId={systemId} waypointId={waypointId}/>
              ): <></>}
              {marketToggle? (
                <Marketplace systemId={systemId} waypointId={waypointId} ship={localShip}/>
              ): <></>}
              {shipyardToggle? (
                <Shipyard systemId={systemId} waypointId={waypointId}/>
              ): <></>}
            </div>
            <div className='w-full h-full ml-1'>
              {localShip? (
                <>
                  <h1 className='text-center text-2xl'>Current Ship</h1>
                  <SymbolAutoComplete items={localShips} selectedItem={localShip} setSelectedShip={setLocalShip} />
                  <NavLink to={`/fleet/${localShip.symbol}`}>Ship Info</NavLink>
                  <Button onClick={orbit_ship}>Orbit</Button>
                </>
              ): <></>}
              {localShip && otherShip? ( <hr className='my-4'/> ): <></>}
              {otherShip? (
                <>
                  <h1 className='text-center text-2xl'>Remote Ships</h1>
                  <SymbolAutoComplete items={otherShips} selectedItem={otherShip} setSelectedShip={setOtherShip}/>
                  <NavLink to={`/fleet/${otherShip.symbol}`}>Ship Info</NavLink>
                  {otherShip.nav.route.destination.symbol == waypointId? (<>
                    <Button onClick={dock_ship}>Dock</Button>
                  </>): <>
                    <Button onClick={navigate}>Navigate Here</Button>
                  </>}
                </>
              ): <></>}
            </div>
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
    invoke("get_jump_gate", { token: Storage.getToken(), system: systemId, waypoint: waypointId }).then(response => {
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

function Marketplace({systemId, waypointId, ship}) {
  const [market, setMarket] = useState({});
  const [exportsToggle, setExportsToggle] = useState(false);
  const [importsToggle, setImportsToggle] = useState(false);
  const [exchangeToggle, setExchangeToggle] = useState(false);
  const [transactionsToggle, setTransactionsToggle] = useState(false);

  useEffect(() => {
    get_market();
  }, [systemId, waypointId]);

  async function get_market() {
    invoke("get_market", { token: Storage.getToken(), system: systemId, waypoint: waypointId}).then((response) => {
      if (response && response.data) {
        setMarket(response.data);
      } else if (response && response.error) {
        console.error(response.error.message);
      }
    });
  }

  function toggleTransactions() {
    setTransactionsToggle(!transactionsToggle);
  }

  function toggleExports() {
    setExportsToggle(!exportsToggle);
  }

  function toggleImports() {
    setImportsToggle(!importsToggle);
  }

  function toggleExchange() {
    setExchangeToggle(!exchangeToggle);
  }

  return (
    <>
      {market && market.symbol? (
        <div className='block py-2 shadow-md rounded-lg border bg-stone-800 border-stone-900'>
          <h1 className='text-center text-2xl'>Marketplace {!ship? (<>(stale)</>):<></>}</h1>
          <hr className='m-2'/>
          {market.tradeGoods && Array.isArray(market.tradeGoods)? (
            <>
              <table className='w-full text-sm text-left text-gray-500 dark:text-gray-400'>
                <thead className='text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400'>
                  <tr>
                    <th scope='col' className='px-2 py-3'>Item</th>
                    <th scope='col' className='px-2 py-3'>Supply</th>
                    { ship? ( <th scope='col' className='px-2 py-3'>Price</th> ): <th scope='col' className='px-2 py-3'>Buy</th> }
                    { ship? ( <th scope='col' className='px-2 py-3'>Action</th> ): <th scope='col' className='px-2 py-3'>Sell</th> }
                  </tr>
                </thead>
                {market.tradeGoods.map((good, index) => <MarketAction key={index} good={good} ship={ship}/>)}
              </table>
            </>
          ): <></>}
          {market.transactions && Array.isArray(market.transactions)? (
            <>
              <h2 onClick={toggleTransactions} className='text-center text-lg cursor-pointer select-none'>Latest Transactions</h2>
              <table hidden={!transactionsToggle} className='w-full text-sm text-left text-gray-500 dark:text-gray-400'>
                <thead className='text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400'>
                  <tr>
                    <th scope='col' className='px-2 py-3'>Trade Good</th>
                    <th scope='col' className='px-2 py-3'>Ship</th>
                    <th scope='col' className='px-2 py-3'>Type</th>
                    <th scope='col' className='px-2 py-3'>Amount</th>
                    <th scope='col' className='px-2 py-3'>Price</th>
                    <th scope='col' className='px-2 py-3'>Total</th>
                    <th scope='col' className='px-2 py-3'>Timestamp</th>
                  </tr>
                </thead>
                <tbody>
                  {market.transactions.map((transaction, index) => (
                    <tr key={index} className='bg-white border-b dark:bg-gray-800 dark:border-gray-700'>
                      <th scope='row' className='px-2 py-2 font-medium text-gray-900 whitespace-nowrap dark:text-white'>{Text.capitalize(transaction.tradeSymbol)}</th>
                      <td className='px-2 py-2'>{transaction.shipSymbol}</td>
                      <td className='px-2 py-2'>{Text.capitalize(transaction.type)}</td>
                      <td className='px-2 py-2'>{transaction.units}</td>
                      <td className='px-2 py-2'>{Text.currency(transaction.pricePerUnit)}</td>
                      <td className='px-2 py-2'>{Text.currency(transaction.totalPrice)}</td>
                      <td className='px-2 py-2'>{Text.date(transaction.timestamp)}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </>
          ): <></>}
          </div>
      ): <></>}
    </>
  )
}

function MarketAction({good, ship}) {
  const [agent, setAgent] = useRecoilState(State.agentState);
  const [marketAction, setMarketAction] = useState("buy");
  const [errorMessage, setErrorMessage] = useState("");
  const [amount, setAmount] = useState();
  
  useEffect(() => {
    setErrorMessage("");
  }, [amount, marketAction]);

  async function handleAction() {
    console.log(marketAction, amount);
    if (marketAction == 'buy') {
      invoke("purchase_cargo", { token: Storage.getToken(), symbol: ship.symbol, itemSymbol: good.symbol, units: amount}).then((response) => {
        if (response && response.data) {
          setAgent(response.data.agent);
        } else if (response && response.error) {
          console.error(response.error.message);
          setErrorMessage(response.error.message);
        }
      });
    } else if (marketAction == 'sell') {
      invoke("sell_cargo", { token: Storage.getToken(), symbol: ship.symbol, itemSymbol: good.symbol, units: amount}).then((response) => {
        if (response && response.data) {
          setAgent(response.data.agent);
        } else if (response && response.error) {
          console.error(response.error.message);
          setErrorMessage(response.error.message);
        }
      });
    } else if (marketAction == 'sellAll') {
      
    }
  }

  return (
    <tbody>
      <tr className='bg-white border-b dark:bg-gray-800 dark:border-gray-700'>
        <th scope='row' className='px-2 py-2 font-medium text-gray-900 whitespace-nowrap dark:text-white'>{Text.capitalize(good.symbol)}</th>
        <td className='px-2 py-2'>{Text.capitalize(good.supply)}</td>
        { ship? (
          <>
            {marketAction == "buy"? (
              <td className='px-2 py-2 text-red-400'>{Text.currency(good.purchasePrice)}</td>
            ):
              <td className='px-2 py-2 text-green-400'>{Text.currency(good.sellPrice)}</td>
            }
            <td className='px-2 py-2'>
              <form
                onSubmit={(e) => {
                  e.preventDefault();
                  handleAction();
                }}
                className=''
              >
                <select value={marketAction} onChange={(e) => setMarketAction(e.target.value)} className='mx-1 mt-0.5 p-1 py-2 rounded text-white bg-gray-700'>
                  <option value={'buy'}>Buy</option>
                  <option value={'sell'}>Sell</option>
                  {/* <option value={'sellAll'}>Sell All</option> */}
                </select>
                <input
                  type='number'
                  min={0}
                  value={amount}
                  hidden={marketAction == "sellAll"}
                  onChange={(e) => {
                    setAmount(Math.abs(e.currentTarget.value))
                  }} placeholder={good.tradeVolume}
                  className='mx-1 mt-0.5 p-1 py-2 rounded text-white bg-gray-700'/>
                <button className="float-right button" type="submit">Submit</button>
                <p className='text-red-600'>{errorMessage}</p>
              </form>
            </td>
          </>
        ):
        <>
          <td className='px-2 py-2 text-red-400'>{Text.currency(good.purchasePrice)}</td>
          <td className='px-2 py-2 text-green-400'>{Text.currency(good.sellPrice)}</td>
        </>
        }
      </tr>
    </tbody>
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