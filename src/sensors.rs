extern crate rand;

use rand::distributions::uniform::Uniform;
use rand::prelude::Distribution;

use rand::SeedableRng;
use rand::rngs::SmallRng;

use std::intrinsics::unchecked_add;
use core::arch::x86_64::_rdrand16_step;
pub static mut SEED: u64 = 0;

use apollo::parameters::*;


#[cold]
pub unsafe fn init() {
    let mut _seed: u16 = 0;
    _rdrand16_step(&mut _seed);
    SEED = _seed as u64;
}

#[inline]
fn make_rng() -> SmallRng {
    unsafe{
        SEED = unchecked_add(SEED, 1);
        SmallRng::seed_from_u64(SEED)}
}


pub fn get_location() -> ([u8; LATITUDE_SIZE], [u8; LONGITUDE_SIZE]) {
    
    // TODO: Replace with actual sensor code.
    let mut rng = make_rng();
    // Create a uniform distribution between -90 and 90 degrees.
    let latitude_range: Uniform<f32> = Uniform::new(-90.0, 90.0);
    
    // Create a uniform distribution between -180 and 180 degrees.
    let longitude_range: Uniform<f32> = Uniform::new(-180.0, 180.0);

    // Generate a random latitude and longitude.
    let latitude: f32 = latitude_range.sample(&mut rng);
    let longitude: f32 = longitude_range.sample(&mut rng);

    let latitude_bytes = latitude.to_be_bytes();
    let longitude_bytes = longitude.to_be_bytes();
    return (latitude_bytes, longitude_bytes);
}

pub fn get_altitude() -> [u8; ALTITUDE_SIZE] {
    // Returns the altitude of the balloon, in meters.
    // TODO: Replace with actual sensor code.

    // Generate a random altitude between 0 and 10,000 meters.
    let mut rng = make_rng();
    let altitude_range = Uniform::new(0.0, 10000.0);
    let altitude: f32 = altitude_range.sample(&mut rng);
    return altitude.to_be_bytes();
}

pub fn get_voltage() -> [u8; VOLTAGE_SIZE] {
    // Returns the voltage difference from nomal voltage, in volts.
    // TODO: Replace with actual sensor code.

    // Generate a random voltage difference between -0.1 and 1.5 volts.
    let mut rng = make_rng();
    let voltage_range: Uniform<f32> = Uniform::new(-0.1, 1.5);
    let voltage = voltage_range.sample(&mut rng);
    return voltage.to_be_bytes();
}

pub fn get_temperature() -> [u8; TEMPERATURE_SIZE] {
    // Returns the temperature of the balloon, in degrees Celsius.
    // TODO: Replace with actual sensor code.

    // Generate a random temperature between -50 and 50 degrees Celsius.
    let mut rng = make_rng();
    let temperature_range = Uniform::new(-50.0, 50.0);
    let temperature: f32 = temperature_range.sample(&mut rng);
    return temperature.to_be_bytes();
}