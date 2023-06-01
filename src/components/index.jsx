import { useState, useEffect } from "react"

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

export function Button({className='', onClick=() => {}, children=(<></>), disabled=false}) {
  if (disabled) {
    return (
      <button disabled className={`${className} bg-stone-600 shadow-md rounded-xl m-1 p-1 text-white`} onClick={onClick}>{children}</button>
    )
  } else {
    return (
      <button className={`${className} bg-cyan-600 hover:bg-cyan-800 shadow-md rounded-xl m-1 p-1 text-white`} onClick={onClick}>{children}</button>
    )
  }
}

export function CountdownTimer({duration = 0}) {
  const [counter, setCounter] = useState(Math.ceil(duration));

  useEffect(() => {
    const timer =
      counter > 0 && setInterval(() => setCounter(counter - 1), 1000);
    return () => clearInterval(timer);
  }, [counter]);

  function toTime(totalSeconds) {
    const totalMinutes = Math.floor(totalSeconds / 60);
    const seconds = (totalSeconds % 60).toLocaleString('en-US', {
      minimumIntegerDigits: 2,
      useGrouping: false
    });
    const hours = (Math.floor(totalMinutes / 60)).toLocaleString('en-US', {
      minimumIntegerDigits: 2,
      useGrouping: false
    });
    const minutes = (totalMinutes % 60).toLocaleString('en-US', {
      minimumIntegerDigits: 2,
      useGrouping: false
    });
    return { h: hours, m: minutes, s: seconds };
  }

  return (
    <>
      {toTime(counter).h}:{toTime(counter).m}:{toTime(counter).s}
    </>
  );
}