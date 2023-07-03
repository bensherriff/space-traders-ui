import { useEffect, useState } from "react"
import { Fragment } from 'react'
import { Menu, Transition } from '@headlessui/react'
import { EllipsisVerticalIcon } from '@heroicons/react/20/solid'
import { NavLink } from "react-router-dom";
import { Storage, Text } from '../js';
import { invoke } from "@tauri-apps/api";
import { IContract, IResponse } from "../js/interfaces";

export default function Contracts() {
  const [contracts, setContracts] = useState<IContract[]>([]);

  useEffect(() => {
    listContracts();
  }, []);

  async function listContracts() {
    const response: IResponse = await invoke("list_contracts", { token: Storage.getToken() });
    if (response && response.data) {
      setContracts(response.data);
    } else if (response && response.error) {
      console.error(response.error);
    }
  }

  function classNames(...classes: string[]) {
    return classes.filter(Boolean).join(' ')
  }

  const statuses = {
    Fulfilled: 'text-green-700 bg-green-50 ring-green-600/20',
    "In Progress": 'text-gray-600 bg-gray-50 ring-gray-500/10',
    Open: 'text-yellow-800 bg-yellow-50 ring-yellow-600/20',
  }

  function determineStatus(contract: any) {
    if (contract.fulfilled) {
      return "Fulfilled";
    } else if (contract.accepted) {
      return "In Progress";
    } else {
      return "Open";
    }
  }

  return (
    <div>
      <h1 className='text-center pb-2 text-2xl'>Contracts</h1>
      <ul role="list" className="divide-y divide-gray-100">
        {contracts.map((contract) => (
          <li key={contract.id} className="flex items-center justify-between gap-x-6 py-5 pl-2 hover:bg-stone-800 select-none cursor-default">
            <div className="min-w-0">
              <div className="flex items-start gap-x-3">
                <p className="text-sm font-semibold leading-6 text-gray-200">{contract.id}</p>
                <p
                  className={classNames(
                    statuses[determineStatus(contract)],
                    'rounded-md whitespace-nowrap mt-0.5 px-1.5 py-0.5 text-xs font-medium ring-1 ring-inset'
                  )}
                >
                  {determineStatus(contract)}
                </p>
              </div>
              <div className="mt-1 flex items-center gap-x-2 text-xs leading-5 text-gray-400">
                <p className="whitespace-nowrap">
                  Expires on {Text.date(contract.deadlineToAccept)}
                </p>
                <svg viewBox="0 0 2 2" className="h-0.5 w-0.5 fill-current">
                  <circle cx={1} cy={1} r={1} />
                </svg>
                <p className="truncate">Accept by {Text.date(contract.deadlineToAccept)}</p>
              </div>
            </div>
            <div className="flex flex-none items-center gap-x-4">
              <NavLink
                to={`/contracts/${contract.id}`}
                className="hidden rounded-md bg-white px-2.5 py-1.5 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:block"
              >
                View Contract<span className="sr-only">, {contract.id}</span>
              </NavLink>
              <Menu as="div" className="relative flex-none">
                <Menu.Button className="-m-2.5 block p-2.5 text-gray-500 hover:text-gray-900">
                  <span className="sr-only">Open options</span>
                  <EllipsisVerticalIcon className="h-5 w-5" aria-hidden="true" />
                </Menu.Button>
                <Transition
                  as={Fragment}
                  enter="transition ease-out duration-100"
                  enterFrom="transform opacity-0 scale-95"
                  enterTo="transform opacity-100 scale-100"
                  leave="transition ease-in duration-75"
                  leaveFrom="transform opacity-100 scale-100"
                  leaveTo="transform opacity-0 scale-95"
                >
                  <Menu.Items className="absolute right-0 z-10 mt-2 w-32 origin-top-right rounded-md bg-white py-2 shadow-lg ring-1 ring-gray-900/5 focus:outline-none">
                    <Menu.Item>
                      {({ active }) => (
                        <a
                          href="#"
                          className={classNames(
                            active ? 'bg-gray-50' : '',
                            'block px-3 py-1 text-sm leading-6 text-gray-900'
                          )}
                        >
                          Accept
                        </a>
                      )}
                    </Menu.Item>
                    <Menu.Item>
                      {({ active }) => (
                        <a
                          href="#"
                          className={classNames(
                            active ? 'bg-gray-50' : '',
                            'block px-3 py-1 text-sm leading-6 text-gray-900'
                          )}
                        >
                          Deliver
                        </a>
                      )}
                    </Menu.Item>
                    <Menu.Item>
                      {({ active }) => (
                        <a
                          href="#"
                          className={classNames(
                            active ? 'bg-gray-50' : '',
                            'block px-3 py-1 text-sm leading-6 text-gray-900'
                          )}
                        >
                          Fulfill
                        </a>
                      )}
                    </Menu.Item>
                  </Menu.Items>
                </Transition>
              </Menu>
            </div>
          </li>
        ))}
      </ul>
    </div>
  )
}