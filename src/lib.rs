// Copyright 2021 Caleb Mitchell Smith-Woolrich (PixelCoda)
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # lifx-rs
//! 
//! ## Description
//! 
//! A synchronous + asynchronous library for communicating with the LIFX-API. 
//! 
//! ## Supported API Methods:
//! * List Lights
//! * Set State
//! * Set States
//! * State Delta
//! * Toggle Power
//! * Clean (HEV)
//! * List Scenes
//! * Validate Color
//! 
//! ## How to use library
//! 
//! Add the following line to your cargo.toml:
//! ```
//! lifx-rs = "0.1.1"
//! ```
//! 
//! Example:
//! ```rust
//! extern crate lifx_rs as lifx;
//! 
//! fn main() {
//! 
//!     let key = "xxx".to_string();
//!     
//!     let mut off_state = lifx::State::new();
//!     off_state.power = Some(format!("off"));
//! 
//!     // Turn off all lights
//!     lifx::Light::set_state_by_selector(key.clone(), format!("all"), off_state);
//! 
//! 
//!     let all_lights = lifx::Light::list_all(key.clone());
//!     match all_lights {
//!         Ok(lights) => {
//!             println!("{:?}",lights.clone());
//! 
//!             let mut state = lifx::State::new();
//!             state.power = Some(format!("on"));
//!             state.brightness = Some(1.0);
//!         
//!             for light in lights {
//!                 let results = light.set_state(key.clone(), state.clone());
//!                 println!("{:?}",results);
//!             }
//!         },
//!         Err(e) => println!("{}",e)
//!     }
//! 
//! }
//! ```
//! ## License
//! 
//! Released under Apache 2.0.
//! 
//! # Support and follow my work by:
//! 
//! #### Buying my dope NTFs:
//!  * https://opensea.io/accounts/PixelCoda
//! 
//! #### Checking out my Github:
//!  * https://github.com/PixelCoda
//! 
//! #### Following my facebook page:
//!  * https://www.facebook.com/pixelcoda/
//! 
//! #### Subscribing to my Patreon:
//!  * https://www.patreon.com/calebsmith_pixelcoda
//! 
//! #### Or donating crypto:
//!  * ADA:    addr1vyjsx8zthl5fks8xjsf6fkrqqsxr4f5tprfwux5zsnz862glwmyr3
//!  * BTC:    3BCj9kYsqyENKU5YgrtHgdQh5iA7zxeJJi
//!  * MANA:   0x10DFc66F881226f2B91D552e0Cf7231C1e409114
//!  * SHIB:   0xdE897d5b511A66276E9B91A8040F2592553e6c28

use serde_json::json;

use serde::{Serialize, Deserialize};
use std::convert::TryInto;

pub type Lights = Vec<Light>;

/// Represents a LIFX Light Object
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Light {
    pub id: String,
    pub uuid: String,
    pub label: String,
    pub connected: bool,
    pub power: String,
    pub color: Color,
    pub brightness: f64,
    pub group: Group,
    pub location: Location,
    pub product: Product,
    #[serde(rename = "last_seen")]
    pub last_seen: String,
    #[serde(rename = "seconds_since_seen")]
    pub seconds_since_seen: i64,
    pub error: Option<String>,
    pub errors: Option<Vec<Error>>,
}
impl Light {

    /// Asynchronously switch a light to clean mode, with a set duration. 
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `clean` - A Clean object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let all_lights = lifx_rs::Light::list_all(key.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut clean = lifx_rs::Clean::new();
    ///             clean.duration = Some(0);
    ///             clean.stop = Some(false);
    ///         
    ///             for light in lights {
    ///                 let results = light.clean(key.clone(), clean.clone());
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub async fn async_clean(&self, access_token: String, clean: Clean) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_clean_by_selector(access_token, format!("id:{}", self.id), clean).await;
    }

    /// Asynchronously switch a selected LIFX object to clean mode, with a set duration. 
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `clean` - A Clean object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let mut clean = lifx_rs::Clean::new();
    ///     clean.duration = Some(0);
    ///     clean.stop = Some(false);
    ///     
    ///     // Set all light to clean mode
    ///     lifx_rs::Light::clean_by_selector(key.clone(), format!("all"), clean);
    /// }
    ///  ```
    pub async fn async_clean_by_selector(access_token: String, selector: String, clean: Clean) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("https://api.lifx.com/v1/lights/{}/clean", selector);

        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", access_token))
            .form(&clean.to_params())
            .send().await?;
    
        let json = request.json::<LiFxResults>().await?;
        return Ok(json);
    }



    /// Asynchronously gets ALL lights belonging to the authenticated account
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let all_lights = lifx_rs::Light::async_list_all(key).await?;
    /// }
    ///  ```
    pub async fn async_list_all(access_token: String) -> Result<Lights, reqwest::Error> {
        return Self::async_list_by_selector(access_token, format!("all")).await;
    }

    /// Asynchronously gets lights belonging to the authenticated account. Filtering the lights using selectors. Properties such as id, label, group and location can be used in selectors.
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let all_lights = lifx_rs::Light::async_list_by_selector(key, format!("all")).await?;
    /// }
    ///  ```
    pub async fn async_list_by_selector(access_token: String, selector: String) -> Result<Lights, reqwest::Error> {
        let mut url = format!("https://api.lifx.com/v1/lights/{}", selector);
        let request = reqwest::Client::new().get(url).header("Authorization", format!("Bearer {}", access_token)).send().await?;
        let json = request.json::<Lights>().await?;
        return Ok(json);
    }


    /// Asynchronously sets the state for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `state` - A State object containing the values of the State to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let all_lights = lifx_rs::Light::list_all(key.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut state = lifx_rs::State::new();
    ///             state.power = Some(format!("on"));
    ///             state.brightness = Some(1.0);
    ///         
    ///             for light in lights {
    ///                 let results = light.set_state(key.clone(), state.clone());
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub async fn async_set_state(&self, access_token: String, state: State) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_set_state_by_selector(access_token, format!("id:{}", self.id), state).await;
    }

    /// Asynchronously sets the state for the selected LIFX object
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `state` - A State object containing the values of the State to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let mut off_state = lifx_rs::State::new();
    ///     off_state.power = Some(format!("off"));
    ///     
    ///     // Turn off all lights
    ///     lifx_rs::Light::set_state_by_selector(key.clone(), format!("all"), off_state);
    /// }
    ///  ```
    pub async fn async_set_state_by_selector(access_token: String, selector: String, state: State) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("https://api.lifx.com/v1/lights/{}/state", selector);

        let request = reqwest::Client::new().put(url)
            .header("Authorization", format!("Bearer {}", access_token))
            .form(&state.to_params())
            .send().await?;
    
        let json = request.json::<LiFxResults>().await?;
        return Ok(json);
    }

    /// Asynchronously sets the state for the selected LIFX object(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `states` - A vector of States with defaults
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let mut set_states = lifx_rs::States::new();
    ///     let mut states: Vec<lifx_rs::State> = Vec::new();
    ///     let mut defaults = lifx_rs::State::new();
    ///     
    ///     defaults.brightness = Some(1.0);
    ///     
    ///     let mut state_1 = lifx_rs::State::new();
    ///     state_1.selector = Some(format!("id:xxx"));
    ///     state_1.power = Some(format!("on"));
    ///     
    ///     let mut state_2 = lifx_rs::State::new();
    ///     state_2.selector = Some(format!("id:xyz"));
    ///     state_2.power = Some(format!("on"));
    ///     
    ///     set_states.states = Some(states);
    ///     set_states.defaults = Some(defaults);
    ///     
    ///     lifx_rs::Light::async_set_states(key.clone(), set_states).await;
    /// }
    ///  ```
    pub async fn async_set_states(access_token: String, states: States) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("https://api.lifx.com/v1/lights/state");

        let request = reqwest::blocking::Client::new().put(url)
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&states)
            .send()?;
    
        let json = request.json::<LiFxResults>()?;
        return Ok(json);
    }

    /// Asynchronously set parameters other than power and duration change the state of the lights by the amount specified.
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `delta` - A StateDelta object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let mut delta = lifx_rs::StateDelta::new();
    ///     delta.duration = Some(0);
    ///     delta.power = Some(format!("on"));
    ///     
    ///     // Send StateDelta
    ///     lifx_rs::Light::async_state_delta_by_selector(key.clone(), format!("all"), toggle).await;
    /// }
    ///  ```
    pub async fn async_state_delta_by_selector(access_token: String, selector: String, delta: StateDelta) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("https://api.lifx.com/v1/lights/{}/state/delta", selector);

        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", access_token))
            .form(&delta.to_params())
            .send().await?;
    
        let json = request.json::<LiFxResults>().await?;
        return Ok(json);
    }



    /// Turn off light if on, or turn them on if it is off. 
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `clean` - A Clean object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let all_lights = lifx_rs::Light::list_all(key.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut toggle = lifx_rs::Toggle::new();
    ///             toggle.duration = Some(0);
    ///         
    ///             for light in lights {
    ///                 let results = light.toggle(key.clone(), clean.clone());
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub async fn async_toggle(&self, access_token: String, toggle: Toggle) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_toggle_by_selector(access_token, format!("id:{}", self.id), toggle).await;
    }

    /// Turn off lights if any of them are on, or turn them on if they are all off. 
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `clean` - A Clean object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let mut toggle = lifx_rs::Toggle::new();
    ///     toggle.duration = Some(0);
    ///     
    ///     // Toggle all lights
    ///     lifx_rs::Light::toggle_by_selector(key.clone(), format!("all"), toggle);
    /// }
    ///  ```
    pub async fn async_toggle_by_selector(access_token: String, selector: String, toggle: Toggle) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("https://api.lifx.com/v1/lights/{}/toggle", selector);

        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", access_token))
            .form(&toggle.to_params())
            .send().await?;
    
        let json = request.json::<LiFxResults>().await?;
        return Ok(json);
    }

    // =======================================
    // END OF ASYNC FUNCTIONS
    // =======================================

    // =======================================
    // BEGINING OF SYNC FUNCTIONS
    // =======================================

    /// This endpoint lets you switch a light to clean mode, with a set duration. 
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `clean` - A Clean object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let all_lights = lifx_rs::Light::list_all(key.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut clean = lifx_rs::Clean::new();
    ///             clean.duration = Some(0);
    ///             clean.stop = Some(false);
    ///         
    ///             for light in lights {
    ///                 let results = light.clean(key.clone(), clean.clone());
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub fn clean(&self, access_token: String, clean: Clean) ->  Result<LiFxResults, reqwest::Error>{
        return Self::clean_by_selector(access_token, format!("id:{}", self.id), clean);
    }

    /// This endpoint lets you switch a selected LIFX object to clean mode, with a set duration. 
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `clean` - A Clean object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let mut clean = lifx_rs::Clean::new();
    ///     clean.duration = Some(0);
    ///     clean.stop = Some(false);
    ///     
    ///     // Set all light to clean mode
    ///     lifx_rs::Light::clean_by_selector(key.clone(), format!("all"), clean);
    /// }
    ///  ```
    pub fn clean_by_selector(access_token: String, selector: String, clean: Clean) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("https://api.lifx.com/v1/lights/{}/clean", selector);

        let request = reqwest::blocking::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", access_token))
            .form(&clean.to_params())
            .send()?;
    
        let json = request.json::<LiFxResults>()?;
        return Ok(json);
    }

    /// Gets ALL lights belonging to the authenticated account
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let all_lights = lifx_rs::Light::list_all(key)?;
    /// }
    ///  ```
    pub fn list_all(access_token: String) -> Result<Lights, reqwest::Error> {
        return Self::list_by_selector(access_token, format!("all"));
    }

    /// Gets lights belonging to the authenticated account. Filtering the lights using selectors. Properties such as id, label, group and location can be used in selectors.
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let all_lights = lifx_rs::Light::list_by_selector(key, format!("all"))?;
    /// }
    ///  ```
    pub fn list_by_selector(access_token: String, selector: String) -> Result<Lights, reqwest::Error> {
        let mut url = format!("https://api.lifx.com/v1/lights/{}", selector);
        let request = reqwest::blocking::Client::new().get(url).header("Authorization", format!("Bearer {}", access_token)).send()?;
        let json = request.json::<Lights>()?;
        return Ok(json);
    }



    /// Sets the state for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `state` - A State object containing the values of the State to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let all_lights = lifx_rs::Light::list_all(key.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut state = lifx_rs::State::new();
    ///             state.power = Some(format!("on"));
    ///             state.brightness = Some(1.0);
    ///         
    ///             for light in lights {
    ///                 let results = light.set_state(key.clone(), state.clone());
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub fn set_state(&self, access_token: String, state: State) ->  Result<LiFxResults, reqwest::Error>{
        return Self::set_state_by_selector(access_token, format!("id:{}", self.id), state);
    }

    /// Sets the state for the selected LIFX object
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `state` - A State object containing the values of the State to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let mut off_state = lifx_rs::State::new();
    ///     off_state.power = Some(format!("off"));
    ///     
    ///     // Turn off all lights
    ///     lifx_rs::Light::set_state_by_selector(key.clone(), format!("all"), off_state);
    /// }
    ///  ```
    pub fn set_state_by_selector(access_token: String, selector: String, state: State) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("https://api.lifx.com/v1/lights/{}/state", selector);

        let request = reqwest::blocking::Client::new().put(url)
            .header("Authorization", format!("Bearer {}", access_token))
            .form(&state.to_params())
            .send()?;
    
        let json = request.json::<LiFxResults>()?;
        return Ok(json);
    }

    /// Sets the state for the selected LIFX object
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `states` - A vector of States with defaults
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let mut set_states = lifx_rs::States::new();
    ///     let mut states: Vec<lifx_rs::State> = Vec::new();
    ///     let mut defaults = lifx_rs::State::new();
    ///     
    ///     defaults.brightness = Some(1.0);
    ///     
    ///     let mut state_1 = lifx_rs::State::new();
    ///     state_1.selector = Some(format!("id:xxx"));
    ///     state_1.power = Some(format!("on"));
    ///     
    ///     let mut state_2 = lifx_rs::State::new();
    ///     state_2.selector = Some(format!("id:xyz"));
    ///     state_2.power = Some(format!("on"));
    ///     
    ///     set_states.states = Some(states);
    ///     set_states.defaults = Some(defaults);
    ///     
    ///     lifx_rs::Light::set_states(key.clone(), set_states);
    /// }
    ///  ```
    pub fn set_states(access_token: String, states: States) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("https://api.lifx.com/v1/lights/state");

        let request = reqwest::blocking::Client::new().put(url)
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&states)
            .send()?;
    
        let json = request.json::<LiFxResults>()?;
        return Ok(json);
    }

    /// Set parameters other than power and duration change the state of the lights by the amount specified.
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `delta` - A StateDelta object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let mut delta = lifx_rs::StateDelta::new();
    ///     delta.duration = Some(0);
    ///     delta.power = Some(format!("on"));
    ///     
    ///     // Send StateDelta
    ///     lifx_rs::Light::state_delta_by_selector(key.clone(), format!("all"), toggle);
    /// }
    ///  ```
    pub fn state_delta_by_selector(access_token: String, selector: String, delta: StateDelta) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("https://api.lifx.com/v1/lights/{}/state/delta", selector);

        let request = reqwest::blocking::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", access_token))
            .form(&delta.to_params())
            .send()?;
    
        let json = request.json::<LiFxResults>()?;
        return Ok(json);
    }


    /// Turn off light if on, or turn them on if it is off. 
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `clean` - A Clean object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let all_lights = lifx_rs::Light::list_all(key.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut toggle = lifx_rs::Toggle::new();
    ///             toggle.duration = Some(0);
    ///         
    ///             for light in lights {
    ///                 let results = light.toggle(key.clone(), clean.clone());
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub fn toggle(&self, access_token: String, toggle: Toggle) ->  Result<LiFxResults, reqwest::Error>{
        return Self::toggle_by_selector(access_token, format!("id:{}", self.id), toggle);
    }

    /// Turn off lights if any of them are on, or turn them on if they are all off. 
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `clean` - A Clean object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    /// 
    ///     let mut toggle = lifx_rs::Toggle::new();
    ///     toggle.duration = Some(0);
    ///     
    ///     // Toggle all lights
    ///     lifx_rs::Light::toggle_by_selector(key.clone(), format!("all"), toggle);
    /// }
    ///  ```
    pub fn toggle_by_selector(access_token: String, selector: String, toggle: Toggle) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("https://api.lifx.com/v1/lights/{}/toggle", selector);

        let request = reqwest::blocking::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", access_token))
            .form(&toggle.to_params())
            .send()?;
    
        let json = request.json::<LiFxResults>()?;
        return Ok(json);
    }
}

pub type Scenes = Vec<Scene>;

/// Represents an LIFX Scene
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scene {
    pub uuid: String,
    pub name: String,
    pub account: Account,
    pub states: Vec<State>,
    #[serde(rename = "created_at")]
    pub created_at: i64,
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
    pub error: Option<String>,
    pub errors: Option<Vec<Error>>,
}
impl Scene {
    pub async fn async_list(access_token: String) -> Result<Scenes, reqwest::Error> {
        let mut url = format!("https://api.lifx.com/v1/scenes");
        let request = reqwest::Client::new().get(url).header("Authorization", format!("Bearer {}", access_token)).send().await?;
        let json = request.json::<Scenes>().await?;
        return Ok(json);
    }

    pub fn list(access_token: String) -> Result<Scenes, reqwest::Error> {
        let mut url = format!("https://api.lifx.com/v1/scenes");
        let request = reqwest::blocking::Client::new().get(url).header("Authorization", format!("Bearer {}", access_token)).send()?;
        let json = request.json::<Scenes>()?;
        return Ok(json);
    }
}

/// Represents an LIFX Color
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub hue: Option<i64>,
    pub saturation: Option<i64>,
    pub kelvin: Option<i64>,
    pub brightness: Option<i64>,
    pub error: Option<String>,
    pub errors: Option<Vec<Error>>,
}
impl Color {
    pub async fn async_validate(access_token: String, color: String) -> Result<Color, reqwest::Error> {
        let mut url = format!("https://api.lifx.com/v1/color?string={}", color);
        let request = reqwest::Client::new().get(url).header("Authorization", format!("Bearer {}", access_token)).send().await?;
        let json = request.json::<Color>().await?;
        return Ok(json);
    }

    pub fn validate(access_token: String, color: String) -> Result<Color, reqwest::Error> {
        let mut url = format!("https://api.lifx.com/v1/color?string={}", color);
        let request = reqwest::blocking::Client::new().get(url).header("Authorization", format!("Bearer {}", access_token)).send()?;
        let json = request.json::<Color>()?;
        return Ok(json);
    }
}

/// Used to set the duration/state of the HEV Clean array
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clean {
    pub stop: Option<bool>,
    pub duration: Option<i64>
}
impl Clean {
    pub fn new() -> Self {
        return Clean{
            stop: None,
            duration: None
        };
    }

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];
        match &self.stop{
            Some(stop) => params.push(("stop".to_string(), stop.to_string())),
            None => {}
        }
        match &self.duration{
            Some(duration) => params.push(("duration".to_string(), duration.to_string())),
            None => {}
        }
       
        return params;
    }


}

/// Used to descripe the state of an LIFX Light Source
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub power: Option<String>,
    pub color: Option<String>,
    pub brightness: Option<f64>,
    pub duration: Option<f64>,
    pub infrared: Option<f64>,
    pub selector:  Option<String>,
    pub fast: Option<bool>
}
impl State {
    pub fn new() -> Self {
        return State{
            power: None,
            color: None,
            brightness: None,
            duration: None,
            infrared: None,
            selector: None,
            fast: None
        };
    }

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];
        match &self.power{
            Some(power) => params.push(("power".to_string(), power.to_string())),
            None => {}
        }
        match &self.color{
            Some(color) => params.push(("color".to_string(), color.to_string())),
            None => {}
        }
        match &self.brightness{
            Some(brightness) => params.push(("brightness".to_string(), brightness.to_string())),
            None => {}
        }
        match &self.duration{
            Some(duration) => params.push(("duration".to_string(), duration.to_string())),
            None => {}
        }
        match &self.infrared{
            Some(infrared) => params.push(("infrared".to_string(), infrared.to_string())),
            None => {}
        }
        match &self.selector{
            Some(selector) => params.push(("selector".to_string(), selector.to_string())),
            None => {}
        }
        match &self.fast{
            Some(fast) => params.push(("fast".to_string(), fast.to_string())),
            None => {}
        }
        return params;
    }


}

/// Used to set the duration of a Toggle event
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toggle {
    pub duration: Option<i64>
}
impl Toggle {
    pub fn new() -> Self {
        return Toggle{
            duration: None
        };
    }

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];
        match &self.duration{
            Some(duration) => params.push(("duration".to_string(), duration.to_string())),
            None => {}
        }
        return params;
    }


}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct States {
    pub states: Option<Vec<State>>,
    pub defaults: Option<State>,
}
impl States {
    pub fn new() -> Self {
        return States{
            states: None,
            defaults: None
        };
    }

    // fn to_params(&self) -> Vec<(String, String)> {
    //     let mut params: Vec<(String, String)> = vec![];
    //     match &self.duration{
    //         Some(duration) => params.push(("duration".to_string(), duration.to_string())),
    //         None => {}
    //     }
    //     return params;
    // }


}

/// Defines parameters for StateDelta
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateDelta {
    /// The power state you want to set on the selector. on or off
    pub power: Option<String>,
    /// How long in seconds you want the power action to take. Range: 0.0 – 3155760000.0 (100 years)
    pub duration: Option<f64>,
    /// The maximum brightness of the infrared channel.
    pub infrared: Option<f64>,
    /// Rotate the hue by this angle in degrees. Range: -360.0 – 360.0 degrees
    pub hue: Option<f64>,
    /// Change the saturation by this additive amount; the resulting saturation is clipped to [0, 1].
    pub saturation: Option<f64>,
    /// Change the brightness by this additive amount; the resulting brightness is clipped to [0, 1].
    pub brightness: Option<f64>,
    /// Change the kelvin by this additive amount; the resulting kelvin is clipped to [2500, 9000].
    pub kelvin: Option<i64>,
    /// Execute the query fast, without initial state checks and wait for no results.
    pub fast: Option<bool>,
}
impl StateDelta {
    pub fn new() -> Self {
        return StateDelta{
            power: None,
            duration: None,
            infrared: None,
            hue: None,
            saturation: None,
            brightness: None,
            kelvin: None,
            fast: None
        };
    }

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];
        match &self.power{
            Some(power) => params.push(("power".to_string(), power.to_string())),
            None => {}
        }

        match &self.duration{
            Some(duration) => params.push(("duration".to_string(), duration.to_string())),
            None => {}
        }

        match &self.infrared{
            Some(infrared) => params.push(("infrared".to_string(), infrared.to_string())),
            None => {}
        }

        match &self.hue{
            Some(hue) => params.push(("hue".to_string(), hue.to_string())),
            None => {}
        }

        match &self.saturation{
            Some(saturation) => params.push(("saturation".to_string(), saturation.to_string())),
            None => {}
        }

        match &self.brightness{
            Some(brightness) => params.push(("brightness".to_string(), brightness.to_string())),
            None => {}
        }

        match &self.kelvin{
            Some(kelvin) => params.push(("kelvin".to_string(), kelvin.to_string())),
            None => {}
        }

        match &self.fast{
            Some(fast) => params.push(("fast".to_string(), fast.to_string())),
            None => {}
        }

        return params;
    }

}

/// Defines parameters for the BreatheEffect
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreatheEffect {
    /// The color to use for the breathe effect.
    pub color: Option<String>,
    /// The color to start the effect from. If this parameter is omitted then the color the bulb is currently set to is used instead.
    pub from_color: Option<String>,
    /// The time in seconds for one cycle of the effect.
    pub period: Option<f64>,
    /// The number of times to repeat the effect.
    pub cycles: Option<f64>,
    /// If false set the light back to its previous value when effect ends, if true leave the last effect color.
    pub persist: Option<bool>,
    /// If true, turn the bulb on if it is not already on.
    pub power_on: Option<bool>,
    /// Defines where in a period the target color is at its maximum. Minimum 0.0, maximum 1.0.
    pub peak: Option<f64>,
}
impl BreatheEffect {
    pub fn new() -> Self {
        return BreatheEffect{
            color: None,
            from_color: None,
            period: None,
            cycles: None,
            persist: None,
            power_on: None,
            peak: None
        };
    }

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];
        match &self.color{
            Some(color) => params.push(("color".to_string(), color.to_string())),
            None => {}
        }

        match &self.from_color{
            Some(from_color) => params.push(("from_color".to_string(), from_color.to_string())),
            None => {}
        }

        match &self.period{
            Some(period) => params.push(("period".to_string(), period.to_string())),
            None => {}
        }

        match &self.cycles{
            Some(cycles) => params.push(("cycles".to_string(), cycles.to_string())),
            None => {}
        }

        match &self.persist{
            Some(persist) => params.push(("persist".to_string(), persist.to_string())),
            None => {}
        }

        match &self.power_on{
            Some(power_on) => params.push(("power_on".to_string(), power_on.to_string())),
            None => {}
        }

        match &self.peak{
            Some(peak) => params.push(("peak".to_string(), peak.to_string())),
            None => {}
        }

        return params;
    }

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Group {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Location {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Product {
    pub name: String,
    pub identifier: String,
    pub company: String,
    #[serde(rename = "vendor_id")]
    pub vendor_id: i64,
    #[serde(rename = "product_id")]
    pub product_id: i64,
    pub capabilities: Capabilities,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Capabilities {
    #[serde(rename = "has_color")]
    pub has_color: bool,
    #[serde(rename = "has_variable_color_temp")]
    pub has_variable_color_temp: bool,
    #[serde(rename = "has_ir")]
    pub has_ir: bool,
    #[serde(rename = "has_hev")]
    pub has_hev: bool,
    #[serde(rename = "has_chain")]
    pub has_chain: bool,
    #[serde(rename = "has_matrix")]
    pub has_matrix: bool,
    #[serde(rename = "has_multizone")]
    pub has_multizone: bool,
    #[serde(rename = "min_kelvin")]
    pub min_kelvin: i64,
    #[serde(rename = "max_kelvin")]
    pub max_kelvin: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Account {
    pub uuid: String,
}




#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct Error {
    pub field: String,
    pub message: Vec<String>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct LiFxResults {
    pub results: Option<Vec<LiFxResult>>,
    pub error: Option<String>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc(hidden)]
pub struct LiFxResult {
    pub id: String,
    pub label: String,
    pub status: String,
}
