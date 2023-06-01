export function Error({error}) {
  if (error && error.code && error.message) {
    if (error.code == 409) {
      return (<></>)
    }
    return (
      <div>
        <h1>{error.code}</h1>
        <p>{error.message}</p>
      </div>
    )
  } else {
    return (<></>)
  }
}

export function ErrorText({children}) {
  return (
    <span className='ml-1 text-red-600'>{children}</span>
  )
}

export function Button({className='', onClick=() => {}, children=(<></>)}) {
  return (
    <button className={`${className} bg-cyan-600 hover:bg-cyan-800 shadow-md rounded-xl m-1 p-1 text-white`} onClick={onClick}>{children}</button>
  )
}