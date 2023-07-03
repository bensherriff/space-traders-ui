export interface IAgent {
  accountId: string;
  symbol: string;
  headquarters: string;
  credits: number;
  startingFaction: string;
}

export interface IError {
  code: number;
  message: string;
}
