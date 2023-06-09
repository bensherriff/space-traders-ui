import { useState } from 'react'
import { CheckIcon, ChevronUpDownIcon } from '@heroicons/react/20/solid'
import { Combobox } from '@headlessui/react'

function classNames(...classes: (string | boolean)[]) {
  return classes.filter(Boolean).join(' ')
}

export default function SymbolAutoComplete({ items, selectedItem, setSelectedItem, filterLimit = 50 } : { items: { symbol: string }[], selectedItem: { symbol: string }, setSelectedItem: (item: { symbol: string }) => void, filterLimit?: number }) {
  const [query, setQuery] = useState('')

  function filteredItems() {
    if (query === '') {
      return items.slice(0, filterLimit);
    } else {
      const filtered: any[] = [];
      for (const item of items) {
        if (item.symbol.toLowerCase().includes(query)) {
          filtered.push(item);
        }
        if (filtered.length >= filterLimit) {
          break;
        }
      }
      return filtered;
    }
  }

  return (
    <Combobox as="div" value={selectedItem} onChange={setSelectedItem}>
      <div className="relative mt-2">
        <Combobox.Input
          className="w-full rounded-md border-0 bg-stone-800 py-1.5 pl-3 pr-10 text-white shadow-sm ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
          onChange={(event) => setQuery(event.target.value.toLowerCase())}
          displayValue={(s: { symbol: string }) => s?.symbol}
        />
        <Combobox.Button className="absolute inset-y-0 right-0 flex items-center rounded-r-md px-2 focus:outline-none">
        <ChevronUpDownIcon className="h-5 w-5 text-gray-400" aria-hidden="true" />
        </Combobox.Button>

        {filteredItems().length > 0 && (
          <Combobox.Options className="absolute z-10 mt-1 max-h-60 w-full overflow-auto rounded-md bg-stone-700 py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm">
            {filteredItems().map((s) => (
              <Combobox.Option
                key={s.symbol}
                value={s}
                className={({ active }) =>
                  classNames(
                    'relative cursor-default select-none py-2 pl-3 pr-9',
                    active ? 'bg-indigo-600 text-white' : 'text-gray-950'
                  )
                }
              >
                {({ active, selected }) => (
                  <>
                    <span className={classNames('block truncate', selected && 'font-semibold')}>{s.symbol}</span>

                    {selected && (
                      <span
                        className={classNames(
                          'absolute inset-y-0 right-0 flex items-center pr-4',
                          active ? 'text-white' : 'text-indigo-600'
                        )}
                      >
                        <CheckIcon className="h-5 w-5" aria-hidden="true" />
                      </span>
                    )}
                  </>
                )}
              </Combobox.Option>
            ))}
          </Combobox.Options>
        )}
      </div>
    </Combobox>
  )
}