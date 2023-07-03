import { atom } from 'recoil';
import { IAgent } from './interfaces';

export const agentState = atom({
  key: 'agentState',
  default: null as IAgent | null
})