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

use serde_json::json;

use serde::{Serialize, Deserialize};
use std::convert::TryInto;

pub type Lights = Vec<Light>;

/// Represents your Stripe balance.
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





}

pub type Scenes = Vec<Scene>;

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


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
pub struct Account {
    pub uuid: String,
}


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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub field: String,
    pub message: Vec<String>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiFxResults {
    pub results: Option<Vec<LiFxResult>>,
    pub error: Option<String>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiFxResult {
    pub id: String,
    pub label: String,
    pub status: String,
}
