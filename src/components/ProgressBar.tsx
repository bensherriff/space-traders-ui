export function ProgressBarWithLabel({ label, text, percentage }: { label: string, text: string, percentage: number }) {
  return (
    <div className="w-64">
      <label className="ml-2 mb-1 text-md">{label}</label>
      <ProgressBar text={text} percentage={percentage} />
    </div>
  )
}

export function ProgressBar({ text, percentage }: { text: string, percentage: number }) {
  return (
    <div className="w-64 h-6 mb-4 bg-gray-400 rounded-full flex">
      <div className="h-6 bg-blue-600 rounded-full" style={{width: percentage + '%'}}/>
      <div className="pl-2 text-white absolute">{text}</div>
    </div>
  )
}