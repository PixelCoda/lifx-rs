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
//! A synchronous + asynchronous library for communicating with the official LIFX-API and the unoffical offline API. 
//!
//! ## LIFX-API Supported Methods:
//! * List Lights
//! * Set State
//! * Set States
//! * State Delta
//! * Toggle Power
//! * Breathe Effect
//! * Move Effect
//! * Morph Effect
//! * Flame Effect
//! * Pulse Effect
//! * Effects Off
//! * Clean (HEV)
//! * List Scenes
//! * Validate Color
//!
//! ## Unofficial Offline API Supported Methods:
//! * List Lights
//! * Set State
//! * Set States
//! 
//! ## To use offline use the Un-Official API Server:
//! [lifx-api-server](https://crates.io/crates/lifx-api-server)
//!
//! ## How to use library
//!
//! Add the following line to your cargo.toml:
//! ```
//! lifx-rs = "0.1.28"
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
//!     let mut api_endpoints: Vec<String> = Vec::new();
//!  
//!     // Official API
//!     api_endpoints.push(format!("https://api.lifx.com"));
//!
//!     // lifx-server-api (Un-Official)
//!     api_endpoints.push(format!("http://localhost:8089"));
//!
//!     let config = lifx::LifxConfig{
//!         access_token: key.clone(),
//!         api_endpoints: api_endpoints
//!     };
//!
//!     // Build an "OffState" to set
//!     let mut off_state = lifx::State::new();
//!     off_state.power = Some(format!("off"));
//!
//!     // Turn off all lights
//!     lifx::Light::set_state_by_selector(config.clone(), format!("all"), off_state);
//!
//!     let all_lights = lifx::Light::list_all(config.clone());
//!     match all_lights {
//!         Ok(lights) => {
//!             println!("{:?}",lights.clone());
//!
//!             let mut state = lifx::State::new();
//!             state.power = Some(format!("on"));
//!             state.brightness = Some(1.0);
//!      
//!             for light in lights {
//!                 let results = light.set_state(config.clone(), state.clone());
//!                 println!("{:?}",results);
//!             }
//!         },
//!         Err(e) => println!("{}",e)
//!     }
//!
//! }
//!
//! ```
//!
//!
//! Async Example:
//! ```rust
//! extern crate lifx_rs as lifx;
//!
//! #[tokio::main]
//! async fn main() {
//!
//!     let key = "xxx".to_string();
//!  
//!     let mut api_endpoints: Vec<String> = Vec::new();
//!  
//!     // Official API
//!     api_endpoints.push(format!("https://api.lifx.com"));
//!
//!     // lifx-server-api (Un-Official)
//!     api_endpoints.push(format!("http://localhost:8089"));
//!
//!     let config = lifx::LifxConfig{
//!         access_token: key.clone(),
//!         api_endpoints: api_endpoints
//!     };
//!
//!     // Build "OffState" to set
//!     let mut off_state = lifx::State::new();
//!     off_state.power = Some(format!("off"));
//!  
//!     // Turn off all lights
//!     lifx::Light::async_set_state_by_selector(config.clone(), format!("all"), off_state).await;
//! }
//! ```
//!
//!
//! ## License
//!
//! Released under Apache 2.0 or MIT.
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
//!  * ADA: addr1qyp299a45tgvveh83tcxlf7ds3yaeh969yt3v882lvxfkkv4e0f46qvr4wzj8ty5c05jyffzq8a9pfwz9dl6m0raac7s4rac48
//!  * ALGO: VQ5EK4GA3IUTGSPNGV64UANBUVFAIVBXVL5UUCNZSDH544XIMF7BAHEDM4
//!  * ATOM: cosmos1wm7lummcealk0fxn3x9tm8hg7xsyuz06ul5fw9
//!  * BTC: bc1qh5p3rff4vxnv23vg0hw8pf3gmz3qgc029cekxz
//!  * ETH: 0x7A66beaebF7D0d17598d37525e63f524CfD23452
//!  * ERC20: 0x7A66beaebF7D0d17598d37525e63f524CfD23452
//!  * XLM: GCJAUMCO2L7PTYMXELQ6GHBTF25MCQKEBNSND2C4QMUPTSVCPEN3LCOG
//!  * XTZ: tz1SgJppPn56whprsDDGcqR4fxqCr2PXvg1R

pub mod lan;



use serde::{Serialize, Deserialize};





/// Represents a LIFX Config Object
/// Supports two api_endpoints.....if the first one fails...falls back on second api
/// TODO - Support unlimited api_endpoints
/// TODO - Use multithreaded timeout to detect primary api failures faster
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifxConfig {
    pub access_token: String,
    pub api_endpoints: Vec<String>,
}


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

    /// Asynchronously set the breathe animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `breathe` - A BreatheEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_all(config.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut breathe = lifx::BreatheEffect::new();
    ///             breathe.color = Some(format!("red"));
    ///             breathe.from_color = Some(format!("green"));
    ///             breathe.period = Some(10);
    ///             breathe.persist = Some(true);
    ///             breathe.power_on = Some(true);
    ///         
    ///             for light in lights {
    ///                 let results = light.async_breathe_effect(key.clone(), breathe.clone()).await;
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub async fn async_breathe_effect(&self, config: LifxConfig, breathe: BreatheEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_breathe_effect_by_selector(config, format!("id:{}", self.id), breathe).await;
    }

    /// Asynchronously activate the breathe animation for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `breathe` - A BreatheEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut breathe = lifx::BreatheEffect::new();
    ///     breathe.color = Some(format!("red"));
    ///     breathe.from_color = Some(format!("green"));
    ///     breathe.period = Some(10);
    ///     breathe.persist = Some(true);
    ///     breathe.power_on = Some(true);
    ///     
    ///     // Apply breathe effect to all light(s)
    ///     lifx::Light::async_breathe_effect_by_selector(key.clone(), format!("all"), breathe).await;
    /// }
    ///  ```
    pub async fn async_breathe_effect_by_selector(config: LifxConfig, selector: String, breathe: BreatheEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/breathe", config.api_endpoints[0], selector);

        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&breathe.to_params())
            .send().await;
            
        match request{
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/breathe", config.api_endpoints[1], selector);

                    let request = reqwest::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&breathe.to_params())
                        .send().await;
                        
                    match request{
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }


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
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_all(config.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut clean = lifx::Clean::new();
    ///             clean.duration = Some(0);
    ///             clean.stop = Some(false);
    ///         
    ///             for light in lights {
    ///                 let results = light.async_clean(key.clone(), clean.clone()).await;
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub async fn async_clean(&self, config: LifxConfig, clean: Clean) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_clean_by_selector(config, format!("id:{}", self.id), clean).await;
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
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut clean = lifx::Clean::new();
    ///     clean.duration = Some(0);
    ///     clean.stop = Some(false);
    ///     
    ///     // Set all light to clean mode
    ///     lifx::Light::async_clean_by_selector(key.clone(), format!("all"), clean).await;
    /// }
    ///  ```
    pub async fn async_clean_by_selector(config: LifxConfig, selector: String, clean: Clean) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/clean", config.api_endpoints[0], selector);

        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&clean.to_params())
            .send().await;

        match request{
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/clean", config.api_endpoints[1], selector);

                    let request = reqwest::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&clean.to_params())
                        .send().await;
            
                    match request{
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }


    /// Stops animation(s) for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `flame_effect` - A FlameEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_all(config.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut effects_off = lifx::EffectsOff::new();
    ///             effects_off.power_off = Some(true);
    ///         
    ///             for light in lights {
    ///                 let results = light.async_effects_off(key.clone(), effects_off.clone()).await;
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub async fn async_effects_off(&self, config: LifxConfig, effects_off: EffectsOff) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_effects_off_by_selector(config, format!("id:{}", self.id), effects_off).await;
    }

    /// Stops animation(s) for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `effects_off` - A EffectsOff object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut effects_off = lifx::EffectsOff::new();
    ///     effects_off.power_off = Some(true);
    ///     
    ///     // Send morph effect to all lights
    ///     lifx::Light::async_effects_off_by_selector(key.clone(), format!("all"), effects_off).await;
    /// }
    ///  ```
    pub async fn async_effects_off_by_selector(config: LifxConfig, selector: String, effects_off: EffectsOff) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/off", config.api_endpoints[0], selector);

        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&effects_off.to_params())
            .send().await;

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/off", config.api_endpoints[1], selector);

                    let request = reqwest::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&effects_off.to_params())
                        .send().await;
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                
                } else {
                    return Err(err);
                }
            }
        }
    

    }



    /// Activate the flame animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `flame_effect` - A FlameEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_all(config.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut flame_effect = lifx::FlameEffect::new();
    ///             flame_effect.period = Some(10);
    ///             flame_effect.duration = Some(0);
    ///             flame_effect.power_on = Some(true);
    ///         
    ///             for light in lights {
    ///                 let results = light.async_flame_effect(key.clone(), flame_effect.clone()).await;
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub async fn async_flame_effect(&self, config: LifxConfig, flame_effect: FlameEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_flame_effect_by_selector(config, format!("id:{}", self.id), flame_effect).await;
    }

    /// Activate the flame animation for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `flame_effect` - A FlameEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut flame_effect = lifx::FlameEffect::new();
    ///     flame_effect.period = Some(10);
    ///     flame_effect.duration = Some(0);
    ///     flame_effect.power_on = Some(true);
    ///     
    ///     // Send morph effect to all lights
    ///     lifx::Light::async_flame_effect_by_selector(key.clone(), format!("all"), flame_effect).await;
    /// }
    ///  ```
    pub async fn async_flame_effect_by_selector(config: LifxConfig, selector: String, flame_effect: FlameEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/flame", config.api_endpoints[0], selector);

        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&flame_effect.to_params())
            .send().await;

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/flame", config.api_endpoints[1], selector);

                    let request = reqwest::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&flame_effect.to_params())
                        .send().await;
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

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
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::async_list_all(config).await?;
    /// }
    ///  ```
    pub async fn async_list_all(config: LifxConfig) -> Result<Lights, reqwest::Error> {
        return Self::async_list_by_selector(config, format!("all")).await;
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
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::async_list_by_selector(key, format!("all")).await?;
    /// }
    ///  ```
    pub async fn async_list_by_selector(config: LifxConfig, selector: String) -> Result<Lights, reqwest::Error> {
        let url = format!("{}/v1/lights/{}", config.api_endpoints[0], selector);
        let request = reqwest::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send().await;
        match request {
            Ok(req) => {
                let json = req.json::<Lights>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}", config.api_endpoints[1], selector);
                    let request = reqwest::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send().await;
                    match request {
                        Ok(req) => {
                            let json = req.json::<Lights>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    }

    /// Asynchronously activate the morph animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `morph_effect` - A MorphEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_all(config.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut morph_effect = lifx::MorphEffect::new();
    ///             morph_effect.period = Some(10);
    ///             morph_effect.duration = Some(0);
    /// 
    ///             let mut palette: Vec<String> = Vec::new();
    ///             palette.push(format!("red"));
    ///             palette.push(format!("green"));
    /// 
    ///             morph_effect.palette = Some(palette);
    ///             morph_effect.power_on = Some(true);
    ///         
    ///             for light in lights {
    ///                 let results = light.async_morph_effect(key.clone(), morph_effect.clone()).await;
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub async fn async_morph_effect(&self, config: LifxConfig, morph_effect: MorphEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_morph_effect_by_selector(config, format!("id:{}", self.id), morph_effect).await;
    }

    /// Asynchronously activate the morph animation for the selected light(s)
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
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut morph_effect = lifx::MorphEffect::new();
    ///     morph_effect.period = Some(10);
    ///     morph_effect.duration = Some(0);
    /// 
    ///     let mut palette: Vec<String> = Vec::new();
    ///     palette.push(format!("red"));
    ///     palette.push(format!("green"));
    /// 
    ///     morph_effect.palette = Some(palette);
    ///     morph_effect.power_on = Some(true);
    ///     
    ///     // Send morph effect to all lights
    ///     lifx::Light::async_morph_effect_by_selector(key.clone(), format!("all"), morph_effect).await;
    /// }
    ///  ```
    pub async fn async_morph_effect_by_selector(config: LifxConfig, selector: String, morph_effect: MorphEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/morph", config.api_endpoints[0], selector);
        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&morph_effect.to_params())
            .send().await;
        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/morph", config.api_endpoints[1], selector);
                    let request = reqwest::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&morph_effect.to_params())
                        .send().await;
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }

    /// Asynchronously activate the move animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `move_effect` - A MoveEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_all(config.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut move_effect = lifx::MoveEffect::new();
    ///             move_effect.direction = Some(format!("forward")); // or backward
    ///             move_effect.period = Some(10);
    ///             move_effect.cycles = Some(0.9);
    ///             move_effect.power_on = Some(true);
    ///         
    ///             for light in lights {
    ///                 let results = light.async_move_effect(key.clone(), move_effect.clone()).await;
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub async fn async_move_effect(&self, config: LifxConfig, move_effect: MoveEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_move_effect_by_selector(config, format!("id:{}", self.id), move_effect).await;
    }

    /// Asynchronously activate the move animation for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `move_effect` - A MoveEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut move_effect = lifx::MoveEffect::new();
    ///     move_effect.direction = Some(format!("forward")); // or backward
    ///     move_effect.period = Some(10);
    ///     move_effect.cycles = Some(0.9);
    ///     move_effect.power_on = Some(true);
    ///     
    ///     // Toggle all lights
    ///     lifx::Light::async_move_effect_by_selector(key.clone(), format!("all"), move_effect).await;
    /// }
    ///  ```
    pub async fn async_move_effect_by_selector(config: LifxConfig, selector: String, move_effect: MoveEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/move", config.api_endpoints[0], selector);

        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&move_effect.to_params())
            .send().await;

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/move", config.api_endpoints[1], selector);

                    let request = reqwest::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&move_effect.to_params())
                        .send().await;
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }

    /// Asynchronously activate the pulse animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `pulse_effect` - A PulseEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_all(config.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut pulse = lifx::PulseEffect::new();
    ///             pulse.color = Some(format!("red"));
    ///             pulse.from_color = Some(format!("green"));
    ///             pulse.period = Some(10);
    ///             pulse.persist = Some(true);
    ///             pulse.power_on = Some(true);
    ///         
    ///             for light in lights {
    ///                 let results = light.async_pulse_effect(key.clone(), pulse.clone()).await;
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub async fn async_pulse_effect(&self, config: LifxConfig, pulse_effect: PulseEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_pulse_effect_by_selector(config, format!("id:{}", self.id), pulse_effect).await;
    }

    /// Asynchronously activate the pulse animation for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `pulse_effect` - A PulseEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut pulse = lifx::PulseEffect::new();
    ///     pulse.color = Some(format!("red"));
    ///     pulse.from_color = Some(format!("green"));
    ///     pulse.period = Some(10);
    ///     pulse.persist = Some(true);
    ///     pulse.power_on = Some(true);
    ///     
    ///     // Toggle all lights
    ///     lifx::Light::async_pulse_effect_by_selector(key.clone(), format!("all"), pulse).await;
    /// }
    ///  ```
    pub async fn async_pulse_effect_by_selector(config: LifxConfig, selector: String, pulse_effect: PulseEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/pulse", config.api_endpoints[0], selector);

        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&pulse_effect.to_params())
            .send().await;

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/pulse", config.api_endpoints[1], selector);

                    let request = reqwest::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&pulse_effect.to_params())
                        .send().await;
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                
            
                } else {
                    return Err(err);
                }
            }
        }
    

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
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_all(config.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut state = lifx::State::new();
    ///             state.power = Some(format!("on"));
    ///             state.brightness = Some(1.0);
    ///         
    ///             for light in lights {
    ///                 let results = light.async_set_state(key.clone(), state.clone()).await;
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub async fn async_set_state(&self, config: LifxConfig, state: State) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_set_state_by_selector(config, format!("id:{}", self.id), state).await;
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
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut off_state = lifx::State::new();
    ///     off_state.power = Some(format!("off"));
    ///     
    ///     // Turn off all lights
    ///     lifx::Light::async_set_state_by_selector(key.clone(), format!("all"), off_state).await;
    /// }
    ///  ```
    pub async fn async_set_state_by_selector(config: LifxConfig, selector: String, state: State) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/state", config.api_endpoints[0], selector);

        let request = reqwest::Client::new().put(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&state.to_params())
            .send().await;

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/state", config.api_endpoints[0], selector);

                    let request = reqwest::Client::new().put(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&state.to_params())
                        .send().await;
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                          return Err(err2);  
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

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
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut set_states = lifx::States::new();
    ///     let mut states: Vec<lifx::State> = Vec::new();
    ///     let mut defaults = lifx::State::new();
    ///     
    ///     defaults.brightness = Some(1.0);
    ///     
    ///     let mut state_1 = lifx::State::new();
    ///     state_1.selector = Some(format!("id:xxx"));
    ///     state_1.power = Some(format!("on"));
    ///     
    ///     let mut state_2 = lifx::State::new();
    ///     state_2.selector = Some(format!("id:xyz"));
    ///     state_2.power = Some(format!("on"));
    ///     
    ///     set_states.states = Some(states);
    ///     set_states.defaults = Some(defaults);
    ///     
    ///     lifx::Light::async_set_states(key.clone(), set_states).await;
    /// }
    ///  ```
    pub async fn async_set_states(config: LifxConfig, states: States) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/state", config.api_endpoints[0]);

        let request = reqwest::blocking::Client::new().put(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .json(&states)
            .send();

        match request{
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(e) => {
                if config.api_endpoints.len() > 1 {

                    let url = format!("{}/v1/lights/state", config.api_endpoints[1]);

                    let request = reqwest::blocking::Client::new().put(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .json(&states)
                        .send();
            
                    match request{
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(e2) => {
                            return Err(e2);
                        }
                    }


                } else {
                    return Err(e);
                }
            }
        }
    

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
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut delta = lifx::StateDelta::new();
    ///     delta.duration = Some(0);
    ///     delta.power = Some(format!("on"));
    ///     
    ///     // Send StateDelta
    ///     lifx::Light::async_state_delta_by_selector(key.clone(), format!("all"), toggle).await;
    /// }
    ///  ```
    pub async fn async_state_delta_by_selector(config: LifxConfig, selector: String, delta: StateDelta) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/state/delta", config.api_endpoints[0], selector);

        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&delta.to_params())
            .send().await;

        match request{
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/state/delta", config.api_endpoints[1], selector);

                    let request = reqwest::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&delta.to_params())
                        .send().await;
            
                    match request{
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2)
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

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
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_all(config.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut toggle = lifx::Toggle::new();
    ///             toggle.duration = Some(0);
    ///         
    ///             for light in lights {
    ///                 let results = light.async_toggle(key.clone(), clean.clone()).await;
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub async fn async_toggle(&self, config: LifxConfig, toggle: Toggle) ->  Result<LiFxResults, reqwest::Error>{
        return Self::async_toggle_by_selector(config, format!("id:{}", self.id), toggle).await;
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
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut toggle = lifx_rs::Toggle::new();
    ///     toggle.duration = Some(0);
    ///     
    ///     // Toggle all lights
    ///     lifx_rs::Light::async_toggle_by_selector(key.clone(), format!("all"), toggle).await?;
    /// }
    ///  ```
    pub async fn async_toggle_by_selector(config: LifxConfig, selector: String, toggle: Toggle) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/toggle", config.api_endpoints[0], selector);

        let request = reqwest::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&toggle.to_params())
            .send().await;

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/toggle", config.api_endpoints[1], selector);

                    let request = reqwest::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&toggle.to_params())
                        .send().await;
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }

    // =======================================
    // END OF ASYNC FUNCTIONS
    // =======================================

    // =======================================
    // BEGINING OF SYNC FUNCTIONS
    // =======================================

    /// Set the breathe animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `breathe` - A BreatheEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_all(config.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut breathe = lifx::BreatheEffect::new();
    ///             breathe.color = Some(format!("red"));
    ///             breathe.from_color = Some(format!("green"));
    ///             breathe.period = Some(10);
    ///             breathe.persist = Some(true);
    ///             breathe.power_on = Some(true);
    ///         
    ///             for light in lights {
    ///                 let results = light.breathe_effect(key.clone(), breathe.clone());
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub fn breathe_effect(&self, config: LifxConfig, breathe: BreatheEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::breathe_by_selector_effect(config, format!("id:{}", self.id), breathe);
    }

    /// Activate the breathe animation for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `breathe` - A BreatheEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut breathe = lifx::BreatheEffect::new();
    ///     breathe.color = Some(format!("red"));
    ///     breathe.from_color = Some(format!("green"));
    ///     breathe.period = Some(10);
    ///     breathe.persist = Some(true);
    ///     breathe.power_on = Some(true);
    ///     
    ///     // Apply breathe effect to all light(s)
    ///     lifx::Light::breathe_by_selector_effect(key.clone(), format!("all"), breathe);
    /// }
    ///  ```
    pub fn breathe_by_selector_effect(config: LifxConfig, selector: String, breathe: BreatheEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/breathe", config.api_endpoints[0], selector);

        let request = reqwest::blocking::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&breathe.to_params())
            .send();

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(e) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/breathe", config.api_endpoints[1], selector);

                    let request = reqwest::blocking::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&breathe.to_params())
                        .send();
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(e2) => {
                            return Err(e2);
                        }
                    }
                } else {
                    return Err(e);
                }
            }
        }
    

    }

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
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_all(config.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut clean = lifx::Clean::new();
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
    pub fn clean(&self, config: LifxConfig, clean: Clean) ->  Result<LiFxResults, reqwest::Error>{
        return Self::clean_by_selector(config, format!("id:{}", self.id), clean);
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
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut clean = lifx::Clean::new();
    ///     clean.duration = Some(0);
    ///     clean.stop = Some(false);
    ///     
    ///     // Set all light to clean mode
    ///     lifx::Light::clean_by_selector(key.clone(), format!("all"), clean);
    /// }
    ///  ```
    pub fn clean_by_selector(config: LifxConfig, selector: String, clean: Clean) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/clean", config.api_endpoints[0], selector);

        let request = reqwest::blocking::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&clean.to_params())
            .send();

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/clean", config.api_endpoints[1], selector);

                    let request = reqwest::blocking::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&clean.to_params())
                        .send();
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }

    /// Stops animation(s) for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `flame_effect` - A FlameEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_all(config.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut effects_off = lifx::EffectsOff::new();
    ///             effects_off.power_off = Some(true);
    ///         
    ///             for light in lights {
    ///                 let results = light.effects_off(key.clone(), effects_off.clone());
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub fn effects_off(&self, config: LifxConfig, effects_off: EffectsOff) ->  Result<LiFxResults, reqwest::Error>{
        return Self::effects_off_by_selector(config, format!("id:{}", self.id), effects_off);
    }

    /// Stops animation(s) for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `effects_off` - A EffectsOff object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut effects_off = lifx::EffectsOff::new();
    ///     effects_off.power_off = Some(true);
    ///     
    ///     // Send morph effect to all lights
    ///     lifx::Light::effects_off_by_selector(key.clone(), format!("all"), effects_off);
    /// }
    ///  ```
    pub fn effects_off_by_selector(config: LifxConfig, selector: String, effects_off: EffectsOff) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/off", config.api_endpoints[0], selector);

        let request = reqwest::blocking::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&effects_off.to_params())
            .send();

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/off", config.api_endpoints[1], selector);

                    let request = reqwest::blocking::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&effects_off.to_params())
                        .send();
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

    }

    /// Activate the flame animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `flame_effect` - A FlameEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_all(config.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut flame_effect = lifx::FlameEffect::new();
    ///             flame_effect.period = Some(10);
    ///             flame_effect.duration = Some(0);
    ///             flame_effect.power_on = Some(true);
    ///         
    ///             for light in lights {
    ///                 let results = light.flame_effect(key.clone(), flame_effect.clone());
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub fn flame_effect(&self, config: LifxConfig, flame_effect: FlameEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::flame_effect_by_selector(config, format!("id:{}", self.id), flame_effect);
    }

    /// Activate the flame animation for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `flame_effect` - A FlameEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut flame_effect = lifx::FlameEffect::new();
    ///     flame_effect.period = Some(10);
    ///     flame_effect.duration = Some(0);
    ///     flame_effect.power_on = Some(true);
    ///     
    ///     // Send morph effect to all lights
    ///     lifx::Light::flame_effect_by_selector(key.clone(), format!("all"), flame_effect);
    /// }
    ///  ```
    pub fn flame_effect_by_selector(config: LifxConfig, selector: String, flame_effect: FlameEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/flame", config.api_endpoints[0], selector);

        let request = reqwest::blocking::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&flame_effect.to_params())
            .send();

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/flame", config.api_endpoints[1], selector);

                    let request = reqwest::blocking::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&flame_effect.to_params())
                        .send();
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

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
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_all(config)?;
    /// }
    ///  ```
    pub fn list_all(config: LifxConfig) -> Result<Lights, reqwest::Error> {
        return Self::list_by_selector(config, format!("all"));
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
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_by_selector(key, format!("all"))?;
    /// }
    ///  ```
    pub fn list_by_selector(config: LifxConfig, selector: String) -> Result<Lights, reqwest::Error> {
        let url = format!("{}/v1/lights/{}", config.api_endpoints[0], selector);
        let request = reqwest::blocking::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send();
        match request {
            Ok(req) => {
                let json = req.json::<Lights>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}", config.api_endpoints[1], selector);
                    let request = reqwest::blocking::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send();
                    match request {
                        Ok(req) => {
                            let json = req.json::<Lights>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }

    }

    /// Activate the morph animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `morph_effect` - A MorphEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_all(config.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut morph_effect = lifx::MorphEffect::new();
    ///             morph_effect.period = Some(10);
    ///             morph_effect.duration = Some(0);
    /// 
    ///             let mut palette: Vec<String> = Vec::new();
    ///             palette.push(format!("red"));
    ///             palette.push(format!("green"));
    /// 
    ///             morph_effect.palette = Some(palette);
    ///             morph_effect.power_on = Some(true);
    ///         
    ///             for light in lights {
    ///                 let results = light.morph_effect(key.clone(), morph_effect.clone());
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub fn morph_effect(&self, config: LifxConfig, morph_effect: MorphEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::morph_effect_by_selector(config, format!("id:{}", self.id), morph_effect);
    }

    /// Activate the morph animation for the selected light(s)
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
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut morph_effect = lifx::MorphEffect::new();
    ///     morph_effect.period = Some(10);
    ///     morph_effect.duration = Some(0);
    /// 
    ///     let mut palette: Vec<String> = Vec::new();
    ///     palette.push(format!("red"));
    ///     palette.push(format!("green"));
    /// 
    ///     morph_effect.palette = Some(palette);
    ///     morph_effect.power_on = Some(true);
    ///     
    ///     // Send morph effect to all lights
    ///     lifx::Light::morph_effect_by_selector(key.clone(), format!("all"), morph_effect);
    /// }
    ///  ```
    pub fn morph_effect_by_selector(config: LifxConfig, selector: String, morph_effect: MorphEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/morph", config.api_endpoints[0], selector);
        let request = reqwest::blocking::Client::new().post(url).header("Authorization", format!("Bearer {}", config.access_token)).form(&morph_effect.to_params()).send();
        match request{
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/morph", config.api_endpoints[1], selector);
                    let request = reqwest::blocking::Client::new().post(url).header("Authorization", format!("Bearer {}", config.access_token)).form(&morph_effect.to_params()).send();
                    match request{
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }



    }

    /// Activate the move animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `move_effect` - A MoveEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_all(config.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut move_effect = lifx::MoveEffect::new();
    ///             move_effect.direction = Some(format!("forward")); // or backward
    ///             move_effect.period = Some(10);
    ///             move_effect.cycles = Some(0.9);
    ///             move_effect.power_on = Some(true);
    ///         
    ///             for light in lights {
    ///                 let results = light.move_effect(key.clone(), move_effect.clone());
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub fn move_effect(&self, config: LifxConfig, move_effect: MoveEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::move_effect_by_selector(config, format!("id:{}", self.id), move_effect);
    }

    /// Activate the move animation for the selected light(s)
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
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut move_effect = lifx::MoveEffect::new();
    ///     move_effect.direction = Some(format!("forward")); // or backward
    ///     move_effect.period = Some(10);
    ///     move_effect.cycles = Some(0.9);
    ///     move_effect.power_on = Some(true);
    ///     
    ///     // Toggle all lights
    ///     lifx::Light::move_effect_by_selector(key.clone(), format!("all"), move_effect);
    /// }
    ///  ```
    pub fn move_effect_by_selector(config: LifxConfig, selector: String, move_effect: MoveEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/move", config.api_endpoints[0], selector);
        let request = reqwest::blocking::Client::new().post(url).header("Authorization", format!("Bearer {}", config.access_token)).form(&move_effect.to_params()).send();
        match request{
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/move", config.api_endpoints[1], selector);
                    let request = reqwest::blocking::Client::new().post(url).header("Authorization", format!("Bearer {}", config.access_token)).form(&move_effect.to_params()).send();
                    match request{
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }

    }

    /// Activate the pulse animation for the current light
    /// 
    /// # Arguments
    ///
    /// * `self` - A Light object.
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `pulse_effect` - A PulseEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_all(config.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut pulse = lifx::PulseEffect::new();
    ///             pulse.color = Some(format!("red"));
    ///             pulse.from_color = Some(format!("green"));
    ///             pulse.period = Some(10);
    ///             pulse.persist = Some(true);
    ///             pulse.power_on = Some(true);
    ///         
    ///             for light in lights {
    ///                 let results = light.pulse_effect(key.clone(), pulse.clone());
    ///                 println!("{:?}",results);
    ///             }
    ///         },
    ///         Err(e) => println!("{}",e)
    ///     }
    /// }
    ///  ```
    pub fn pulse_effect(&self, config: LifxConfig, pulse_effect: PulseEffect) ->  Result<LiFxResults, reqwest::Error>{
        return Self::pulse_effect_by_selector(config, format!("id:{}", self.id), pulse_effect);
    }

    /// Activate the pulse animation for the selected light(s)
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    /// * `selector` - An LIFX selector ex: all, id:xxx, group_id:xxx
    /// * `pulse_effect` - A PulseEffect object containing the values to set
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut pulse = lifx::PulseEffect::new();
    ///     pulse.color = Some(format!("red"));
    ///     pulse.from_color = Some(format!("green"));
    ///     pulse.period = Some(10);
    ///     pulse.persist = Some(true);
    ///     pulse.power_on = Some(true);
    ///     
    ///     // Toggle all lights
    ///     lifx::Light::pulse_effect_by_selector(key.clone(), format!("all"), pulse);
    /// }
    ///  ```
    pub fn pulse_effect_by_selector(config: LifxConfig, selector: String, pulse_effect: PulseEffect) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/effects/pulse", config.api_endpoints[0], selector);
        let request = reqwest::blocking::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&pulse_effect.to_params())
            .send();
        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/effects/pulse", config.api_endpoints[1], selector);
                    let request = reqwest::blocking::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&pulse_effect.to_params())
                        .send();
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }

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
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_all(config.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut state = lifx::State::new();
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
    pub fn set_state(&self, config: LifxConfig, state: State) ->  Result<LiFxResults, reqwest::Error>{
        return Self::set_state_by_selector(config, format!("id:{}", self.id), state);
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
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut off_state = lifx::State::new();
    ///     off_state.power = Some(format!("off"));
    ///     
    ///     // Turn off all lights
    ///     lifx::Light::set_state_by_selector(key.clone(), format!("all"), off_state);
    /// }
    ///  ```
    pub fn set_state_by_selector(config: LifxConfig, selector: String, state: State) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/state", config.api_endpoints[0], selector);

        let request = reqwest::blocking::Client::new().put(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&state.to_params())
            .send();
        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/state", config.api_endpoints[1], selector);

                    let request = reqwest::blocking::Client::new().put(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&state.to_params())
                        .send();
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

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
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut set_states = lifx::States::new();
    ///     let mut states: Vec<lifx::State> = Vec::new();
    ///     let mut defaults = lifx::State::new();
    ///     
    ///     defaults.brightness = Some(1.0);
    ///     
    ///     let mut state_1 = lifx::State::new();
    ///     state_1.selector = Some(format!("id:xxx"));
    ///     state_1.power = Some(format!("on"));
    ///     
    ///     let mut state_2 = lifx::State::new();
    ///     state_2.selector = Some(format!("id:xyz"));
    ///     state_2.power = Some(format!("on"));
    ///     
    ///     set_states.states = Some(states);
    ///     set_states.defaults = Some(defaults);
    ///     
    ///     lifx::Light::set_states(key.clone(), set_states);
    /// }
    ///  ```
    pub fn set_states(config: LifxConfig, states: States) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/state", config.api_endpoints[0]);

        let request = reqwest::blocking::Client::new().put(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .json(&states)
            .send();

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/state", config.api_endpoints[1]);

                    let request = reqwest::blocking::Client::new().put(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .json(&states)
                        .send();
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }
    

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
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut delta = lifx::StateDelta::new();
    ///     delta.duration = Some(0);
    ///     delta.power = Some(format!("on"));
    ///     
    ///     // Send StateDelta
    ///     lifx::Light::state_delta_by_selector(key.clone(), format!("all"), toggle);
    /// }
    ///  ```
    pub fn state_delta_by_selector(config: LifxConfig, selector: String, delta: StateDelta) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/state/delta", config.api_endpoints[0], selector);

        let request = reqwest::blocking::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&delta.to_params())
            .send();

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/state/delta", config.api_endpoints[1], selector);

                    let request = reqwest::blocking::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&delta.to_params())
                        .send();
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }

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
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let all_lights = lifx::Light::list_all(config.clone());
    ///     match all_lights {
    ///         Ok(lights) => {
    ///             println!("{:?}",lights.clone());
    ///     
    ///             let mut toggle = lifx::Toggle::new();
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
    pub fn toggle(&self, config: LifxConfig, toggle: Toggle) ->  Result<LiFxResults, reqwest::Error>{
        return Self::toggle_by_selector(config, format!("id:{}", self.id), toggle);
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
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut toggle = lifx::Toggle::new();
    ///     toggle.duration = Some(0);
    ///     
    ///     // Toggle all lights
    ///     lifx::Light::toggle_by_selector(key.clone(), format!("all"), toggle);
    /// }
    ///  ```
    pub fn toggle_by_selector(config: LifxConfig, selector: String, toggle: Toggle) ->  Result<LiFxResults, reqwest::Error>{
        let url = format!("{}/v1/lights/{}/toggle", config.api_endpoints[0], selector);

        let request = reqwest::blocking::Client::new().post(url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .form(&toggle.to_params())
            .send();

        match request {
            Ok(req) => {
                let json = req.json::<LiFxResults>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/lights/{}/toggle", config.api_endpoints[1], selector);

                    let request = reqwest::blocking::Client::new().post(url)
                        .header("Authorization", format!("Bearer {}", config.access_token))
                        .form(&toggle.to_params())
                        .send();
            
                    match request {
                        Ok(req) => {
                            let json = req.json::<LiFxResults>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                
                } else {
                    return Err(err);
                }
            }
        }
    

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
    /// Asynchronously gets ALL scenes belonging to the authenticated account
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let scenes = lifx::Scene::async_list(config).await?;
    /// }
    ///  ```
    pub async fn async_list(config: LifxConfig) -> Result<Scenes, reqwest::Error> {
        let url = format!("{}/v1/scenes", config.api_endpoints[0]);
        let request = reqwest::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send().await;
        match request {
            Ok(req) => {
                let json = req.json::<Scenes>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/scenes", config.api_endpoints[1]);
                    let request = reqwest::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send().await;
                    match request {
                        Ok(req) => {
                            let json = req.json::<Scenes>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
            
                } else {
                    return Err(err);
                }
            }
        }

    }

    /// Gets ALL scenes belonging to the authenticated account
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let scenes = lifx::Scene::list_all(config)?;
    /// }
    ///  ```
    pub fn list(config: LifxConfig) -> Result<Scenes, reqwest::Error> {
        let url = format!("{}/v1/scenes", config.api_endpoints[0]);
        let request = reqwest::blocking::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send();

        match request{
            Ok(req) => {
                let json = req.json::<Scenes>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/scenes", config.api_endpoints[1]);
                    let request = reqwest::blocking::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send();
            
                    match request{
                        Ok(req) => {
                            let json = req.json::<Scenes>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }


    }
}

/// Represents an LIFX Color
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub hue: Option<f64>,
    pub saturation: Option<f64>,
    pub kelvin: Option<i64>,
    pub brightness: Option<f64>,
    pub error: Option<String>,
    pub errors: Option<Vec<Error>>,
}
impl Color {
    /// Asynchronously validates a color
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let scenes = lifx::Color::async_validate(key, format!("red")).await?;
    /// }
    ///  ```
    pub async fn async_validate(config: LifxConfig, color: String) -> Result<Color, reqwest::Error> {
        let url = format!("{}/v1/color?string={}", config.api_endpoints[0], color);
        let request = reqwest::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send().await;
        match request {
            Ok(req) => {
                let json = req.json::<Color>().await?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/color?string={}", config.api_endpoints[1], color);
                    let request = reqwest::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send().await;
                    match request {
                        Ok(req) => {
                            let json = req.json::<Color>().await?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }

    }

    /// Validates a color
    /// 
    /// # Arguments
    ///
    /// * `access_token` - A personal acces token for authentication with LIFX.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let scenes = lifx::Color::validate(config)?;
    /// }
    ///  ```
    pub fn validate(config: LifxConfig, color: String) -> Result<Color, reqwest::Error> {
        let url = format!("{}/v1/color?string={}", config.api_endpoints[0], color);
        let request = reqwest::blocking::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send();
        match request {
            Ok(req) => {
                let json = req.json::<Color>()?;
                return Ok(json);
            },
            Err(err) => {
                if config.api_endpoints.len() > 1 {
                    let url = format!("{}/v1/color?string={}", config.api_endpoints[1], color);
                    let request = reqwest::blocking::Client::new().get(url).header("Authorization", format!("Bearer {}", config.access_token)).send();
                    match request {
                        Ok(req) => {
                            let json = req.json::<Color>()?;
                            return Ok(json);
                        },
                        Err(err2) => {
                            return Err(err2);
                        }
                    }
                } else {
                    return Err(err);
                }
            }
        }


    }
}

/// Used to set the duration/state of the HEV Clean array
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Clean {
    /// Turn the device on / off
    pub stop: Option<bool>,
    /// Duration in seconds (leaving blank or 0 sets the default duration for the device)
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
    /// The power state you want to set on the selector. on or off
    pub power: Option<String>,
    /// The color to set the light to.
    pub color: Option<String>,
    /// The brightness level from 0.0 to 1.0. Overrides any brightness set in color (if any).
    pub brightness: Option<f64>,
    /// How long in seconds you want the power action to take. Range: 0.0 – 3155760000.0 (100 years)
    pub duration: Option<f64>,
    /// The maximum brightness of the infrared channel from 0.0 to 1.0.
    pub infrared: Option<f64>,
    /// The selector to limit which light to use for set_states()
    pub selector:  Option<String>,
    /// Execute the query fast, without initial state checks and wait for no results.
    pub fast: Option<bool>
}
impl State {

    /// Returns a new State object
    /// 
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut state = lifx::State::new();
    ///     state.power = Some(format!("off"));
    /// }
    ///  ```
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

/// Used to set the params when posting a Toggle event
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toggle {
    pub duration: Option<i64>
}
impl Toggle {
    /// Returns a new Toggle object
    /// 
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut toggle = lifx::Toggle::new();
    ///     toggle.duration = Some(0);
    /// }
    ///  ```
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
    /// Returns a new States object
    /// 
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut states = lifx::States::new();
    /// }
    ///  ```
    pub fn new() -> Self {
        return States{
            states: None,
            defaults: None
        };
    }
}

/// Used to set the params when posting a StateDelta event
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
    /// Returns a new StateDelta object
    /// 
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut delta = lifx::StateDelta::new();
    ///     delta.duration = Some(0);
    /// }
    ///  ```
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

/// Used to set the params when posting a BreatheEffect event
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
    /// Returns a new BreatheEffect object
    /// 
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut breathe = lifx::BreatheEffect::new();
    ///     breathe.color = Some(format!("red"));
    ///     breathe.from_color = Some(format!("green"));
    ///     breathe.period = Some(10);
    ///     breathe.persist = Some(true);
    ///     breathe.power_on = Some(true);
    /// }
    ///  ```
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

/// Used to set the params when posting a MoveEffect event
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveEffect {
    /// The color to use for the breathe effect.
    pub direction: Option<String>,
    /// The time in seconds for one cycle of the effect.
    pub period: Option<i64>,
    /// The number of times to repeat the effect.
    pub cycles: Option<f64>,
    /// If true, turn the bulb on if it is not already on.
    pub power_on: Option<bool>,
    /// Execute the query fast, without initial state checks and wait for no results.
    pub fast: Option<bool>,
}
impl MoveEffect {
    /// Returns a new MoveEffect object
    /// 
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut move_effect = lifx::MoveEffect::new();
    ///     move_effect.direction = Some(format!("forward")); // or backward
    ///     move_effect.period = Some(10);
    ///     move_effect.cycles = Some(0.9);
    ///     move_effect.power_on = Some(true);
    /// }
    ///  ```
    pub fn new() -> Self {
        return MoveEffect{
            direction: None,
            period: None,
            cycles: None,
            power_on: None,
            fast: None
        };
    }

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];
        match &self.direction{
            Some(direction) => params.push(("direction".to_string(), direction.to_string())),
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

        match &self.power_on{
            Some(power_on) => params.push(("power_on".to_string(), power_on.to_string())),
            None => {}
        }

        match &self.fast{
            Some(fast) => params.push(("fast".to_string(), fast.to_string())),
            None => {}
        }

        return params;
    }

}

/// Used to set the params when posting a MorphEffect event
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MorphEffect {
    /// The time in seconds for one cycle of the effect.
    pub period: Option<i64>,
    /// How long the animation lasts for in seconds. Not specifying a duration makes the animation never stop. Specifying 0 makes the animation stop. Note that there is a known bug where the tile remains in the animation once it has completed if duration is nonzero.
    pub duration: Option<f64>,
    /// You can control the colors in the animation by specifying a list of color specifiers. For example ["red", "hue:100 saturation:1"]. See https://api.developer.lifx.com/docs/colors
    pub palette: Option<Vec<String>>,
    /// If true, turn the bulb on if it is not already on.
    pub power_on: Option<bool>,
    /// Execute the query fast, without initial state checks and wait for no results.
    pub fast: Option<bool>,
}
impl MorphEffect {
    /// Returns a new MorphEffect object
    /// 
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut morph_effect = lifx::MorphEffect::new();
    ///     morph_effect.period = Some(10);
    ///     morph_effect.duration = Some(0);
    /// 
    ///     let mut palette: Vec<String> = Vec::new();
    ///     palette.push("red");
    ///     palette.push("green");
    /// 
    ///     morph_effect.palette = Some(palette);
    ///     morph_effect.power_on = Some(true);
    /// 
    /// }
    ///  ```
    pub fn new() -> Self {
        return MorphEffect{
            period: None,
            duration: None,
            palette: None,
            power_on: None,
            fast: None
        };
    }

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];
        match &self.period{
            Some(period) => params.push(("period".to_string(), period.to_string())),
            None => {}
        }

        match &self.duration{
            Some(duration) => params.push(("duration".to_string(), duration.to_string())),
            None => {}
        }

        match &self.palette{
            Some(palette) => params.push(("palette".to_string(), string_vec_to_params(palette.to_vec()))),
            None => {}
        }

        match &self.power_on{
            Some(power_on) => params.push(("power_on".to_string(), power_on.to_string())),
            None => {}
        }

        match &self.fast{
            Some(fast) => params.push(("fast".to_string(), fast.to_string())),
            None => {}
        }

        return params;
    }

}



/// Used to set the params when posting a PulseEffect event
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PulseEffect {
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
}
impl PulseEffect {
    /// Returns a new PulseEffect object
    /// 
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut pulse = lifx::PulseEffect::new();
    ///     pulse.color = Some(format!("red"));
    ///     pulse.from_color = Some(format!("green"));
    ///     pulse.period = Some(10);
    ///     pulse.persist = Some(true);
    ///     pulse.power_on = Some(true);
    /// }
    ///  ```
    pub fn new() -> Self {
        return PulseEffect{
            color: None,
            from_color: None,
            period: None,
            cycles: None,
            persist: None,
            power_on: None
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

        return params;
    }

}

/// Used to set the params when posting a EffectsOff event
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectsOff {
    /// If true, the devices will also be turned off
    pub power_off: Option<bool>,
}
impl EffectsOff {
    /// Returns a new EffectsOff object
    /// 
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut ef = lifx::EffectsOff::new();
    ///     ef.power_off = Some(true);
    /// }
    ///  ```
    pub fn new() -> Self {
        return EffectsOff{
            power_off: None,
        };
    }

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];
        match &self.power_off{
            Some(power_off) => params.push(("power_off".to_string(), power_off.to_string())),
            None => {}
        }

        return params;
    }

}



/// Used to set the params when posting a FlameEffect event
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlameEffect {
    /// The time in seconds for one cycle of the effect.
    pub period: Option<i64>,
    /// How long the animation lasts for in seconds. Not specifying a duration makes the animation never stop. Specifying 0 makes the animation stop. Note that there is a known bug where the tile remains in the animation once it has completed if duration is nonzero.
    pub duration: Option<f64>,
    /// If true, turn the bulb on if it is not already on.
    pub power_on: Option<bool>,
    /// Execute the query fast, without initial state checks and wait for no results.
    pub fast: Option<bool>,
}
impl FlameEffect {
    /// Returns a new FlameEffect object
    /// 
    /// # Examples
    ///
    /// ```
    /// extern crate lifx_rs as lifx;
    /// 
    /// fn main() {
    /// 
    ///     let key = "xxx".to_string();
    ///     let mut api_endpoints: Vec<String> = Vec::new();
    ///
    ///     api_endpoints.push(format!("https://api.lifx.com"));
    ///     api_endpoints.push(format!("http://localhost:8089"));
    ///
    ///     let config = lifx::LifxConfig{
    ///        access_token: key.clone(),
    ///        api_endpoints: api_endpoints
    ///     };
    /// 
    ///     let mut flame_effect = lifx::FlameEffect::new();
    ///     flame_effect.period = Some(10);
    ///     flame_effect.duration = Some(0);
    ///     flame_effect.power_on = Some(true);
    /// 
    /// }
    ///  ```
    pub fn new() -> Self {
        return FlameEffect{
            period: None,
            duration: None,
            power_on: None,
            fast: None
        };
    }

    fn to_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = vec![];
        match &self.period{
            Some(period) => params.push(("period".to_string(), period.to_string())),
            None => {}
        }

        match &self.duration{
            Some(duration) => params.push(("duration".to_string(), duration.to_string())),
            None => {}
        }

        match &self.power_on{
            Some(power_on) => params.push(("power_on".to_string(), power_on.to_string())),
            None => {}
        }

        match &self.fast{
            Some(fast) => params.push(("fast".to_string(), fast.to_string())),
            None => {}
        }

        return params;
    }

}

pub fn string_vec_to_params(input: Vec<String>) -> String {

    let mut params = String::new();
    let count = 0;
    for iput in input {
        if count == 0 {
            params = format!("[\"{}\"", iput);
        } else {
            params = format!("{}, \"{}\"",params, iput);
        }
    }

    params = format!("{}]", params);

    return params;
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
