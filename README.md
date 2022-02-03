# lifx-rs

## Description

A synchronous + asynchronous library for communicating with the LIFX-API. 

## Supported API Methods:
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

## How to use library

Add the following line to your cargo.toml:
```
lifx-rs = "0.1.2"
```

Example:
```rust
extern crate lifx_rs as lifx;

fn main() {

    let key = "xxx".to_string();
    
    let mut off_state = lifx::State::new();
    off_state.power = Some(format!("off"));

    // Turn off all lights
    lifx::Light::set_state_by_selector(key.clone(), format!("all"), off_state);


    let all_lights = lifx::Light::list_all(key.clone());
    match all_lights {
        Ok(lights) => {
            println!("{:?}",lights.clone());

            let mut state = lifx::State::new();
            state.power = Some(format!("on"));
            state.brightness = Some(1.0);
        
            for light in lights {
                let results = light.set_state(key.clone(), state.clone());
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

    let mut off_state = lifx::State::new();
    off_state.power = Some(format!("off"));
    
    // Turn off all lights
    lifx::Light::set_state_by_selector(key.clone(), format!("all"), off_state);
}
```




## License

Released under Apache 2.0.

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
 * ADA:    addr1vyjsx8zthl5fks8xjsf6fkrqqsxr4f5tprfwux5zsnz862glwmyr3
 * BTC:    3BCj9kYsqyENKU5YgrtHgdQh5iA7zxeJJi
 * MANA:   0x10DFc66F881226f2B91D552e0Cf7231C1e409114
 * SHIB:   0xdE897d5b511A66276E9B91A8040F2592553e6c28


