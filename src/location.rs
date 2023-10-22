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


use Country::*;

impl Country {
  /// Matches country to continent
  /// # Arguments
  /// * self - country enum
  /// # Returns
  /// * Continent enum
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
  /// Converts country string to country enum
  /// # Arguments
  /// * s - string slice containing country
  /// # Returns
  /// * Result enum that returns a Country Enum if successful or a string containing an error message.
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

use std::fmt;


#[derive(Debug)]
pub enum Continent {
  NorthAmerica,
  Europe,
  Asia,
  Oceania,
  SouthAmerica
}


impl fmt::Display for Continent {
    /// Converts Continent enum to string
    /// # Usage
    /// Continent::Asia.to_string() => "Asia"
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Continent::NorthAmerica =>write!(f,"NorthAmerica"),
            Continent::Europe =>write!(f,"Europe"),
            Continent::Asia =>write!(f,"Asia"),
            Continent::Oceania =>write!(f,"Oceania"),
            Continent::SouthAmerica =>write!(f,"SouthAmerica"),
        }
    }
}
