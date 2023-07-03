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
    deliver: {
      tradeSymbol: string;
      destinationSymbol: string;
      unitsRequired: number;
      unitsFulfilled: number;
    }[]
  }
  accepted: boolean;
  deadlineToAccept: string;
}

export interface IStatus {
  status: string;
  version: string;
  resetDate: string;
  description: string;
  stats: {
    agents: number;
    ships: number;
    systems: number;
    waypoints: number;
  }
  leaderboard: {
    mostCredits: {
      agentSymbol: string;
      credits: number;
    }[]
    mostSubmittedCharts: {
      agentSymbol: string;
      chartCount: number;
    }[]
  }
  serverResets: {
    next: string;
    frequency: string;
  }
  announcements: {
    title: string;
    body: string;
  }[]
  links: {
    name: string;
    url: string;
  }[]
}