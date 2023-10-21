#[derive(Debug)]
pub enum Country {
  UnitedStates,
  Canada,
  UnitedKingdom,
  Germany,
  France,
  Japan,
  Australia,
  China,
  Brazil,
  SouthKorea,
  Ireland,
  Spain, 
  India,
  Switzerland
}
#[derive(Debug)]
pub enum Continent {
  NorthAmerica,
  Europe,
  Asia,
  Oceania,
  SouthAmerica
}

use Country::*;

impl Country {
  pub fn country_to_continent(&self) -> Continent {
    match self {
      UnitedStates | Canada => Continent::NorthAmerica,
      UnitedKingdom | Germany | France | Ireland | Spain | Switzerland => Continent::Europe,
      Japan | China | SouthKorea | India => Continent::Asia,
      Australia => Continent::Oceania,
      Brazil => Continent::SouthAmerica,      
    }
  }
}

impl std::str::FromStr for Country {
  type Err = &'static str;
  fn from_str(s: &str) -> Result<Country, Self::Err>{
    match s {
      "USA" => Ok(Country::UnitedStates),
      "Canada" => Ok(Country::Canada),
      "UK" => Ok(Country::UnitedKingdom),
      "Germany" => Ok(Country::Germany),
      "France" => Ok(Country::France),
      "Japan" => Ok(Country::Japan),
      "Australia" => Ok(Country::Australia),
      "China" => Ok(Country::China),
      "Brazil" => Ok(Country::Brazil),
      "South Korea" => Ok(Country::SouthKorea),
      "Ireland" => Ok(Country::Ireland),
      "Spain" => Ok(Country::Spain),
      "India" => Ok(Country::India),
      "Switzerland" => Ok(Country::Switzerland),
      _ => Err("Invalid Country name"),
    }

  }
}