import { n } from "@tauri-apps/api/event-41a9edf5";
import { symbol } from "prop-types";

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

export interface ISystem {
  symbol: string;
  sectorSymbol: string;
  type: string;
  x: number;
  y: number;
  waypoints: ISystemWaypoint[]
  factions: {
    symbol: string;
  }[]
}

export interface ISystemWaypoint {
  symbol: string;
    type: string;
    x: number;
    y: number;
}

export interface IShip {
  symbol: string;
  registration: {
    name: string;
    factionSymbol: string;
    role: string;
  }
  nav: {
    systemSymbol: string;
    waypointSymbol: string;
    route: {
      destination: {
        symbol: string;
        type: string;
        systemSymbol: string;
        x: number;
        y: number;
      }
      departure: {
        symbol: string;
        type: string;
        systemSymbol: string;
        x: number;
        y: number;
      }
      departureTime: string;
      arrival: string;
    }
    status: string;
    flightMode: string;
  }
  crew: {
    current: number;
    required: number;
    capacity: number;
    rotation: string;
    morale: number;
    wages: number;
  }
  frame: {
    symbol: string;
    name: string;
    description: string;
    condition: number;
    moduleSlots: number;
    mountingPoints: number;
    fuelCapacity: number;
    requirements: IRequirements;
  }
  reactor: {
    symbol: string;
    name: string;
    description: string;
    condition: number;
    powerOutput: number;
    requirements: IRequirements;
  }
  engine: {
    symbol: string;
    name: string;
    description: string;
    condition: number;
    speed: number;
    requirements: IRequirements;
  }
  modules: IModule[]
  mounts: IMount[]
  cargo: {
    capacity: number;
    units: number;
    inventory: {
      symbol: string;
      name: string;
      description: string;
      units: number;
    }[]
  }
  fuel: {
    current: number;
    capacity: number;
    consumed: {
      amount: number;
      timestamp: string;
    }
  }
}

export interface IMount {
  symbol: string;
  name: string;
  description: string;
  strength: number;
  deposits: string[];
  requirements: IRequirements;
}

export interface IModule {
  symbol: string;
  capacity: number;
  range: number;
  name: string;
  description: string;
  requirements: IRequirements;
}

export interface IRequirements {
  power: number;
  crew: number;
  slots: number;
}

export interface IFaction {
  symbol: string;
  name: string;
  description: string;
  headquarters: string;
  traits: {
    symbol: string;
    name: string;
    description: string;
  }[]
  isRecruiting: boolean;
}