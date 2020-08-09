use configparser::ini::Ini;
use std::collections::HashMap;
use std::error::Error;

pub fn load_config(
) -> Result<HashMap<String, HashMap<String, Option<String>>>, Box<dyn std::error::Error>> {
    let mut config = Ini::new();
    let map = config.load("/etc/fll-scoring/config.ini")?;

    Ok(map)
}

pub fn load_service_config(
    service_name: String,
) -> Result<HashMap<String, Option<String>>, Box<dyn std::error::Error>> {
    let config = load_config()?;
    let service_config = config[&service_name].clone();

    Ok(service_config)
}

pub fn load_global_config() -> Result<HashMap<String, Option<String>>, Box<dyn std::error::Error>> {
    let config = load_config()?;
    let global_config = config["fll-scoring"].clone();

    Ok(global_config)
}

/// Gets a global configuration value
pub fn get_global_value(key: &str, panic: bool) -> Result<String, Box<dyn Error>> {
    let config = load_global_config()?;
    let mut value = String::new();
    if panic {
        value = match config.get(key) {
            Some(opt) => match opt {
                Some(val) => val.clone(),
                None => {
                    panic!("{} not set in global configuration!", key);
                }
            },
            None => panic!("{} not set in global config!", key),
        }
    } else {
        if let Some(opt) = config.get(key) {
            if let Some(val) = opt {
                value = val.clone();
            }
        }
    }

    Ok(value)
}

/// Gets a service-specific config value, optionally panic!ing if it's not established
pub fn get_service_config_value(
    service_name: &str,
    config_key: &str,
    panic: bool,
) -> Result<String, Box<dyn Error>> {
    let config = load_service_config(service_name.to_string())?;
    let mut value = String::new();
    if panic {
        value = match config.get(config_key) {
            Some(opt) => match opt {
                Some(val) => val.clone(),
                None => {
                    panic!("{} not set in {} config!", config_key, service_name);
                }
            },
            None => {
                panic!("{} not set in {} config!", config_key, service_name);
            }
        }
    } else {
        if let Some(opt) = config.get(config_key) {
            if let Some(val) = opt {
                value = val.clone();
            }
        }
    }

    Ok(value)
}
