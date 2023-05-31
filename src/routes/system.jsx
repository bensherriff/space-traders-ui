import {useParams} from 'react-router-dom';

export default function System() {
  const {systemId} = useParams();
  return (
    <div>
      System
    </div>
  )
}