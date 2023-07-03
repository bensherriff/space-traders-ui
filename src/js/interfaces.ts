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

export interface IMeta {
  total: number;
  page: number;
  limit: number;
}

export interface IResponse {
  data: any;
  error: IError;
  meta: IMeta;
}

export interface IContract {
  id: string;
  factionSymbol: string;
  type: string;
  terms: {
    deadline: string;
    payment: {
      onAccepted: number;
      onFullfilled: number;
    }
    deliver: [
      {
        tradeSymbol: string;
        destinationSymbol: string;
        unitsRequired: number;
        unitsFulfilled: number;
      }
    ]
  }
  accepted: boolean;
  deadlineToAccept: string;
}