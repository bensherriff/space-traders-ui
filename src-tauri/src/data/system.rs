use std::str::FromStr;

use crate::data::models::{WaypointDB, NewWaypointDB, WaypointTraitDB, NewWaypointTraitDB, SystemWaypointDB, NewSystemWaypointDB, MarketDB, MarketTransactionsDB, MarketTradeGoodsDB, NewMarketDB, NewMarketTradeGoodsDB, NewMarketTransactionsDB};
use crate::models::SymbolResponse;
use crate::models::chart::Chart;
use crate::models::market::{Market, MarketItem, MarketItemType, TradeGood, SupplyType};
use crate::models::system::{System, SystemWaypoint, SystemType};
use crate::data::{models::{SystemDB, NewSystemDB}, schema};
use crate::models::trait_type::TraitType;
use crate::models::transaction::{Transaction, TransactionType};
use crate::models::waypoint::{WaypointType, Waypoint, WaypointTrait};

use diesel::dsl::{count_star};
use diesel::{prelude::*, replace_into};
use diesel::{RunQueryDsl, QueryDsl, insert_or_ignore_into, SqliteConnection, r2d2::{Pool, ConnectionManager}};

pub fn get_system(pool: &Pool<ConnectionManager<SqliteConnection>>, system_symbol: &str) -> Option<System> {
  use schema::systems;

  let mut connection = pool.get().unwrap();
  let result: Result<SystemDB, diesel::result::Error> = systems::table
    .filter(systems::system_symbol.eq(system_symbol))
    .select(SystemDB::as_select())
    .first(&mut connection);
  match result {
      Ok(r) => {
        let mut system_factions: Vec<SymbolResponse> = vec![];
        for (_index, faction_symbol) in r.factions.split(",").filter(|&x| !x.is_empty()).enumerate() {
          system_factions.push(SymbolResponse { symbol: faction_symbol.to_string() })
        }

        Some(System {
          symbol: r.system_symbol,
          sector_symbol: r.sector_symbol,
          system_type: SystemType::from_str(&r.system_type).unwrap(),
          x: r.x,
          y: r.y,
          waypoints: get_system_waypoints(pool, system_symbol),
          factions: system_factions
        })
      }
      Err(_err) => None
    }
}

pub fn get_systems_count(pool: &Pool<ConnectionManager<SqliteConnection>>) -> i64 {
  use schema::systems;

  let mut connection = pool.get().unwrap();
  let result: Result<i64, diesel::result::Error> = systems::table
    .select(count_star())
    .get_result(&mut connection);
  match result {
    Ok(r) => r,
    Err(_err) => 0
  }
}

pub fn get_all_systems(pool: &Pool<ConnectionManager<SqliteConnection>>, min: Option<i32>, max: Option<i32>) -> Vec<System> {
  use schema::systems;

  let min_id = match min {
    Some(id) => id,
    None => 1
  };
  let max_id = match max {
    Some(id) => id,
    None => i32::MAX
  };

  let mut connection = pool.get().unwrap();
  let result: Result<Vec<SystemDB>, diesel::result::Error> = systems::table
    .select(SystemDB::as_select())
    .filter(systems::rowid.ge(min_id).and(systems::rowid.le(max_id)))
    .order(systems::rowid.asc())
    .load(&mut connection);
  match result {
      Ok(r) => {
        let mut _systems: Vec<System> = vec![];
        for system in r.iter() {
          let mut system_factions: Vec<SymbolResponse> = vec![];
          for (_index, faction_symbol) in system.factions.split(",").filter(|&x| !x.is_empty()).enumerate() {
            system_factions.push(SymbolResponse { symbol: faction_symbol.to_string() })
          }

          _systems.push(System {
            symbol: system.system_symbol.to_string(),
            sector_symbol: system.sector_symbol.to_string(),
            system_type: SystemType::from_str(&system.system_type).unwrap(),
            x: system.x,
            y: system.y,
            waypoints: get_system_waypoints(pool, &system.system_symbol),
            factions: system_factions
          });
        }
        _systems
      }
      Err(_err) => vec![]
    }
}

pub fn insert_system(pool: &Pool<ConnectionManager<SqliteConnection>>, system: &System) {
  use schema::systems;

  let mut connection: diesel::r2d2::PooledConnection<ConnectionManager<SqliteConnection>> = pool.get().unwrap();
  let mut _waypoints = "".to_string();
  for (_index, waypoint) in system.waypoints.iter().enumerate() {
    insert_system_waypoint(pool, &system.symbol, waypoint);
  }
  let mut _factions = "".to_string();
  for (index, faction) in system.factions.iter().enumerate() {
    if index < system.factions.len() - 1 {
      _factions = format!("{}{},", _factions, faction.symbol);
    } else {
      _factions = format!("{}{}", _factions, faction.symbol);
    }
  }

  let _system = NewSystemDB {
    system_symbol: &system.symbol,
    sector_symbol: &system.sector_symbol,
    system_type: &system.system_type.to_string(),
    x: system.x,
    y: system.y,
    factions: &_factions
  };
  insert_or_ignore_into(systems::table)
    .values(&_system)
    .execute(&mut connection)
    .expect("Error saving new system");
}

pub fn get_system_waypoints(pool: &Pool<ConnectionManager<SqliteConnection>>, system_symbol: &str) -> Vec<SystemWaypoint> {
  use schema::system_waypoints;

  let mut connection = pool.get().unwrap();
  let result: Result<Vec<SystemWaypointDB>, diesel::result::Error> = system_waypoints::table
    .filter(system_waypoints::system_symbol.eq(system_symbol))
    .select(SystemWaypointDB::as_select())
    .load(&mut connection);
  match result {
    Ok(w) => {
      let mut waypoints: Vec<SystemWaypoint> = vec![];
      for item in w.iter() {
        waypoints.push(SystemWaypoint {
          symbol: item.waypoint_symbol.to_string(),
          waypoint_type: WaypointType::from_str(&item.waypoint_type).unwrap(),
          x: item.x,
          y: item.y
        });
      }
      waypoints
    }
    Err(_err) => vec![]
  }
}

// pub fn get_system_waypoint(pool: &Pool<ConnectionManager<SqliteConnection>>, system_symbol: &str, waypoint_symbol: &str) -> Option<SystemWaypoint> {
//   use schema::waypoints;

//   let mut connection = pool.get().unwrap();
//   let result: Result<WaypointDB, diesel::result::Error> = waypoints::table
//     .filter(waypoints::system_symbol.eq(system_symbol).and(waypoints::waypoint_symbol.eq(waypoint_symbol)))
//     .select(WaypointDB::as_select())
//     .first(&mut connection);
//   match result {
//     Ok(w) => {
//       Some(SystemWaypoint {
//         symbol: w.waypoint_symbol,
//         waypoint_type: WaypointType::from_str(&w.waypoint_type).unwrap(),
//         x: w.x,
//         y: w.y
//       })
//     }
//     Err(_err) => None
//   }
// }

pub fn insert_system_waypoint(pool: &Pool<ConnectionManager<SqliteConnection>>, system_symbol: &str, waypoint: &SystemWaypoint) {
  use schema::system_waypoints;

  let mut connection = pool.get().unwrap();
  let _system_waypoint = NewSystemWaypointDB {
    waypoint_symbol: &waypoint.symbol,
    system_symbol: &system_symbol,
    waypoint_type: &waypoint.waypoint_type.to_string(),
    x: waypoint.x,
    y: waypoint.y,
  };

  insert_or_ignore_into(system_waypoints::table)
    .values(&_system_waypoint)
    .execute(&mut connection)
    .expect("Error saving new system waypoint");
}

pub fn get_waypoint(pool: &Pool<ConnectionManager<SqliteConnection>>, waypoint_symbol: &str) -> Option<Waypoint> {
  use schema::waypoints;

  let mut connection = pool.get().unwrap();
  let result: Result<WaypointDB, diesel::result::Error> = waypoints::table
    .filter(waypoints::waypoint_symbol.eq(waypoint_symbol))
    .select(WaypointDB::as_select())
    .first(&mut connection);
  match result {
    Ok(w) => {
      let mut orbitals: Vec<SymbolResponse> = vec![];
      for (_index, item) in w.orbitals.split(",").filter(|&x| !x.is_empty()).enumerate() {
        orbitals.push(SymbolResponse { symbol: item.to_string() })
      }
      let mut traits: Vec<WaypointTrait> = vec![];
      for (_index, item) in w.traits.split(",").filter(|&x| !x.is_empty()).enumerate() {
        match get_waypoint_trait(pool, item) {
          Some(t) => traits.push(t),
          None => {}
        }
      }

      Some(Waypoint {
        symbol: w.waypoint_symbol,
        waypoint_type: WaypointType::from_str(&w.waypoint_type).unwrap(),
        system_symbol: w.system_symbol,
        x: w.x,
        y: w.y,
        orbitals,
        faction: match w.faction {
          Some(f) => Some(SymbolResponse { symbol: f }),
          None => None
        },
        traits,
        chart: Some(Chart {
          waypoint: w.chart_waypoint,
          submitted_by: w.chart_submitted_by,
          submitted_on: w.chart_submitted_on
        })
      })
    }
    Err(_err) => None
  }
}

pub fn insert_waypoint(pool: &Pool<ConnectionManager<SqliteConnection>>, waypoint: &Waypoint) {
  use schema::waypoints;

  let mut connection = pool.get().unwrap();
  let mut orbitals = "".to_string();
  for (index, item) in waypoint.orbitals.iter().enumerate() {
    if index < waypoint.orbitals.len() - 1 {
      orbitals = format!("{}{},", orbitals, item.symbol);
    } else {
      orbitals = format!("{}{}", orbitals, item.symbol);
    }
  }
  let mut traits = "".to_string();
  for (index, item) in waypoint.traits.iter().enumerate() {
    insert_waypoint_trait(pool, item);
    if index < waypoint.traits.len() - 1 {
      traits = format!("{}{},", traits, item.symbol);
    } else {
      traits = format!("{}{}", traits, item.symbol);
    }
  }

  let _waypoint = NewWaypointDB {
    waypoint_symbol: &waypoint.symbol,
    system_symbol: &waypoint.system_symbol,
    waypoint_type: &waypoint.waypoint_type.to_string(),
    x: waypoint.x,
    y: waypoint.y,
    orbitals: &orbitals,
    faction: match &waypoint.faction {
      Some(f) => Some(&f.symbol),
      None => None
    },
    traits: &traits,
    chart_waypoint: match &waypoint.chart {
      Some(c) => match &c.waypoint {
        Some(w) => Some(&w),
        None => None
      },
      None => None
    },
    chart_submitted_by: match &waypoint.chart {
      Some(c) => match &c.submitted_by {
        Some(w) => Some(&w),
        None => None
      },
      None => None
    },
    chart_submitted_on: match &waypoint.chart {
      Some(c) => match &c.submitted_on {
        Some(w) => Some(&w),
        None => None
      },
      None => None
    },
  };

  insert_or_ignore_into(waypoints::table)
    .values(&_waypoint)
    .execute(&mut connection)
    .expect("Error saving new waypoint");
}

pub fn get_waypoint_trait(pool: &Pool<ConnectionManager<SqliteConnection>>, trait_symbol: &str) -> Option<WaypointTrait> {
  use schema::waypoint_traits;

  let mut connection = pool.get().unwrap();
  let result: Result<WaypointTraitDB, diesel::result::Error> = waypoint_traits::table
    .filter(waypoint_traits::trait_symbol.eq(trait_symbol))
    .select(WaypointTraitDB::as_select())
    .first(&mut connection);
  match result {
    Ok(t) => {
      Some(WaypointTrait { symbol: TraitType::from_str(&t.trait_symbol).unwrap(), name: t.name, description: t.description })
    }
    Err(_err) => None
  }
}

pub fn insert_waypoint_trait(pool: &Pool<ConnectionManager<SqliteConnection>>, waypoint_trait: &WaypointTrait) {
  use schema::waypoint_traits;

  let mut connection = pool.get().unwrap();
  let _waypoint_trait = NewWaypointTraitDB {
    trait_symbol: &waypoint_trait.symbol.to_string(),
    name: &waypoint_trait.name,
    description: &waypoint_trait.description,
  };

  insert_or_ignore_into(waypoint_traits::table)
    .values(&_waypoint_trait)
    .execute(&mut connection)
    .expect("Error saving new waypoint trait");
}

pub fn get_market(pool: &Pool<ConnectionManager<SqliteConnection>>, waypoint_symbol: &str) -> Option<Market> {
  use schema::markets;
  use schema::market_trade_goods;
  use schema::market_transactions;

  let mut connection = pool.get().unwrap();
  let market_results: Result<Vec<MarketDB>, diesel::result::Error> = markets::table
    .filter(markets::waypoint_symbol.eq(waypoint_symbol))
    .select(MarketDB::as_select())
    .load(&mut connection);
  let mut imports: Vec<MarketItem> = vec![];
  let mut exports: Vec<MarketItem> = vec![];
  let mut exchange: Vec<MarketItem> = vec![];
  match market_results {
    Ok(r) => {
      if r.len() == 0 as usize {
        return None;
      }
      for item in r.iter() {
        let market_item = MarketItem {
          symbol: MarketItemType::from_str(&item.symbol).unwrap(),
          name: item.name.to_string(),
          description: item.description.to_string()
        };
        if item.market_type.eq("import") {
          imports.push(market_item);
        } else if item.market_type.eq("export") {
          exports.push(market_item);
        } else if item.market_type.eq("exchange") {
          exchange.push(market_item);
        }
      }
    }
    Err(_err) => return None
  };

  let transaction_results: Result<Vec<MarketTransactionsDB>, diesel::result::Error> = market_transactions::table
    .filter(market_transactions::waypoint_symbol.eq(waypoint_symbol))
    .order(market_transactions::timestamp.desc())
    .select(MarketTransactionsDB::as_select())
    .load(&mut connection);
  let mut transactions: Vec<Transaction> = vec![];
  match transaction_results {
    Ok(r) => {
      for transaction in r.iter() {
        transactions.push(Transaction {
          waypoint_symbol: transaction.waypoint_symbol.to_string(),
          ship_symbol: transaction.ship_symbol.to_string(),
          trade_symbol: transaction.trade_symbol.to_string(),
          transaction_type: TransactionType::from_str(&transaction.transaction_type).unwrap(),
          units: transaction.units,
          price_per_unit: transaction.price_per_unit,
          total_price: transaction.total_price,
          timestamp: transaction.timestamp.to_string()
        });
      }
    }
    Err(_err) => {}
  };

    let trade_good_results: Result<Vec<MarketTradeGoodsDB>, diesel::result::Error> = market_trade_goods::table
    .filter(market_trade_goods::waypoint_symbol.eq(waypoint_symbol))
    .select(MarketTradeGoodsDB::as_select())
    .load(&mut connection);
  let mut trade_goods: Vec<TradeGood> = vec![];
  match trade_good_results {
    Ok(r) => {
      for trade_good in r.iter() {
        trade_goods.push(TradeGood {
          symbol: trade_good.symbol.to_string(),
          trade_volume: trade_good.trade_volume,
          supply_type: SupplyType::from_str(&trade_good.supply_type).unwrap(),
          purchase_price: trade_good.purchase_price,
          sell_price: trade_good.sell_price
        });
      }
    }
    Err(_err) => {}
  };

  Some(Market {
    symbol: waypoint_symbol.to_string(),
    exports,
    imports,
    exchange,
    transactions: Some(transactions),
    trade_goods: Some(trade_goods),
  })
}

pub fn insert_market(pool: &Pool<ConnectionManager<SqliteConnection>>, waypoint_symbol: &str, market: &Market) {
  use schema::markets;
  use schema::market_trade_goods;
  use schema::market_transactions;

  let mut connection = pool.get().unwrap();
  for item in market.exports.iter() {
    let _market_item = NewMarketDB {
      waypoint_symbol,
      market_type: "exports",
      symbol: &item.symbol.to_string(),
      name: &item.name,
      description: &item.description,
    };
    replace_into(markets::table)
      .values(_market_item)
      .execute(&mut connection)
      .expect("Error saving new markets");
  }

  for item in market.imports.iter() {
    let _market_item = NewMarketDB {
      waypoint_symbol,
      market_type: "import",
      symbol: &item.symbol.to_string(),
      name: &item.name,
      description: &item.description,
    };
    replace_into(markets::table)
      .values(_market_item)
      .execute(&mut connection)
      .expect("Error saving new markets");
  }

  for item in market.exchange.iter() {
    let _market_item = NewMarketDB {
      waypoint_symbol,
      market_type: "exchange",
      symbol: &item.symbol.to_string(),
      name: &item.name,
      description: &item.description,
    };
    replace_into(markets::table)
      .values(_market_item)
      .execute(&mut connection)
      .expect("Error saving new markets");
  }

  match &market.trade_goods {
    Some(g) => {
      for good in g.iter() {
        let _good = NewMarketTradeGoodsDB {
            waypoint_symbol,
            symbol: &good.symbol,
            trade_volume: good.trade_volume,
            supply_type: &good.supply_type.to_string(),
            purchase_price: good.purchase_price,
            sell_price: good.sell_price,
        };
        replace_into(market_trade_goods::table)
          .values(_good)
          .execute(&mut connection)
          .expect("Error saving new market goods");
      }
    }
    None => {}
  }

  match &market.transactions {
    Some(t) => {
      for transaction in t.iter() {
        let _transaction = NewMarketTransactionsDB {
            waypoint_symbol,
            ship_symbol: &transaction.ship_symbol,
            trade_symbol: &transaction.trade_symbol,
            transaction_type: &transaction.transaction_type.to_string(),
            units: transaction.units,
            price_per_unit: transaction.price_per_unit,
            total_price: transaction.total_price,
            timestamp: &transaction.timestamp,
        };
        replace_into(market_transactions::table)
          .values(_transaction)
          .execute(&mut connection)
          .expect("Error saving new market transactions");
      }
    }
    None => {}
  }
}
