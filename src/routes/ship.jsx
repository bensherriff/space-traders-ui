import { useEffect, useState } from 'react';
import {useParams} from 'react-router-dom';
import { invoke } from "@tauri-apps/api/tauri";
import { Storage, Text } from '../js';
import { Button, Error, ErrorText } from '../components';
import { ProgressBarWithLabel } from '../components/ProgressBar';
import { CargoInfo, ModulesInfo, MountsInfo, NavStatusLink, Navigation } from '../components/Ship';

export default function Ship() {
  const {shipId} = useParams();
  const [ship, setShip] = useState({});
  const [error, setError] = useState({});
  const [refuelError, setRefuelError] = useState(null);
  const [waypoint, setWaypoint] = useState(null);
  const [market, setMarket] = useState(null);
  const [shipyard, setShipyard] = useState(null);

  useEffect(() => {
    get_ship();
  }, []);

  function update_ship(ship) {
    setShip(ship);
  }

  async function get_ship() {
    invoke("get_ship", { token: Storage.getToken(), symbol: shipId}).then(response => {
      if (response && response.data) {
        setShip(response.data);
        setError("");
      } else if (response && response.error) {
        setError(response.error);
      }
    });
  }

  async function get_waypoint() {

  }

  async function get_market() {

  }

  async function get_shipyard() {

  }

  async function refuel_ship() {
    invoke('refuel_ship', { token: Storage.getToken(), symbol: ship.symbol }).then(response => {
      if (response && response.data) {
        Storage.setAgent(response.data.agent);
        update_ship({
          ...ship,
          fuel: {
            ...ship.fuel,
            current: response.data.fuel.current
          }
        });
        setRefuelError("");
      } else if (response && response.error) {
        setRefuelError(response.error.message);
      }
    }); 
  }

  return (
    <>
      <Error error={error}/>
      {ship && ship.symbol? (
        <div>
          <div className='my-4 p-2 border-stone-900 border-2 text-l shadow-md bg-stone-700 rounded-xl'>
            <span className='flex'>
              <h1 className='font-bold mr-4'>{ship.registration.name} <i>{Text.capitalize(ship.registration.role)} {ship.frame.name.split(" ")[1]}</i></h1>
            </span>
            <div className='flex justify-between'>
              <ProgressBarWithLabel label="Condition" text={`${ship.frame.condition}/100`} percentage={ship.frame.condition} />
              <ProgressBarWithLabel label={<>Fuel
                {ship.fuel.current < ship.fuel.capacity && ship.nav.status !== 'IN_TRANSIT'? (
                  <Button className='ml-1 text-sm' onClick={refuel_ship}>Refuel</Button>
                ): <></>}
                <ErrorText>{refuelError}</ErrorText>
              </>} text={`${ship.fuel.current}/${ship.fuel.capacity}`} percentage={ship.fuel.current/ship.fuel.capacity*100} />
              <ProgressBarWithLabel
                label={`Crew (${Text.capitalize(ship.crew.rotation)})`}
                text={`${ship.crew.current}/${ship.crew.capacity} (${ship.crew.required} required)`}
                percentage={ship.crew.current/ship.crew.capacity*100}
              />
            </div>
            <div className='flex justify-center'>
              <div className='mx-4'>
                <ProgressBarWithLabel
                  label={`${ship.engine.name} Engine (${ship.engine.speed} Speed)`}
                  text={`${ship.engine.condition}/100`}
                  percentage={ship.engine.condition} />
              </div>
              <div className='mx-4'>
                <ProgressBarWithLabel
                  label={`${ship.reactor.name} (${ship.reactor.powerOutput} Output)`}
                  text={`${ship.reactor.condition}/100`}
                  percentage={ship.reactor.condition} />
              </div>
            </div>
          </div>
          <CargoInfo ship={ship}/>
          <Navigation ship={ship} updateShip={update_ship}/>
          <div className='flex justify-center'>
            <ModulesInfo ship={ship}/>
            <MountsInfo ship={ship}/>
          </div>
        </div>
      ):<></>}
    </>
  )
}