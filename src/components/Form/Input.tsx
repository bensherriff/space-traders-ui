import { ExclamationCircleIcon } from '@heroicons/react/20/solid'
import { v4 as uuidv4 } from 'uuid';

export default function Input({ label, type="text", placeholder, errorMsg, value, onChange=(e: any) => { e.preventDefault()} } : { label?: string, type?: string, placeholder?: string, errorMsg?: string, value?: string, onChange?: any }) {
  const id = uuidv4();
  return (
    <>
      <label htmlFor={id} className="block text-sm font-medium leading-6 text-gray-300 text-left">
        {label}
      </label>
      <div className="relative mt-1 rounded-md shadow-sm">
        <input
          type={type}
          name={type}
          id={id}
          className={`block w-full rounded-md border-0 py-1.5 pr-10 ring-1 ring-inset ${errorMsg ? "text-red-900 ring-red-300 placeholder:text-red-300 focus:ring-red-500" : "text-gray-900 ring-gray-300 placeholder:text-gray-300 focus:ring-gray-500"} focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6`}
          placeholder={placeholder}
          value={value}
          onChange={onChange}
          aria-invalid="true"
          aria-describedby="text-error"
        />
        {errorMsg ? (
          <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3">
            <ExclamationCircleIcon className="h-5 w-5 text-red-500" aria-hidden="true" />
          </div>
        ): <></>}
      </div>
      <p className="mt-2 text-sm text-red-600" id="text-error">
        {errorMsg}
      </p>
    </>
  )
}