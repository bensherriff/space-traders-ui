import {useParams} from 'react-router-dom';

export default function Ship() {
  const {shipId} = useParams();
  return (
    <div>
      {shipId}
    </div>
  )
}