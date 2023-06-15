export default function Tag({ text, className="", bgColor="bg-green-700", textColor="text-white" }) {
  return (
    <span className={`${className} mx-1 px-2 py-2 ${bgColor} ${textColor} rounded-md select-none align-middle cursor-default shadow-md font-bold`}>
      {text}
    </span>
  )
}