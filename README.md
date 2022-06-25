# lifx-rs

## Description

A synchronous + asynchronous library for communicating with the official LIFX-API and the unoffical offline API. 

## LIFX-API Supported Methods:
* List Lights
* Set State
* Set States
* State Delta
* Toggle Power
* Breathe Effect
* Move Effect
* Morph Effect
* Flame Effect
* Pulse Effect
* Effects Off
* Clean (HEV)
* List Scenes
* Validate Color

## Unofficial Offline API Supported Methods:
* List Lights
* Set State
* Set States

## To use offline use the Un-Official API Server:
[lifx-api-server](https://crates.io/crates/lifx-api-server)

## How to use library

Add the following line to your cargo.toml:
```
lifx-rs = "0.1.27"
```

Example:
```rust
extern crate lifx_rs as lifx;

fn main() {

    let key = "xxx".to_string();
    
    let mut api_endpoints: Vec<String> = Vec::new();
    
    // Official API
    api_endpoints.push(format!("https://api.lifx.com"));

    // lifx-server-api (Un-Official)
    api_endpoints.push(format!("http://localhost:8089"));

    let config = lifx::LifxConfig{
        access_token: key.clone(),
        api_endpoints: api_endpoints
    };

    // Build an "OffState" to set
    let mut off_state = lifx::State::new();
    off_state.power = Some(format!("off"));

    // Turn off all lights
    lifx::Light::set_state_by_selector(config.clone(), format!("all"), off_state);


    let all_lights = lifx::Light::list_all(config.clone());
    match all_lights {
        Ok(lights) => {
            println!("{:?}",lights.clone());

            let mut state = lifx::State::new();
            state.power = Some(format!("on"));
            state.brightness = Some(1.0);
        
            for light in lights {
                let results = light.set_state(config.clone(), state.clone());
                println!("{:?}",results);
            }
        },
        Err(e) => println!("{}",e)
    }

}

```


Async Example:
```rust
extern crate lifx_rs as lifx;

#[tokio::main]
async fn main() {

    let key = "xxx".to_string();
    
    let mut api_endpoints: Vec<String> = Vec::new();
    
    // Official API
    api_endpoints.push(format!("https://api.lifx.com"));

    // lifx-server-api (Un-Official)
    api_endpoints.push(format!("http://localhost:8089"));

    let config = lifx::LifxConfig{
        access_token: key.clone(),
        api_endpoints: api_endpoints
    };

    // Build "OffState" to set
    let mut off_state = lifx::State::new();
    off_state.power = Some(format!("off"));
    
    // Turn off all lights
    lifx::Light::async_set_state_by_selector(config.clone(), format!("all"), off_state).await;
}
```


## License

Released under Apache 2.0 or MIT.

# Support and follow my work by:

#### Buying my dope NTFs:
 * https://opensea.io/accounts/PixelCoda

#### Checking out my Github:
 * https://github.com/PixelCoda

#### Following my facebook page:
 * https://www.facebook.com/pixelcoda/

#### Subscribing to my Patreon:
 * https://www.patreon.com/calebsmith_pixelcoda

#### Or donating crypto:
 * ADA: addr1qyp299a45tgvveh83tcxlf7ds3yaeh969yt3v882lvxfkkv4e0f46qvr4wzj8ty5c05jyffzq8a9pfwz9dl6m0raac7s4rac48
 * ALGO: VQ5EK4GA3IUTGSPNGV64UANBUVFAIVBXVL5UUCNZSDH544XIMF7BAHEDM4
 * ATOM: cosmos1wm7lummcealk0fxn3x9tm8hg7xsyuz06ul5fw9
 * BTC: bc1qh5p3rff4vxnv23vg0hw8pf3gmz3qgc029cekxz
 * ETH: 0x7A66beaebF7D0d17598d37525e63f524CfD23452
 * ERC20: 0x7A66beaebF7D0d17598d37525e63f524CfD23452
 * XLM: GCJAUMCO2L7PTYMXELQ6GHBTF25MCQKEBNSND2C4QMUPTSVCPEN3LCOG
 * XTZ: tz1SgJppPn56whprsDDGcqR4fxqCr2PXvg1R