export default function Tag({ text, className="" }) {
  return (
    <span className={`${className} m-1 px-4 py-2 bg-green-700 border-green-700 border-2 rounded-full select-none align-middle cursor-default shadow-md font-bold`}>
      {text}
    </span>
  )
}