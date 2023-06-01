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